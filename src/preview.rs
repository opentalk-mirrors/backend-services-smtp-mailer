// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::time::Duration;

use chrono::{Timelike, Utc};
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use mail_worker_protocol::{
    self as protocol,
    v1::{
        ExternalEventCancellation, ExternalEventInvite, RegisteredEventCancellation,
        RegisteredEventInvite, RegisteredEventUninvite, RegisteredEventUpdate,
        UnregisteredEventCancellation, UnregisteredEventInvite,
    },
};
use protocol::v1::{CallIn, Event, Room, Time};
use smtp_mailer::{send_mail_v1, settings, MailBuilder, MailTemplate};
use types::common::shared_folder::{SharedFolder, SharedFolderAccess};
use uuid::Uuid;

const EVENT_DESCRIPTION_REGISTERED: &str = "This event is a dummy event, and you should have got this invite as a registered user. You can safely ignore this description";
const EVENT_DESCRIPTION_EXTERNAL: &str = "This event is a dummy event, and you should have got this invite as a external user. You can safely ignore this description";
const EVENT_DESCRIPTION_UNREGISTERED: &str = "This event is a dummy event, and you should have got this invite as a keycloak user. You can safely ignore this description";

fn build_shared_folder() -> Option<SharedFolder> {
    Some(SharedFolder {
        read: SharedFolderAccess {
            url: "https://www.example.org/read".to_owned(),
            password: "ReadSecret".to_owned(),
        },
        read_write: Some(SharedFolderAccess {
            url: "https://www.example.org/write".to_owned(),
            password: "WriteSecret".to_owned(),
        }),
    })
}

fn unregistered_invitee(email_address: &Option<String>) -> protocol::v1::UnregisteredUser {
    protocol::v1::UnregisteredUser {
        email: email_address
            .as_deref()
            .unwrap_or("invitee@example.org")
            .into(),
        first_name: "Ingrid".to_string(),
        last_name: "Invitee".to_string(),
    }
}

fn external_invitee(email_address: &Option<String>) -> protocol::v1::ExternalUser {
    protocol::v1::ExternalUser {
        email: email_address
            .as_deref()
            .unwrap_or("external.invitee@example.org")
            .into(),
    }
}

fn registered_invitee(
    language: &str,
    email_address: &Option<String>,
) -> protocol::v1::RegisteredUser {
    protocol::v1::RegisteredUser {
        email: email_address
            .as_deref()
            .unwrap_or("invitee@example.org")
            .into(),
        first_name: "Ingrid".to_string(),
        last_name: "Invitee".to_string(),
        title: "Prof.".to_string(),
        language: language.to_string(),
    }
}

fn registered_inviter(
    language: &str,
    email_address: &Option<String>,
) -> protocol::v1::RegisteredUser {
    protocol::v1::RegisteredUser {
        email: email_address
            .as_deref()
            .unwrap_or("ernest.inviter@example.org")
            .into(),
        first_name: "Ernest".to_string(),
        last_name: "Inviter".to_string(),
        title: "Dr.".to_string(),
        language: language.to_string(),
    }
}

pub trait ExampleData: Sized + MailTemplate {
    fn generate_example(language: &str, email_address: &Option<String>) -> Self;

    fn preview(settings: &settings::Settings, html: bool, lang: &str) -> String {
        let mail_builder = MailBuilder::new(settings).unwrap();

        let demo_message = Self::generate_example(lang, &None);

        if html {
            return demo_message
                .generate_email_html(&mail_builder)
                .unwrap_or_else(|e| {
                    log::error!("Error while generating html: {e:?}");
                    "".into()
                });
        }

        demo_message
            .generate_email_plain(&mail_builder)
            .unwrap_or_else(|e| {
                log::error!("Error while generating plain text: {e:?}");
                "".into()
            })
    }
}

fn generate_example_event(description: String) -> Event {
    let next_hour: chrono::DateTime<chrono::Utc> = (chrono::Utc::now()
        + chrono::Duration::hours(1))
    .with_minute(0)
    .unwrap()
    .with_second(0)
    .unwrap();

    Event {
        id: Uuid::new_v4(),
        name: "This is a Preview Event".into(),
        description,
        room: Room {
            id: Uuid::nil(),
            password: Some("password123".into()),
        },
        created_at: Time {
            time: Utc::now(),
            timezone: "Europe/Berlin".into(),
        },
        start_time: Some(Time {
            time: next_hour,
            timezone: "Europe/Berlin".into(),
        }),
        end_time: Some(Time {
            time: next_hour + chrono::Duration::minutes(40),
            timezone: "Europe/Berlin".into(),
        }),
        rrule: None,
        call_in: Some(CallIn {
            sip_tel: "+4930405051330".into(),
            sip_id: "1234567890".into(),
            sip_password: "1234567890".into(),
        }),
        revision: 0,
        shared_folder: build_shared_folder(),
        adhoc_retention_seconds: None,
    }
}

impl ExampleData for protocol::v1::UnregisteredEventInvite {
    fn generate_example(language: &str, email_address: &Option<String>) -> Self {
        Self {
            invitee: unregistered_invitee(email_address),
            event: generate_example_event(EVENT_DESCRIPTION_UNREGISTERED.to_string()),
            inviter: registered_inviter(language, &None),
        }
    }
}

impl ExampleData for protocol::v1::UnregisteredEventCancellation {
    fn generate_example(language: &str, email_address: &Option<String>) -> Self {
        Self {
            invitee: unregistered_invitee(email_address),
            event: generate_example_event(EVENT_DESCRIPTION_UNREGISTERED.to_string()),
            inviter: registered_inviter(language, &None),
        }
    }
}

