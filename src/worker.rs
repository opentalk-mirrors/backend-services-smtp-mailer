use crate::{mail::MailBuilder, settings};
use anyhow::Result;
use futures::stream::StreamExt;
use lapin::options::BasicRejectOptions;
use lettre::message::{header, MultiPart, SinglePart};
use mail_worker_proto as proto;

pub struct Worker<T> {
    mail_backend: T,
    mail_builder: MailBuilder,
}

impl<T> Worker<T>
where
    T: lettre::AsyncTransport + Sync,
    anyhow::Error: From<T::Error>,
{
    /// Creates a new Worker instance
    ///
    pub fn new(mail_backend: T, settings: &settings::Settings) -> Result<Self> {
        let mail_builder = MailBuilder::new(settings)?;

        Ok(Self {
            mail_backend,
            mail_builder,
        })
    }

    pub async fn start(&self, settings: &settings::RabbitMqConfig) -> Result<()> {
        let mut rabbitmq = crate::rabbitmq::RabbitMqService::new(settings).await?;

        log::info!("Worker started");
        // TODO refactor this in a way, that we here have a generic stream.
        // Maybe try the Pipeline Server Pattern from tokio-tower
        while let Some(delivery) = rabbitmq.consumer.next().await {
            log::trace!("Recevied new mail task");

            let delivery = delivery?;
            let data = &delivery.data;

            // TODO add text_map_propagator::TextMapPropagator based tracing extraction here.

            match dbg!(self.handler(data).await) {
                Result::Ok(_) => {
                    if let Err(e) = delivery.ack(Default::default()).await {
                        log::error!("Ack Error{}", e);
                    }
                }
                Result::Err(e) => {
                    if let Err(e) = delivery.reject(BasicRejectOptions { requeue: true }).await {
                        log::error!("Ack Error{}", e);
                    }
                    log::error!("Handler Error: {}", e);
                    continue;
                }
            }
        }
        Ok(())
    }

    async fn handler(&self, data: &[u8]) -> Result<()> {
        let de = &mut serde_json::Deserializer::from_slice(&data);
        let versioned_message: proto::MailTask = serde_path_to_error::deserialize(de)?;

        match versioned_message {
            proto::MailTask::V1(message) => {
                send_mail_v1(&self.mail_backend, &self.mail_builder, &message).await?
            }
        }
        Ok(())
    }
}

async fn send_mail_v1<T>(
    mail_backend: &T,
    mail_builder: &MailBuilder,
    message: &proto::v1::Message,
) -> Result<()>
where
    T: lettre::AsyncTransport + Sync,
    anyhow::Error: From<T::Error>,
{
    let email = mail_builder.generate_email(message)?;

    let txt = mail_builder.generate_email_body(message)?;
    let html = mail_builder.generate_email_html(message)?;
    let email = email.multipart(
        MultiPart::alternative()
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_PLAIN)
                    .body(txt),
            )
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(html),
            ),
    )?;

    mail_backend.send(email).await?;

    Ok(())
}
