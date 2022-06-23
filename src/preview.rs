use lettre::{AsyncSmtpTransport, Tokio1Executor};
use mail_worker_protocol as protocol;
use protocol::v1::{CallIn, Event, Room, Time};
use smtp_mailer::{send_mail_v1, settings, MailBuilder, MailTemplate};
use uuid::Uuid;

fn default_unregistered_invite(
    lang: &str,
    to_overwrite: Option<String>,
) -> protocol::v1::UnregisteredEventInvite {
    protocol::v1::UnregisteredEventInvite {
        invitee: to_overwrite
            .unwrap_or_else(|| "receiver@example.org".into())
            .into(),
        event: Event {
            id: Uuid::new_v4(),
            name: "This is a Preview Event".into(),
            description: "This event is a dummy event, and you should have got this invite as a keycloak user. You can safely ignore this description".into(),
            room: Room {
                id: Uuid::nil(),
                password: Some("password123".into()),
            },
            start_time: Some(Time {
                time: chrono::Utc::now() + chrono::Duration::minutes(10),
                timezone: "Europe/Berlin".into(),
            }),
            end_time: Some(Time {
                time: chrono::Utc::now() + chrono::Duration::minutes(40),
                timezone: "Europe/Berlin".into(),
            }),
            rrule: None,
            call_in: Some(CallIn {
                sip_tel: "+4930405051330".into(),
                sip_id: "1234567890".into(),
                sip_password: "1234567890".into(),
            }),
        },
        inviter: protocol::v1::User {
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
    protocol::v1::RegisteredEventInvite {
        invitee: protocol::v1::User {
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
                time: chrono::Utc::now() + chrono::Duration::minutes(10),
                timezone: "Europe/Berlin".into(),
            }),
            end_time: Some(Time {
                time: chrono::Utc::now() + chrono::Duration::minutes(40),
                timezone: "Europe/Berlin".into(),
            }),
            rrule: Some("RRULE:FREQ=WEEKLY;COUNT=30;INTERVAL=1".to_owned()),
            call_in: Some(CallIn {
                sip_tel: "+4930405051330".to_owned(),
                sip_id: "0123456789".to_owned(),
                sip_password: "555NASE".to_owned(),
            }),
        },
        inviter: protocol::v1::User {
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
    protocol::v1::ExternalEventInvite {
        invitee: to_overwrite
            .unwrap_or_else(|| "invitee@example.org".into())
            .into(),
        event: Event {
            id: Uuid::from_u128(1),
            name: "This is a Preview Event".into(),
            description: "This event is a dummy event, and you should have got this invite as a external user. You can safely ignore this description".into(),
            room: Room {
                id: Uuid::nil(),
                password: Some("password123".into()),
            },
            start_time: Some(Time {
                time: chrono::Utc::now() + chrono::Duration::minutes(10),
                timezone: "Europe/Berlin".into(),
            }),
            end_time: Some(Time {
                time: chrono::Utc::now() + chrono::Duration::minutes(40),
                timezone: "Europe/Berlin".into(),
            }),
            rrule: None,
            call_in: Some(CallIn {
                sip_tel: "+4930405051330".into(),
                sip_id: "1234567890".into(),
                sip_password: "1234567890".into(),
            }),
        },
        inviter: protocol::v1::User {
            email: "sender@example.org".into(),
            title: "10x developer".into(),
            first_name: "Alice".into(),
            last_name: "Wonderland".into(),
            language: lang.into(),
        },
        invite_code: Uuid::nil().to_string(),
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

pub async fn preview_send_mail(
    settings: &settings::Settings,
    template: super::TemplateVariant,
    to: String,
) {
    let smtp_client: AsyncSmtpTransport<Tokio1Executor> =
        settings.smtp.smtp_server.clone().try_into().unwrap();
    let mail_builder = MailBuilder::new(settings).unwrap();
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
    };

    send_mail_v1(&smtp_client, &mail_builder, &message)
        .await
        .unwrap();
}
