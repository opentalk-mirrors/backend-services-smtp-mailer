use std::collections::HashMap;

use crate::{i18n, settings};
use anyhow::Result;
use fluent_templates::{fluent_bundle::FluentValue, FluentLoader, Loader};
use lettre::message::{Mailbox, MessageBuilder};
use mail_worker_proto as proto;
use tera::Tera;

pub(crate) fn create_template_engine(settings: &settings::Settings) -> Result<Tera> {
    let mut tera = Tera::default();
    tera.add_raw_template(
        "macros.html",
        include_str!("../resources/templates/macros.html"),
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
    builder: settings::TemplateBuilder,
    default_language: String,
    tera: Tera,
}

impl MailBuilder {
    pub fn new(settings: &settings::Settings) -> Result<Self> {
        let tera = create_template_engine(settings)?;

        Ok(Self {
            builder: settings.template_builder.to_owned(),
            default_language: settings.languages.default_language.to_owned(),
            tera,
        })
    }

    pub(crate) fn generate_email_body(&self, message: &proto::v1::Message) -> Result<String> {
        match message {
            proto::v1::Message::RegisteredEventInvite {
                event,
                invitee,
                inviter,
            } => {
                let mut context = tera::Context::new();
                let language = if !invitee.language.is_empty() {
                    &invitee.language
                } else {
                    &self.default_language
                };
                context.insert("language", &dbg!(language));

                context.insert("invitee", invitee);
                context.insert("inviter", inviter);
                context.insert("event", event);
                context.insert("join_link", &self.create_join_link(event));
                context.insert("event_link", &self.create_event_link(event));

                self.tera.render("invite.txt", &context).map_err(Into::into)
            }
        }
    }

    pub(crate) fn generate_email_html(&self, message: &proto::v1::Message) -> Result<String> {
        match message {
            proto::v1::Message::RegisteredEventInvite {
                event,
                invitee,
                inviter,
            } => {
                let mut context = tera::Context::new();
                let language = if !invitee.language.is_empty() {
                    &invitee.language
                } else {
                    &self.default_language
                };
                context.insert("language", &dbg!(language));

                context.insert("invitee", invitee);
                context.insert("inviter", inviter);
                context.insert("event", event);
                context.insert("join_link", &self.create_join_link(event));
                context.insert("event_link", &self.create_event_link(event));

                let html = self.tera.render("invite.html", &context)?;

                let inliner = css_inline::CSSInliner::options().build();
                inliner.inline(&html).map_err(Into::into)
            }
        }
    }

    fn create_join_link(&self, event: &proto::v1::Event) -> String {
        let template = &self.builder.join_link_builder;
        template.replace("{room_id}", &event.room.id.to_string())
    }

    fn create_event_link(&self, event: &proto::v1::Event) -> String {
        let template = &self.builder.dashboard_event_link_builder;
        template.replace("{event_id}", &event.id.to_string())
    }

    pub(crate) fn generate_email(&self, message: &proto::v1::Message) -> Result<MessageBuilder> {
        let (from_mb, to_mb, subject) = match message {
            proto::v1::Message::RegisteredEventInvite {
                invitee,
                inviter,
                event,
            } => {
                let from = Mailbox::new(
                    Some(generate_mailbox_name(
                        &inviter.title,
                        &inviter.first_name,
                        &inviter.last_name,
                    )),
                    inviter.email.parse()?,
                );
                let to = Mailbox::new(
                    Some(generate_mailbox_name(
                        &invitee.title,
                        &invitee.first_name,
                        &invitee.last_name,
                    )),
                    invitee.email.parse()?,
                );

                let subject_args = subject_args(event, invitee, inviter);
                let lang = if !invitee.language.is_empty() {
                    invitee.language.parse()?
                } else {
                    self.default_language.parse()?
                };

                let subject = i18n::LOCALES.lookup_complete(
                    &lang,
                    "registered-event-invite-subject",
                    Some(&subject_args),
                );

                (from, to, subject)
            }
        };

        let email = lettre::Message::builder()
            .from(from_mb)
            .to(to_mb)
            .subject(subject);

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

fn subject_args(
    event: &proto::v1::Event,
    invitee: &proto::v1::User,
    inviter: &proto::v1::User,
) -> HashMap<String, FluentValue<'static>> {
    let mut args: HashMap<String, FluentValue<'static>> = HashMap::new();
    args.insert(
        "inviter-first_name".to_owned(),
        inviter.first_name.clone().into(),
    );
    args.insert(
        "inviter-last_name".to_owned(),
        inviter.first_name.clone().into(),
    );
    args.insert(
        "inviter-title".to_owned(),
        inviter.first_name.clone().into(),
    );
    args.insert(
        "invitee-first_name".to_owned(),
        invitee.first_name.clone().into(),
    );
    args.insert(
        "invitee-last_name".to_owned(),
        invitee.last_name.clone().into(),
    );
    args.insert("invitee-title".to_owned(), invitee.title.clone().into());

    args.insert("event-name".to_owned(), event.name.clone().into());
    args
}
