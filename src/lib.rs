// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use anyhow::{Context, Result};
use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};
use service_probe::{start_probe, ServiceState};
use settings::MonitoringSettings;

use crate::worker::Worker;

pub mod i18n;
pub(crate) mod mail;

mod ics;
pub mod preview;
mod rabbitmq;
pub mod settings;
mod worker;

pub use mail::{MailBuilder, MailTemplate};
pub use worker::send_mail_v1;

/// Entry point of the library part of smtp-mailer
pub async fn run(settings: settings::Settings) -> Result<()> {
    if let Some(MonitoringSettings { port, addr }) = settings.monitoring {
        start_probe(addr, port, ServiceState::Up).await?;
    }

    let smtp_client: AsyncSmtpTransport<Tokio1Executor> =
        settings.smtp.smtp_server.clone().try_into()?;

    let worker = Worker::new(smtp_client, &settings)?
        .start(&settings.rabbit_mq)
        .await;

    worker.expect("Error initializing worker");

    Ok(())
}

impl TryFrom<settings::SmtpUri> for AsyncSmtpTransport<Tokio1Executor> {
    type Error = anyhow::Error;
    fn try_from(val: settings::SmtpUri) -> Result<AsyncSmtpTransport<Tokio1Executor>> {
        let host = val.host().context("SMTP Host")?;

        let mut builder = match val.scheme() {
            "smtp" => {
                if val.disable_starttls() {
                    AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(host)
                } else {
                    AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(host)?
                }
            }
            "smtps" => AsyncSmtpTransport::<Tokio1Executor>::relay(host)?,
            _ => unreachable!(),
        };

        match val.credentials()? {
            (Some(u), None) => {
                let creds = Credentials::new(u, "".to_owned());
                builder = builder.credentials(creds);
            }
            (Some(u), Some(p)) => {
                let creds = Credentials::new(u, p);
                builder = builder.credentials(creds);
            }
            _ => {}
        }

        if let Some(port) = val.port() {
            builder = builder.port(port);
        }
        Ok(builder.build())
    }
}