impl ExampleData for protocol::v1::RegisteredEventInvite {
    fn generate_example(language: &str, email_address: &Option<String>) -> Self {
        Self {
            invitee: registered_invitee(language, email_address),
            event: generate_example_event(EVENT_DESCRIPTION_REGISTERED.to_string()),
            inviter: registered_inviter(language, email_address),
        }
    }
}

impl ExampleData for protocol::v1::RegisteredEventUninvite {
    fn generate_example(language: &str, email_address: &Option<String>) -> Self {
        Self {
            invitee: registered_invitee(language, email_address),
            event: generate_example_event(EVENT_DESCRIPTION_REGISTERED.to_string()),
            inviter: registered_inviter(language, email_address),
        }
    }
}

impl ExampleData for protocol::v1::RegisteredEventUpdate {
    fn generate_example(language: &str, email_address: &Option<String>) -> Self {
        Self {
            invitee: registered_invitee(language, email_address),
            event: generate_example_event(EVENT_DESCRIPTION_REGISTERED.to_string()),
            inviter: registered_inviter(language, email_address),
            event_exception: None,
        }
    }
}

impl ExampleData for protocol::v1::RegisteredEventCancellation {
    fn generate_example(language: &str, email_address: &Option<String>) -> Self {
        Self {
            invitee: registered_invitee(language, email_address),
            event: generate_example_event(EVENT_DESCRIPTION_REGISTERED.to_string()),
            inviter: registered_inviter(language, &None),
        }
    }
}

impl ExampleData for protocol::v1::ExternalEventInvite {
    fn generate_example(language: &str, email_address: &Option<String>) -> Self {
        Self {
            invitee: external_invitee(email_address),
            event: generate_example_event(EVENT_DESCRIPTION_EXTERNAL.to_string()),
            inviter: registered_inviter(language, email_address),
            invite_code: Uuid::new_v4().to_string(),
        }
    }
}

impl ExampleData for protocol::v1::ExternalEventCancellation {
    fn generate_example(language: &str, email_address: &Option<String>) -> Self {
        Self {
            invitee: external_invitee(email_address),
            event: generate_example_event(EVENT_DESCRIPTION_EXTERNAL.to_string()),
            inviter: registered_inviter(language, &None),
        }
    }
}

pub async fn preview_send_mail(
    settings: &settings::Settings,
    template: super::TemplateVariant,
    to: String,
    cancellation_delay: u64,
) {
    let smtp_client: AsyncSmtpTransport<Tokio1Executor> =
        settings.smtp.smtp_server.clone().try_into().unwrap();
    let mail_builder = MailBuilder::new(settings).unwrap();

    let duration_between_invite_and_cancellation = Duration::from_secs(cancellation_delay);
    let receiver = Some(to);

    let message = match template {
        crate::TemplateVariant::RegisteredInvite => protocol::v1::Message::RegisteredEventInvite(
            RegisteredEventInvite::generate_example("en-US", &receiver),
        ),
        crate::TemplateVariant::UnregisteredInvite => {
            protocol::v1::Message::UnregisteredEventInvite(
                UnregisteredEventInvite::generate_example("en-US", &receiver),
            )
        }
        crate::TemplateVariant::ExternalInvite => protocol::v1::Message::ExternalEventInvite(
            ExternalEventInvite::generate_example("en-US", &receiver),
        ),
        crate::TemplateVariant::RegisteredCancellation => {
            let invite = RegisteredEventInvite::generate_example("en-US", &receiver);
            let event_id = invite.event.id;

            send_mail_v1(
                &smtp_client,
                &mail_builder,
                &protocol::v1::Message::RegisteredEventInvite(invite),
            )
            .await
            .unwrap();

            tokio::time::sleep(duration_between_invite_and_cancellation).await;

            let mut cancellation =
                RegisteredEventCancellation::generate_example("en-US", &receiver);
            cancellation.event.id = event_id;
            protocol::v1::Message::RegisteredEventCancellation(cancellation)
        }
        crate::TemplateVariant::UnregisteredCancellation => {
            let invite = UnregisteredEventInvite::generate_example("en-US", &receiver);
            let event_id = invite.event.id;

            send_mail_v1(
                &smtp_client,
                &mail_builder,
                &protocol::v1::Message::UnregisteredEventInvite(invite),
            )
            .await
            .unwrap();

            tokio::time::sleep(duration_between_invite_and_cancellation).await;

            let mut cancellation =
                UnregisteredEventCancellation::generate_example("en-US", &receiver);
            cancellation.event.id = event_id;
            protocol::v1::Message::UnregisteredEventCancellation(cancellation)
        }
        crate::TemplateVariant::ExternalCancellation => {
            let invite = ExternalEventInvite::generate_example("en-US", &receiver);
            let event_id = invite.event.id;

            send_mail_v1(
                &smtp_client,
                &mail_builder,
                &protocol::v1::Message::ExternalEventInvite(invite),
            )
            .await
            .unwrap();

            tokio::time::sleep(duration_between_invite_and_cancellation).await;

            let mut cancellation = ExternalEventCancellation::generate_example("en-US", &receiver);
            cancellation.event.id = event_id;
            protocol::v1::Message::ExternalEventCancellation(cancellation)
        }
        crate::TemplateVariant::RegisteredEventUpdate => {
            protocol::v1::Message::RegisteredEventUpdate(RegisteredEventUpdate::generate_example(
                "en-US", &receiver,
            ))
        }
        crate::TemplateVariant::RegisteredUninvite => {
            protocol::v1::Message::RegisteredEventUninvite(
                RegisteredEventUninvite::generate_example("en-US", &receiver),
            )
        }
    };

    send_mail_v1(&smtp_client, &mail_builder, &message)
        .await
        .unwrap();
}
