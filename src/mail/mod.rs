// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{ics::EventStatus, settings};
use anyhow::Result;
use fluent_templates::{fluent_bundle::FluentValue, FluentLoader};
use lettre::{
    message::{
        header::{self, ContentDisposition, ContentTransferEncoding, ContentType},
        Mailbox, MultiPart, SinglePart,
    },
    Message,
};
use mail_worker_protocol as proto;
use serde_json::{to_value, Value};
use std::collections::HashMap;
use tera::{try_get_value, Tera};

mod external_event_cancellation;
mod external_event_update;
mod external_invite;
mod external_uninvite;
mod registered_event_cancellation;
mod registered_event_update;
mod registered_invite;
mod registered_uninvite;
mod unregistered_event_cancellation;
mod unregistered_event_update;
mod unregistered_invite;
mod unregistered_uninvite;

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
    tera.add_raw_template(
        "header_logo.include",
        include_str!("../../resources/templates/header_logo.include"),
    )?;
    tera.add_raw_template(
        "callin_txt.include",
        include_str!("../../resources/templates/callin_txt.include"),
    )?;
    tera.add_raw_template(
        "callin_html.include",
        include_str!("../../resources/templates/callin_html.include"),
    )?;
    tera.add_raw_template(
        "shared_folder_txt.include",
        include_str!("../../resources/templates/shared_folder_txt.include"),
    )?;
    tera.add_raw_template(
        "shared_folder_html.include",
        include_str!("../../resources/templates/shared_folder_html.include"),
    )?;
    tera.add_template_file(
        "resources/templates/quick_guide_txt.include",
        Some("quick_guide_txt.include"),
    )?;
    tera.add_template_file(
        "resources/templates/quick_guide_html.include",
        Some("quick_guide_html.include"),
    )?;
    tera.add_template_file(
        "resources/templates/data_protection_txt.include",
        Some("data_protection_txt.include"),
    )?;
    tera.add_template_file(
        "resources/templates/data_protection_html.include",
        Some("data_protection_html.include"),
    )?;
    tera.add_template_file(
        "resources/templates/data_protection_ics.include",
        Some("data_protection_ics.include"),
    )?;
    tera.add_raw_template(
        "ics_description.txt",
        include_str!("../../resources/templates/ics_description.txt"),
    )?;
    tera.add_template_file(
        "resources/templates/adhoc_txt.include",
        Some("adhoc_txt.include"),
    )?;
    tera.add_template_file(
        "resources/templates/adhoc_html.include",
        Some("adhoc_html.include"),
    )?;

    tera.add_template_files(
        settings
            .templates
            .iter()
            .map(|(path, name)| (path, Some(name))),
    )?;
    tera.build_inheritance_chains()?;

    tera.register_function("fluent", FluentLoader::new(&*crate::i18n::LOCALES));

    tera.register_filter("wrap_text", wrap_text_filter);
    tera.register_filter("build_link", build_link);
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
    from_name: String,
    from_email: String,
    tera: Tera,
}

/// Trait to render the different tasks
pub trait MailTemplate {
    fn generate_email_plain(&self, builder: &MailBuilder) -> Result<String>;
    fn generate_email_html(&self, builder: &MailBuilder) -> Result<String>;
    fn generate_subject(&self, builder: &MailBuilder) -> Result<String>;
    fn generate_reply_to_mbox(&self, builder: &MailBuilder) -> Result<Mailbox>;
    fn generate_to_mbox(&self, builder: &MailBuilder) -> Result<Mailbox>;
    fn generate_attachments(&self, builder: &MailBuilder) -> Result<Vec<SinglePart>>;
}

