use crate::{mail::MailBuilder, mail::MailTemplate, settings};
use chrono::FixedOffset;
use mail_worker_protocol as protocol;
use protocol::v1::{CallIn, Event, Room, Time};
use uuid::Uuid;

pub fn preview_unregistered_invite(
    settings: &settings::Settings,
    html: bool,
    lang: &str,
) -> String {
    let mail_builder = MailBuilder::new(settings).unwrap();

    let demo_message = protocol::v1::UnregisteredEventInvite {
        invitee: "receiver@example.org".into(),
        event: Event {
            id: Uuid::from_u128(1),
            name: "This is a Preview Event".into(),
            description: "You can safely ignore this description".into(),
            room: Room {
                id: Uuid::nil(),
                password: Some("password123".into()),
            },
            start_time: Some(Time {
                time: chrono::DateTime::<FixedOffset>::parse_from_rfc3339(
                    "2021-12-29T15:00:00+00:00",
                )
                .unwrap()
                .into(),
                timezone: "Europe/Berlin".into(),
            }),
            end_time: Some(Time {
                time: chrono::DateTime::<FixedOffset>::parse_from_rfc3339(
                    "2021-12-29T15:30:00+00:00",
                )
                .unwrap()
                .into(),
                timezone: "Europe/Berlin".into(),
            }),
            rrule: None,
            call_in: Some(CallIn {
                sip_tel: "+4912332112".into(),
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
    };

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

    let demo_message = protocol::v1::RegisteredEventInvite {
        invitee: protocol::v1::User {
            email: "receiver@example.org".into(),
            title: "10x developer".into(),
            first_name: "Alice".into(),
            last_name: "Wonderland".into(),
            language: lang.into(),
        },
        event: Event {
            id: Uuid::from_u128(1),
            name: "This is a Preview Event".into(),
            description: "You can safely ignore this description".into(),
            room: Room {
                id: Uuid::nil(),
                password: Some("password123".into()),
            },
            start_time: Some(Time {
                time: chrono::DateTime::<FixedOffset>::parse_from_rfc3339(
                    "2021-12-29T15:00:00+00:00",
                )
                .unwrap()
                .into(),
                timezone: "Europe/Berlin".into(),
            }),
            end_time: Some(Time {
                time: chrono::DateTime::<FixedOffset>::parse_from_rfc3339(
                    "2021-12-29T15:30:00+00:00",
                )
                .unwrap()
                .into(),
                timezone: "Europe/Berlin".into(),
            }),
            rrule: None,
            call_in: Some(CallIn {
                sip_tel: "+4912332112".into(),
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
    };

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

    let demo_message = protocol::v1::ExternalEventInvite {
        invitee: "receiver@example.org".into(),
        event: Event {
            id: Uuid::from_u128(1),
            name: "This is a Preview Event".into(),
            description: "You can safely ignore this description".into(),
            room: Room {
                id: Uuid::nil(),
                password: Some("password123".into()),
            },
            start_time: Some(Time {
                time: chrono::DateTime::<FixedOffset>::parse_from_rfc3339(
                    "2021-12-29T15:00:00+00:00",
                )
                .unwrap()
                .into(),
                timezone: "Europe/Berlin".into(),
            }),
            end_time: Some(Time {
                time: chrono::DateTime::<FixedOffset>::parse_from_rfc3339(
                    "2021-12-29T15:30:00+00:00",
                )
                .unwrap()
                .into(),
                timezone: "Europe/Berlin".into(),
            }),
            rrule: None,
            call_in: Some(CallIn {
                sip_tel: "+4912332112".into(),
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
    };

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
