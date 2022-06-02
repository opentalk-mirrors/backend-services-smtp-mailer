use anyhow::Result;
use fluent_templates::FluentLoader;
use lettre::{
    message::{header, Mailbox, MultiPart, SinglePart},
    Message,
};
use mail_worker_proto as proto;
use tera::Tera;

use crate::settings;

mod registered_invite;
mod unregistered_invite;

pub(crate) fn create_template_engine(settings: &settings::Settings) -> Result<Tera> {
    let mut tera = Tera::default();
    tera.add_raw_template(
        "macros.html",
        include_str!("../../resources/templates/macros.html"),
    )?;
    tera.add_template_files(
        settings
            .templates
            .iter()
            .map(|(path, name)| (path, Some(name))),
    )?;
    tera.build_inheritance_chains()?;

    tera.register_function("fluent", FluentLoader::new(&*crate::i18n::LOCALES));

    Ok(tera)
}

pub struct MailBuilder {
    frontend: settings::Frontend,
    builder: settings::TemplateBuilder,
    default_language: String,
    tera: Tera,
}

pub trait MailTemplate {
    fn generate_email_plain(&self, builder: &MailBuilder) -> Result<String>;
    fn generate_email_html(&self, builder: &MailBuilder) -> Result<String>;
    fn generate_subject(&self, builder: &MailBuilder) -> Result<String>;
}

impl MailBuilder {
    pub fn new(settings: &settings::Settings) -> Result<Self> {
        let tera = create_template_engine(settings)?;

        Ok(Self {
            frontend: settings.frontend.to_owned(),
            builder: settings.template_builder.to_owned(),
            default_language: settings.languages.default_language.to_owned(),
            tera,
        })
    }

    fn create_join_link(&self, event: &proto::v1::Event) -> String {
        let template = &self.builder.join_link_builder;
        template
            .replace("{base_url", &self.frontend.base_url)
            .replace("{room_id}", &event.room.id.to_string())
    }

    fn create_dashboard_event_link(&self, event: &proto::v1::Event) -> String {
        let template = &self.builder.dashboard_event_link_builder;
        template
            .replace("{base_url", &self.frontend.base_url)
            .replace("{event_id}", &event.id.to_string())
    }

    pub(crate) fn generate_email(&self, message: &proto::v1::Message) -> Result<Message> {
        let (from_mb, to_mb, subject, txt, html) = match message {
            proto::v1::Message::RegisteredEventInvite(message) => {
                // TODO Move this to the trait as well.
                let from = Mailbox::new(
                    Some(generate_mailbox_name(
                        &message.inviter.title,
                        &message.inviter.first_name,
                        &message.inviter.last_name,
                    )),
                    message.inviter.email.as_ref().parse()?,
                );

                let to = Mailbox::new(
                    Some(generate_mailbox_name(
                        &message.invitee.title,
                        &message.invitee.first_name,
                        &message.invitee.last_name,
                    )),
                    message.invitee.email.as_ref().parse()?,
                );

                let subject = message.generate_subject(self)?;
                let txt = message.generate_email_plain(self)?;
                let html = message.generate_email_html(self)?;

                (from, to, subject, txt, html)
            }
            proto::v1::Message::UnregisteredEventInvite(message) => {
                let from = Mailbox::new(
                    Some(generate_mailbox_name(
                        &message.inviter.title,
                        &message.inviter.first_name,
                        &message.inviter.last_name,
                    )),
                    message.inviter.email.as_ref().parse()?,
                );

                let to = Mailbox::new(None, message.invitee.as_ref().parse()?);

                let subject = message.generate_subject(self)?;
                let txt = message.generate_email_plain(self)?;
                let html = message.generate_email_html(self)?;

                (from, to, subject, txt, html)
            }
        };

        let email = lettre::Message::builder()
            .from(from_mb)
            .to(to_mb)
            .subject(subject)
            .multipart(
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
