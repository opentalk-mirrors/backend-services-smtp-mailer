use super::MailTemplate;
use fluent_templates::{fluent_bundle::FluentValue, Loader};
use mail_worker_proto as protocol;
use protocol::v1::RegisteredEventInvite;
use std::collections::HashMap;

use crate::i18n;

impl MailTemplate for RegisteredEventInvite {
    fn generate_email_plain(&self, builder: &super::MailBuilder) -> anyhow::Result<String> {
        let mut context = tera::Context::new();
        let language = if !self.invitee.language.is_empty() {
            &self.invitee.language
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

        builder
            .tera
            .render("registered_invite.txt", &context)
            .map_err(Into::into)
    }

    fn generate_email_html(&self, builder: &super::MailBuilder) -> anyhow::Result<String> {
        let mut context = tera::Context::new();
        let language = if !self.invitee.language.is_empty() {
            &self.invitee.language
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

        let html = builder.tera.render("registered_invite.html", &context)?;

        let inliner = css_inline::CSSInliner::options().build();
        inliner.inline(&html).map_err(Into::into)
    }

    fn generate_subject(&self, builder: &super::MailBuilder) -> anyhow::Result<String> {
        let subject_args = subject_args(&self.event, &self.invitee, &self.inviter);
        let lang = if !self.invitee.language.is_empty() {
            self.invitee.language.parse()?
        } else {
            builder.default_language.parse()?
        };

        Ok(i18n::LOCALES.lookup_complete(
            &lang,
            "registered-event-invite-subject",
            Some(&subject_args),
        ))
    }
}

fn subject_args(
    event: &protocol::v1::Event,
    invitee: &protocol::v1::User,
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
