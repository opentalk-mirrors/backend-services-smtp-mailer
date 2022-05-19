use crate::{mail::MailBuilder, settings};
use chrono::FixedOffset;
use mail_worker_proto as proto;
use proto::v1::{CallIn, Event, Room, Time};
use uuid::Uuid;

pub fn preview_invite(settings: &settings::Settings, html: bool, lang: &str) -> String {
    let mail_builder = MailBuilder::new(settings).unwrap();

    let demo_message = proto::v1::Message::RegisteredEventInvite {
        invitee: proto::v1::User {
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
                password: "".into(),
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
            call_in: CallIn {
                sip_tel: None,
                sip_id: None,
                sip_password: None,
            },
        },
        inviter: proto::v1::User {
            email: "sender@example.org".into(),
            title: "10x developer".into(),
            first_name: "Alice".into(),
            last_name: "Wonderland".into(),
            language: lang.into(),
        },
    };

    if html {
        return mail_builder
            .generate_email_html(&demo_message)
            .unwrap_or_else(|e| {
                log::error!("Error while generating html: {e:?}");
                "".into()
            });
    }

    mail_builder
        .generate_email_body(&demo_message)
        .unwrap_or_else(|e| {
            log::error!("Error while generating plain text: {e:?}");
            "".into()
        })
}