impl MailBuilder {
    pub fn new(settings: &settings::Settings) -> Result<Self> {
        let tera = create_template_engine(settings)?;

        Ok(Self {
            frontend: settings.frontend.to_owned(),
            builder: settings.template_builder.to_owned(),
            default_language: settings.languages.default_language.to_owned(),
            support_contact: settings.support_contact.to_owned(),
            from_name: settings.smtp.from_name.to_owned(),
            from_email: settings.smtp.from_email.to_owned(),
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
        let from_mb = generate_from_mbox(self.from_name.clone(), self.from_email.as_ref())?;

        let reply_to_mb = message.generate_reply_to_mbox(self)?;
        let to_mb = message.generate_to_mbox(self)?;

        let subject = message.generate_subject(self)?;
        let txt = message.generate_email_plain(self)?;
        let html = message.generate_email_html(self)?;

        let mut mail_parts = MultiPart::mixed().multipart(
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
        );

        for attachment in message.generate_attachments(self)? {
            mail_parts = mail_parts.singlepart(attachment);
        }

        let email = lettre::Message::builder()
            .from(from_mb)
            .reply_to(reply_to_mb)
            .to(to_mb)
            .subject(subject)
            .multipart(mail_parts)?;

        Ok(email)
    }
}

fn generate_mailbox_name(title: &str, first_name: &str, last_name: &str) -> String {
    format!("{title} {first_name} {last_name}")
        .trim()
        .to_string()
}

macro_rules! forward {
    ($self:ident, $fn:ident, $($arg:ident),*) => {
        match $self {
            // Invites
            ::mail_worker_protocol::v1::Message::RegisteredEventInvite(x) => x.$fn($($arg)*),
            ::mail_worker_protocol::v1::Message::UnregisteredEventInvite(x) => x.$fn($($arg)*),
            ::mail_worker_protocol::v1::Message::ExternalEventInvite(x) => x.$fn($($arg)*),
            // Updates
            ::mail_worker_protocol::v1::Message::RegisteredEventUpdate(x) => x.$fn($($arg)*),
            ::mail_worker_protocol::v1::Message::UnregisteredEventUpdate(x) => x.$fn($($arg)*),
            ::mail_worker_protocol::v1::Message::ExternalEventUpdate(x) => x.$fn($($arg)*),
            // Cancellations
            ::mail_worker_protocol::v1::Message::RegisteredEventCancellation(x) => x.$fn($($arg)*),
            ::mail_worker_protocol::v1::Message::UnregisteredEventCancellation(x) => x.$fn($($arg)*),
            ::mail_worker_protocol::v1::Message::ExternalEventCancellation(x) => x.$fn($($arg)*),
            // Uninvites
            ::mail_worker_protocol::v1::Message::RegisteredEventUninvite(x) => x.$fn($($arg)*),
            ::mail_worker_protocol::v1::Message::UnregisteredEventUninvite(x) => x.$fn($($arg)*),
            ::mail_worker_protocol::v1::Message::ExternalEventUninvite(x) => x.$fn($($arg)*),
        }
    };
}

fn generate_from_mbox(name: String, email: &str) -> anyhow::Result<lettre::message::Mailbox> {
    Ok(Mailbox::new(Some(name), email.parse()?))
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

    fn generate_reply_to_mbox(&self, builder: &MailBuilder) -> Result<Mailbox> {
        forward!(self, generate_reply_to_mbox, builder)
    }

    fn generate_to_mbox(&self, builder: &MailBuilder) -> Result<Mailbox> {
        forward!(self, generate_to_mbox, builder)
    }

    fn generate_attachments(&self, builder: &MailBuilder) -> Result<Vec<SinglePart>> {
        forward!(self, generate_attachments, builder)
    }
}

pub fn wrap_text_filter(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("wrap_text", "value", String, value);

    let width = match args.get("width") {
        Some(width) => try_get_value!("wrap_text", "width", usize, width),
        None => 80,
    };

    let wrapped_string = textwrap::fill(s.as_str(), width);
    Ok(to_value(wrapped_string).unwrap())
}

pub fn build_link(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("build_link", "value", String, value);

    let caption = match args.get("caption") {
        Some(caption) => try_get_value!("build_link", "caption", String, caption),
        None => s.clone(),
    };

    let link_string = format!("<a href=\"{s}\">{caption}</a>");
    Ok(to_value(link_string).unwrap())
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
    Ok(to_value(grouped_string).unwrap())
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

    let formatted_telephone_number = match result {
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

    Ok(to_value(formatted_telephone_number).unwrap())
}

fn common_subject_args(
    event: &proto::v1::Event,
    inviter: &proto::v1::RegisteredUser,
) -> HashMap<String, FluentValue<'static>> {
    let mut args: HashMap<String, FluentValue<'static>> = HashMap::new();
    args.insert(
        "inviter-first_name".to_owned(),
        inviter.first_name.clone().into(),
    );
    args.insert(
        "inviter-last_name".to_owned(),
        inviter.last_name.clone().into(),
    );
    args.insert("inviter-title".to_owned(), inviter.title.clone().into());
    args.insert("event-name".to_owned(), event.name.clone().into());
    args
}

fn registered_invitee_subject_args(
    event: &proto::v1::Event,
    invitee: &proto::v1::RegisteredUser,
    inviter: &proto::v1::RegisteredUser,
) -> HashMap<String, FluentValue<'static>> {
    let mut args = common_subject_args(event, inviter);
    args.insert(
        "invitee-first_name".to_owned(),
        invitee.first_name.clone().into(),
    );
    args.insert(
        "invitee-last_name".to_owned(),
        invitee.last_name.clone().into(),
    );
    args.insert("invitee-title".to_owned(), invitee.title.clone().into());
    args
}

fn unregistered_invitee_subject_args(
    event: &proto::v1::Event,
    invitee: &proto::v1::UnregisteredUser,
    inviter: &proto::v1::RegisteredUser,
) -> HashMap<String, FluentValue<'static>> {
    let mut args = common_subject_args(event, inviter);
    args.insert(
        "invitee-first_name".to_owned(),
        invitee.first_name.clone().into(),
    );
    args.insert(
        "invitee-last_name".to_owned(),
        invitee.last_name.clone().into(),
    );
    args
}

fn external_invitee_subject_args(
    event: &proto::v1::Event,
    inviter: &proto::v1::RegisteredUser,
) -> HashMap<String, FluentValue<'static>> {
    common_subject_args(event, inviter)
}

fn create_ics_attachments(ics: Vec<u8>, event_status: EventStatus) -> Vec<SinglePart> {
    let (file_name, content_type) = match event_status {
        EventStatus::Created | EventStatus::Updated => (
            "invite.ics",
            ContentType::parse("text/calendar; charset=utf-8; method=REQUEST;").unwrap(),
        ),

        EventStatus::Cancelled => (
            "cancel.ics",
            ContentType::parse("text/calendar; charset=utf-8; method=CANCEL;").unwrap(),
        ),
    };

    let calendar_attachment = SinglePart::builder()
        .header(content_type)
        .header(ContentTransferEncoding::QuotedPrintable)
        .body(ics.clone());

    let ics_attachment = SinglePart::builder()
        .content_type(ContentType::parse("application/ics").unwrap())
        .header(ContentDisposition::attachment(file_name))
        .header(ContentTransferEncoding::Base64)
        .body(ics);

    vec![calendar_attachment, ics_attachment]
}
