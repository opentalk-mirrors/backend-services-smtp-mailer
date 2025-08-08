// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use anyhow::{Context, Result};
use tokio_executor_trait::Tokio as TokioExecutor;
use tokio_reactor_trait::Tokio as TokioReactor;

use crate::settings;

pub(crate) struct RabbitMqService {
    _conn: lapin::Connection,
    _channel: lapin::Channel,
    pub consumer: lapin::Consumer,
}

impl RabbitMqService {
    pub async fn new(settings: &settings::RabbitMqConfig) -> Result<Self> {
        let conn = lapin::Connection::connect(
            &settings.url,
            lapin::ConnectionProperties::default()
                .with_executor(TokioExecutor::current())
                .with_reactor(TokioReactor),
        )
        .await
        .context("lapin connect")?;

        let channel = conn.create_channel().await?;
        let queue = channel
            .queue_declare(&settings.queue_name, Default::default(), Default::default())
            .await?;
        let consumer = channel
            .basic_consume(
                queue.name().as_str(),
                "opentalk_mailer",
                Default::default(),
                Default::default(),
            )
            .await?;

        Ok(RabbitMqService {
            _conn: conn,
            _channel: channel,
            consumer,
        })
    }
}
