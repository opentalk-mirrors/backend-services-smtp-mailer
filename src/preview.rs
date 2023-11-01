// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use chrono::Timelike;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use mail_worker_protocol as protocol;
use protocol::v1::{CallIn, Event, Room, Time};
use smtp_mailer::{send_mail_v1, settings, MailBuilder, MailTemplate};
use std::time::Duration;
use types::common::shared_folder::{SharedFolder, SharedFolderAccess};
use uuid::Uuid;

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

fn default_unregistered_invite(
    lang: &str,
    to_overwrite: Option<String>,
) -> protocol::v1::UnregisteredEventInvite {
    let next_hour: chrono::DateTime<chrono::Utc> = (chrono::Utc::now()
        + chrono::Duration::hours(1))
    .with_minute(0)
    .unwrap()
    .with_second(0)
    .unwrap();

    protocol::v1::UnregisteredEventInvite {
        invitee: protocol::v1::UnregisteredUser{
            email: to_overwrite
            .unwrap_or_else(|| "receiver@example.org".into())
            .into(),
            first_name: "".to_string(),
            last_name: "".to_string(),
        },
        event: Event {
            id: Uuid::new_v4(),
            name: "This is a Preview Event".into(),
            description: "This event is a dummy event, and you should have got this invite as a keycloak user. You can safely ignore this description".into(),
            room: Room {
                id: Uuid::nil(),
                password: Some("password123".into()),
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
        },
        inviter: protocol::v1::RegisteredUser {
            email: "sender@example.org".into(),
            title: "10x developer".into(),
            first_name: "Alice".into(),
            last_name: "Wonderland".into(),
            language: lang.into(),
        },
    }
}

fn default_registered_invite(
    lang: &str,
    to_overwrite: Option<String>,
) -> protocol::v1::RegisteredEventInvite {
    let next_hour: chrono::DateTime<chrono::Utc> = (chrono::Utc::now()
        + chrono::Duration::hours(1))
    .with_minute(0)
    .unwrap()
    .with_second(0)
    .unwrap();

    protocol::v1::RegisteredEventInvite {
        invitee: protocol::v1::RegisteredUser {
            email: to_overwrite
                .unwrap_or_else(|| "receiver@example.org".into())
                .into(),
            title: "10x developer".into(),
            first_name: "Alice".into(),
            last_name: "Wonderland".into(),
            language: lang.into(),
        },
        event: Event {
            id: Uuid::new_v4(),
            name: "This is a Preview Event".into(),
            description: "This event is a dummy event, and you should have got this invite as a registered user. You can safely ignore this description".into(),
            room: Room {
                id: Uuid::nil(),
                password: Some("password123".to_owned()),
            },
            start_time: Some(Time {
                time: next_hour,
                timezone: "Europe/Berlin".into(),
            }),
            end_time: Some(Time {
                time: next_hour + chrono::Duration::minutes(40),
                timezone: "Europe/Berlin".into(),
            }),
            rrule: Some("RRULE:FREQ=WEEKLY;COUNT=30;INTERVAL=1".to_owned()),
            call_in: Some(CallIn {
                sip_tel: "+4930405051330".to_owned(),
                sip_id: "0123456789".to_owned(),
                sip_password: "555NASE".to_owned(),
            }),
            revision: 0,
            shared_folder: build_shared_folder(),
            adhoc_retention_seconds: Some(1345),
        },
        inviter: protocol::v1::RegisteredUser {
            email: "sender@example.org".into(),
            title: "10x developer".into(),
            first_name: "Alice".into(),
            last_name: "Wonderland".into(),
            language: lang.into(),
        },
    }
}

fn default_external_invite(
    lang: &str,
    to_overwrite: Option<String>,
) -> protocol::v1::ExternalEventInvite {
    let next_hour: chrono::DateTime<chrono::Utc> = (chrono::Utc::now()
        + chrono::Duration::hours(1))
    .with_minute(0)
    .unwrap()
    .with_second(0)
    .unwrap();

    protocol::v1::ExternalEventInvite {
        invitee: protocol::v1::ExternalUser{
            email: to_overwrite
            .unwrap_or_else(|| "receiver@example.org".into())
            .into(),
        },
        event: Event {
            id: Uuid::from_u128(1),
            name: "This is a Preview Event".into(),
            description: "This event is a dummy event, and you should have got this invite as a external user. You can safely ignore this description".into(),
            room: Room {
                id: Uuid::nil(),
                password: Some("password123".into()),
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
            adhoc_retention_seconds: Some(86400),
        },
        inviter: protocol::v1::RegisteredUser {
            email: "sender@example.org".into(),
            title: "10x developer".into(),
            first_name: "Alice".into(),
            last_name: "Wonderland".into(),
            language: lang.into(),
        },
        invite_code: Uuid::nil().to_string(),
    }
}

fn default_unregistered_cancellation(
    lang: &str,
    to_overwrite: Option<String>,
    event_id: Uuid,
) -> protocol::v1::UnregisteredEventCancellation {
    let next_hour: chrono::DateTime<chrono::Utc> = (chrono::Utc::now()
        + chrono::Duration::hours(1))
    .with_minute(0)
    .unwrap()
    .with_second(0)
    .unwrap();

    protocol::v1::UnregisteredEventCancellation {
        invitee: protocol::v1::UnregisteredUser{
            email: to_overwrite
            .unwrap_or_else(|| "receiver@example.org".into())
            .into(),
            first_name: "".to_string(),
            last_name: "".to_string(),
        },
        event: Event {
            id: event_id,
            name: "This is a Preview Event".into(),
            description: "This event is a dummy event, and you should have got this invite as a keycloak user. You can safely ignore this description".into(),
            room: Room {
                id: Uuid::nil(),
                password: Some("password123".into()),
            },
            start_time: Some(Time {
                time: next_hour,
                timezone: "Europe/Berlin".into(),
            }),
            end_time: Some(Time {
                time: next_hour+ chrono::Duration::minutes(40),
                timezone: "Europe/Berlin".into(),
            }),
            rrule: None,
            call_in: Some(CallIn {
                sip_tel: "+4930405051330".into(),
                sip_id: "1234567890".into(),
                sip_password: "1234567890".into(),
            }),
            revision: 1,
            shared_folder: build_shared_folder(),
            adhoc_retention_seconds: Some(86400),
        },
        inviter: protocol::v1::RegisteredUser {
            email: "sender@example.org".into(),
            title: "10x developer".into(),
            first_name: "Alice".into(),
            last_name: "Wonderland".into(),
            language: lang.into(),
        },
    }
}

fn default_registered_cancellation(
    lang: &str,
    to_overwrite: Option<String>,
    event_id: Uuid,
) -> protocol::v1::RegisteredEventCancellation {
    let next_hour: chrono::DateTime<chrono::Utc> = (chrono::Utc::now()
        + chrono::Duration::hours(1))
    .with_minute(0)
    .unwrap()
    .with_second(0)
    .unwrap();

    protocol::v1::RegisteredEventCancellation {
        invitee: protocol::v1::RegisteredUser {
            email: to_overwrite
                .unwrap_or_else(|| "receiver@example.org".into())
                .into(),
            title: "10x developer".into(),
            first_name: "Alice".into(),
            last_name: "Wonderland".into(),
            language: lang.into(),
        },
        event: Event {
            id: event_id,
            name: "This is a Preview Event".into(),
            description: "This event is a dummy event, and you should have got this invite as a registered user. You can safely ignore this description".into(),
            room: Room {
                id: Uuid::nil(),
                password: Some("password123".to_owned()),
            },
            start_time: Some(Time {
                time: next_hour,
                timezone: "Europe/Berlin".into(),
            }),
            end_time: Some(Time {
                time: next_hour + chrono::Duration::minutes(40),
                timezone: "Europe/Berlin".into(),
            }),
            rrule: Some("RRULE:FREQ=WEEKLY;COUNT=30;INTERVAL=1".to_owned()),
            call_in: Some(CallIn {
                sip_tel: "+4930405051330".to_owned(),
                sip_id: "0123456789".to_owned(),
                sip_password: "555NASE".to_owned(),
            }),
            revision: 1,
            shared_folder: build_shared_folder(),
            adhoc_retention_seconds: Some(86400),
        },
        inviter: protocol::v1::RegisteredUser {
            email: "sender@example.org".into(),
            title: "10x developer".into(),
            first_name: "Alice".into(),
            last_name: "Wonderland".into(),
            language: lang.into(),
        },
    }
}

fn default_external_cancellation(
    lang: &str,
    to_overwrite: Option<String>,
    event_id: Uuid,
) -> protocol::v1::ExternalEventCancellation {
    let next_hour: chrono::DateTime<chrono::Utc> = (chrono::Utc::now()
        + chrono::Duration::hours(1))
    .with_minute(0)
    .unwrap()
    .with_second(0)
    .unwrap();

    protocol::v1::ExternalEventCancellation {
        invitee: protocol::v1::ExternalUser{
            email: to_overwrite
            .unwrap_or_else(|| "receiver@example.org".into())
            .into(),
        },
        event: Event {
            id: event_id,
            name: "This is a Preview Event".into(),
            description: "This event is a dummy event, and you should have got this invite as a external user. You can safely ignore this description".into(),
            room: Room {
                id: Uuid::nil(),
                password: Some("password123".into()),
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
            revision: 1,
            shared_folder: build_shared_folder(),
            adhoc_retention_seconds: Some(86400),
        },
        inviter: protocol::v1::RegisteredUser {
            email: "sender@example.org".into(),
            title: "10x developer".into(),
            first_name: "Alice".into(),
            last_name: "Wonderland".into(),
            language: lang.into(),
        },
    }
}

pub fn preview_unregistered_invite(
    settings: &settings::Settings,
    html: bool,
    lang: &str,
) -> String {
    let mail_builder = MailBuilder::new(settings).unwrap();

    let demo_message = default_unregistered_invite(lang, None);

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

pub fn preview_registered_invite(settings: &settings::Settings, html: bool, lang: &str) -> String {
    let mail_builder = MailBuilder::new(settings).unwrap();

    let demo_message = default_registered_invite(lang, None);

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

pub fn preview_external_invite(settings: &settings::Settings, html: bool, lang: &str) -> String {
    let mail_builder = MailBuilder::new(settings).unwrap();

    let demo_message = default_external_invite(lang, None);

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

pub fn preview_registered_cancellation(
    settings: &settings::Settings,
    html: bool,
    lang: &str,
) -> String {
    let mail_builder = MailBuilder::new(settings).unwrap();

    let demo_message = default_registered_cancellation(lang, None, Uuid::new_v4());

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

pub fn preview_unregistered_cancellation(
    settings: &settings::Settings,
    html: bool,
    lang: &str,
) -> String {
    let mail_builder = MailBuilder::new(settings).unwrap();

    let demo_message = default_registered_cancellation(lang, None, Uuid::new_v4());

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

    let message = match template {
        crate::TemplateVariant::RegisteredInvite => protocol::v1::Message::RegisteredEventInvite(
            default_registered_invite("en-US", Some(to)),
        ),
        crate::TemplateVariant::UnregisteredInvite => {
            protocol::v1::Message::UnregisteredEventInvite(default_unregistered_invite(
                "en-US",
                Some(to),
            ))
        }
        crate::TemplateVariant::ExternalInvite => {
            protocol::v1::Message::ExternalEventInvite(default_external_invite("en-US", Some(to)))
        }
        crate::TemplateVariant::RegisteredCancellation => {
            let invite = default_registered_invite("en-US", Some(to.clone()));
            let event_id = invite.event.id;

            send_mail_v1(
                &smtp_client,
                &mail_builder,
                &protocol::v1::Message::RegisteredEventInvite(invite),
            )
            .await
            .unwrap();

            tokio::time::sleep(duration_between_invite_and_cancellation).await;

            protocol::v1::Message::RegisteredEventCancellation(default_registered_cancellation(
                "en-US",
                Some(to),
                event_id,
            ))
        }
        crate::TemplateVariant::UnregisteredCancellation => {
            let invite = default_unregistered_invite("en-US", Some(to.clone()));
            let event_id = invite.event.id;

            send_mail_v1(
                &smtp_client,
                &mail_builder,
                &protocol::v1::Message::UnregisteredEventInvite(invite),
            )
            .await
            .unwrap();

            tokio::time::sleep(duration_between_invite_and_cancellation).await;

            protocol::v1::Message::UnregisteredEventCancellation(default_unregistered_cancellation(
                "en-US",
                Some(to),
                event_id,
            ))
        }
        crate::TemplateVariant::ExternalCancellation => {
            let invite = default_external_invite("en-US", Some(to.clone()));
            let event_id = invite.event.id;

            send_mail_v1(
                &smtp_client,
                &mail_builder,
                &protocol::v1::Message::ExternalEventInvite(invite),
            )
            .await
            .unwrap();

            tokio::time::sleep(duration_between_invite_and_cancellation).await;

            protocol::v1::Message::ExternalEventCancellation(default_external_cancellation(
                "en-US",
                Some(to),
                event_id,
            ))
        }
    };

    send_mail_v1(&smtp_client, &mail_builder, &message)
        .await
        .unwrap();
}
