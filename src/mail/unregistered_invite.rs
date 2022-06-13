use crate::i18n;
use crate::mail::MailTemplate;
use fluent_templates::{fluent_bundle::FluentValue, Loader};
use lettre::message::Mailbox;
use mail_worker_protocol as protocol;
use protocol::v1::UnregisteredEventInvite;
use std::collections::HashMap;

use super::generate_mailbox_name;

impl MailTemplate for UnregisteredEventInvite {
    fn generate_email_plain(&self, builder: &super::MailBuilder) -> anyhow::Result<String> {
        let mut context = tera::Context::new();
        let language = if !self.inviter.language.is_empty() {
            &self.inviter.language
        } else {
            &builder.default_language
        };
        context.insert("language", &language);

        context.insert("invitee-email", &self.invitee);
        context.insert("inviter", &self.inviter);
        context.insert("event", &self.event);
        context.insert("join_link", &builder.create_join_link(&self.event));
        context.insert(
            "event_link",
            &builder.create_dashboard_event_link(&self.event),
        );

        builder
            .tera
            .render("unregistered_invite.txt", &context)
            .map_err(Into::into)
    }

    fn generate_email_html(&self, builder: &super::MailBuilder) -> anyhow::Result<String> {
        let mut context = tera::Context::new();
        let language = if !self.inviter.language.is_empty() {
            &self.inviter.language
        } else {
            &builder.default_language
        };
        context.insert("language", &language);

        context.insert("invitee", &self.invitee);
        context.insert("inviter", &self.inviter);
        context.insert("event", &self.event);
        context.insert("join_link", &builder.create_join_link(&self.event));
        context.insert(
            "event_link",
            &builder.create_dashboard_event_link(&self.event),
        );

        let html = builder.tera.render("unregistered_invite.html", &context)?;

        let inliner = css_inline::CSSInliner::options().build();
        inliner.inline(&html).map_err(Into::into)
    }

    fn generate_subject(&self, builder: &super::MailBuilder) -> anyhow::Result<String> {
        let subject_args = subject_args(&self.event, &self.inviter);
        let lang = if !self.inviter.language.is_empty() {
            self.inviter.language.parse()?
        } else {
            builder.default_language.parse()?
        };

        Ok(i18n::LOCALES.lookup_complete(
            &lang,
            "unregistered-event-invite-subject",
            Some(&subject_args),
        ))
    }

    fn generate_from_mbox(
        &self,
        _builder: &super::MailBuilder,
    ) -> anyhow::Result<lettre::message::Mailbox> {
        let mbox = Mailbox::new(
            Some(generate_mailbox_name(
                &self.inviter.title,
                &self.inviter.first_name,
                &self.inviter.last_name,
            )),
            self.inviter.email.as_ref().parse()?,
        );

        Ok(mbox)
    }

    fn generate_to_mbox(&self, _builder: &super::MailBuilder) -> anyhow::Result<Mailbox> {
        let mbox = Mailbox::new(None, self.invitee.as_ref().parse()?);

        Ok(mbox)
    }
}

fn subject_args(
    event: &protocol::v1::Event,
    inviter: &protocol::v1::User,
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

    args.insert("event-name".to_owned(), event.name.clone().into());
    args
}
