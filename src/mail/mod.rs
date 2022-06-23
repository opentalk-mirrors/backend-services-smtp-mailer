use crate::{ics::create_ics_v1, settings};
use anyhow::Result;
use fluent_templates::FluentLoader;
use lettre::{
    message::{
        header::{self, ContentType},
        Attachment, Mailbox, MultiPart, SinglePart,
    },
    Message,
};
use mail_worker_protocol as proto;
use serde_json::{to_value, Value};
use std::collections::HashMap;
use tera::{try_get_value, Tera};

mod external_invite;
mod registered_invite;
mod unregistered_invite;

pub(crate) fn create_template_engine(settings: &settings::Settings) -> Result<Tera> {
    let mut tera = Tera::default();
    tera.add_raw_template(
        "macros.html",
        include_str!("../../resources/templates/macros.html"),
    )?;
    tera.add_raw_template(
        "common_styles.css",
        include_str!("../../resources/templates/common_styles.css"),
    )?;
    tera.add_template_files(
        settings
            .templates
            .iter()
            .map(|(path, name)| (path, Some(name))),
    )?;
    tera.build_inheritance_chains()?;

    tera.register_function("fluent", FluentLoader::new(&*crate::i18n::LOCALES));

    tera.register_filter("space_groups", space_groups_filter);
    tera.register_filter("format_telephone_number", format_telephone_number_filter);

    Ok(tera)
}

/// A builder that contains everything needed to render and create a Mail
pub struct MailBuilder {
    frontend: settings::Frontend,
    builder: settings::TemplateBuilder,
    default_language: String,
    support_contact: Option<settings::SupportContact>,
    tera: Tera,
}

/// Trait to render the different tasks
pub trait MailTemplate {
    fn generate_email_plain(&self, builder: &MailBuilder) -> Result<String>;
    fn generate_email_html(&self, builder: &MailBuilder) -> Result<String>;
    fn generate_subject(&self, builder: &MailBuilder) -> Result<String>;
    fn generate_from_mbox(&self, builder: &MailBuilder) -> Result<Mailbox>;
    fn generate_to_mbox(&self, builder: &MailBuilder) -> Result<Mailbox>;
}

impl MailBuilder {
    pub fn new(settings: &settings::Settings) -> Result<Self> {
        let tera = create_template_engine(settings)?;

        Ok(Self {
            frontend: settings.frontend.to_owned(),
            builder: settings.template_builder.to_owned(),
            default_language: settings.languages.default_language.to_owned(),
            support_contact: settings.support_contact.to_owned(),
            tera,
        })
    }

    fn create_join_link(&self, event: &proto::v1::Event) -> String {
        let template = &self.builder.join_link_builder;
        template
            .replace("{base_url}", &self.frontend.base_url)
            .replace("{room_id}", &event.room.id.to_string())
    }

    fn create_dashboard_event_link(&self, event: &proto::v1::Event) -> String {
        let template = &self.builder.dashboard_event_link_builder;
        template
            .replace("{base_url}", &self.frontend.base_url)
            .replace("{event_id}", &event.id.to_string())
    }

    fn create_room_invite_link(&self, invite_id: &str) -> String {
        let template = &self.builder.guest_link_builder;
        template
            .replace("{base_url}", &self.frontend.base_url)
            .replace("{invite_id}", invite_id)
    }

    pub(crate) fn generate_email(&self, message: &proto::v1::Message) -> Result<Message> {
        let from_mb = message.generate_from_mbox(self)?;

        let to_mb = message.generate_to_mbox(self)?;

        let subject = message.generate_subject(self)?;
        let txt = message.generate_email_plain(self)?;
        let html = message.generate_email_html(self)?;

        let mut mail_content = MultiPart::alternative()
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_PLAIN)
                    .body(txt),
            )
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(html),
            );

        if let Some(attachments) = create_attachments(message)? {
            let mut builder = MultiPart::mixed().multipart(mail_content);

            for (name, content_type, body) in attachments.into_iter() {
                let file = Attachment::new(name).body(body, content_type);
                builder = builder.singlepart(file);
            }
            mail_content = builder;
        }

        let email = lettre::Message::builder()
            .from(from_mb)
            .to(to_mb)
            .subject(subject)
            .multipart(mail_content)?;

        Ok(email)
    }
}

fn generate_mailbox_name(title: &str, first_name: &str, last_name: &str) -> String {
    if title.is_empty() {
        format!("{first_name} {last_name}")
    } else {
        format!("{title} {first_name} {last_name}")
    }
}

