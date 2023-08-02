// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{
    mail::{MailBuilder, MailTemplate},
    settings,
};
use anyhow::Result;
use futures::stream::StreamExt;
use lapin::options::BasicRejectOptions;
use mail_worker_protocol as proto;

/// A Mail Worker
pub struct Worker<T> {
    mail_backend: T,
    mail_builder: MailBuilder,
}

impl<T> Worker<T>
where
    T: lettre::AsyncTransport<Error = lettre::transport::smtp::Error> + Sync,
{
    /// Creates a new Worker instance
    pub fn new(mail_backend: T, settings: &settings::Settings) -> Result<Self> {
        let mail_builder = MailBuilder::new(settings)?;

        Ok(Self {
            mail_backend,
            mail_builder,
        })
    }

    /// Starts the worker loop
    ///
    /// Yields when the rabbitMQ queue yields None
    pub async fn start(&self, settings: &settings::RabbitMqConfig) -> Result<()> {
        let mut rabbitmq = crate::rabbitmq::RabbitMqService::new(settings).await?;

        log::info!("Worker started");
        // TODO refactor this in a way, that we here have a generic stream.
        // Maybe try the Pipeline Server Pattern from tokio-tower
        while let Some(delivery) = rabbitmq.consumer.next().await {
            log::trace!("Received new mail task");

            let delivery = delivery?;
            let data = &delivery.data;

            // TODO add text_map_propagator::TextMapPropagator based tracing extraction here.

            match self.handler(data).await {
                Result::Ok(_) => {
                    if let Err(e) = delivery.ack(Default::default()).await {
                        log::error!("Ack Error: {}", e);
                    }
                }
                Result::Err(e) => {
                    // check if the error is a smtp error and a requeue/exit is applicable
                    let (requeue, exit) = if let Some(smtp_error) =
                        e.downcast_ref::<lettre::transport::smtp::Error>()
                    {
                        if smtp_error.is_timeout() {
                            (true, true)
                        } else {
                            (
                                !(smtp_error.is_permanent() || smtp_error.is_client()),
                                false,
                            )
                        }
                    } else {
                        (false, false)
                    };

                    if let Err(e) = delivery.reject(BasicRejectOptions { requeue }).await {
                        log::error!("Ack Error: {}", e);
                    }

                    log::debug!("Handler Error: {}", e);

                    if exit {
                        return Err(e);
                    }

                    continue;
                }
            }
        }
        Ok(())
    }

    async fn handler(&self, data: &[u8]) -> Result<()> {
        let de = &mut serde_json::Deserializer::from_slice(data);
        let versioned_message: proto::MailTask = serde_path_to_error::deserialize(de)?;

        match versioned_message {
            proto::MailTask::V1(message) => {
                send_mail_v1(&self.mail_backend, &self.mail_builder, &message).await?
            }
        }
        Ok(())
    }
}

pub async fn send_mail_v1<T>(
    mail_backend: &T,
    mail_builder: &MailBuilder,
    message: &proto::v1::Message,
) -> Result<()>
where
    T: lettre::AsyncTransport<Error = lettre::transport::smtp::Error> + Sync,
{
    let email = mail_builder.generate_email(message)?;
    // Hack until lettre supports printing the To header
    let to = message.generate_to_mbox(mail_builder);
    mail_backend.send(email).await?;

    log::info!(
        "Send mail to {}",
        to.map(|mailbox| mailbox.to_string())
            .unwrap_or_else(|_| "N/A".to_string())
    );

    Ok(())
}