fn create_attachments(
    message: &proto::v1::Message,
) -> Result<Option<impl IntoIterator<Item = (String, ContentType, Vec<u8>)>>> {
    match message {
        proto::v1::Message::RegisteredEventInvite(message) => {
            let name = format!(
                "{} {}",
                &message.invitee.first_name, &message.invitee.last_name
            );
            let invitee = crate::ics::Invitee::WithName {
                email: message.invitee.email.as_ref(),
                name: &name,
            };

            let body = create_ics_v1(&message.inviter, &message.event, invitee)?;
            if let Some(body) = body {
                return Ok(Some(vec![(
                    "invite.ics".to_string(),
                    ContentType::parse("text/calendar").unwrap(),
                    body,
                )]));
            }
            Ok(None)
        }
        proto::v1::Message::UnregisteredEventInvite(message) => {
            let invitee = crate::ics::Invitee::WithoutName(message.invitee.as_ref());
            let body = create_ics_v1(&message.inviter, &message.event, invitee)?;
            if let Some(body) = body {
                return Ok(Some(vec![(
                    "invite.ics".to_string(),
                    ContentType::parse("text/calendar").unwrap(),
                    body,
                )]));
            }
            Ok(None)
        }
        proto::v1::Message::ExternalEventInvite(message) => {
            let invitee = crate::ics::Invitee::WithoutName(message.invitee.as_ref());
            let body = create_ics_v1(&message.inviter, &message.event, invitee)?;
            if let Some(body) = body {
                return Ok(Some(vec![(
                    "invite.ics".to_string(),
                    ContentType::parse("text/calendar").unwrap(),
                    body,
                )]));
            }
            Ok(None)
        }
    }
}

macro_rules! forward {
    ($self:ident, $fn:ident, $($arg:ident),*) => {
        match $self {
            ::mail_worker_protocol::v1::Message::RegisteredEventInvite(x) => x.$fn($($arg)*),
            ::mail_worker_protocol::v1::Message::UnregisteredEventInvite(x) => x.$fn($($arg)*),
            ::mail_worker_protocol::v1::Message::ExternalEventInvite(x) => x.$fn($($arg)*),
        }
    };
}

impl MailTemplate for proto::v1::Message {
    fn generate_email_plain(&self, builder: &MailBuilder) -> Result<String> {
        forward!(self, generate_email_plain, builder)
    }

    fn generate_email_html(&self, builder: &MailBuilder) -> Result<String> {
        forward!(self, generate_email_html, builder)
    }

    fn generate_subject(&self, builder: &MailBuilder) -> Result<String> {
        forward!(self, generate_subject, builder)
    }

    fn generate_from_mbox(&self, builder: &MailBuilder) -> Result<Mailbox> {
        forward!(self, generate_from_mbox, builder)
    }

    fn generate_to_mbox(&self, builder: &MailBuilder) -> Result<Mailbox> {
        forward!(self, generate_to_mbox, builder)
    }
}

pub fn space_groups_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("space_groups", "value", String, value);

    let grouped_string = s
        .chars()
        .enumerate()
        .fold(String::new(), |mut acc, (index, c)| {
            acc.push(c);
            if index % 4 == 3 {
                acc.push(' ');
            };

            acc
        });
    Ok(to_value(&grouped_string).unwrap())
}

pub fn format_telephone_number_filter(
    value: &Value,
    args: &HashMap<String, Value>,
) -> tera::Result<Value> {
    let input = try_get_value!("format_telephone_number", "value", String, value);

    let mode = match args.get("mode") {
        Some(val) => try_get_value!("format_telephone_number", "mode", String, val),
        None => "e164".to_owned(),
    };

    let mode = match mode.as_str() {
        "international" => phonenumber::Mode::International,
        "national" => phonenumber::Mode::National,
        "e164" => phonenumber::Mode::E164,
        "rfc3966" => phonenumber::Mode::Rfc3966,
        _ => phonenumber::Mode::E164,
    };

    let number = input.clone();
    let result = std::panic::catch_unwind(move || phonenumber::parse(None, number));

    let formatted_telehpone_number = match result {
        Ok(Ok(number)) => number.format().mode(mode).to_string(),
        e if mode == phonenumber::Mode::Rfc3966 => {
            log::warn!(" Failed to parse phone number {:?}", e);
            format!("tel:{input}")
        }
        e => {
            log::warn!(" Failed to parse phone number {:?}", e);
            input
        }
    };

    Ok(to_value(&formatted_telehpone_number).unwrap())
}
