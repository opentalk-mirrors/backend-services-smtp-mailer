use super::{generate_mailbox_name, MailTemplate};
use crate::{i18n, ics::create_ics_v1};
use fluent_templates::{fluent_bundle::FluentValue, Loader};
use lettre::message::{header::ContentType, Attachment, Mailbox, SinglePart};
use mail_worker_protocol as protocol;
use protocol::v1::RegisteredEventInvite;
use std::collections::HashMap;

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
        context.insert("support", &builder.support_contact);

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
        context.insert("support", &builder.support_contact);

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
        let mbox = Mailbox::new(
            Some(generate_mailbox_name(
                &self.invitee.title,
                &self.invitee.first_name,
                &self.invitee.last_name,
            )),
            self.invitee.email.as_ref().parse()?,
        );

        Ok(mbox)
    }

    fn generate_attachments(
        &self,
        builder: &crate::MailBuilder,
    ) -> anyhow::Result<Vec<SinglePart>> {
        let language = if !self.inviter.language.is_empty() {
            &self.inviter.language
        } else {
            &builder.default_language
        };

        let mut context = tera::Context::new();
        context.insert("meeting_link", &builder.create_join_link(&self.event));
        context.insert("language", &language);
        context.insert("event", &self.event);

        let description = builder.tera.render("ics_description.txt", &context)?;

        let name = format!("{} {}", &self.invitee.first_name, &self.invitee.last_name);
        let invitee = crate::ics::Invitee::WithName {
            email: self.invitee.email.as_ref(),
            name: &name,
        };

        let ics = create_ics_v1(&self.inviter, &self.event, invitee, &description)?;

        let mut attachments = vec![];

        if let Some(ics) = ics {
            let ics = Attachment::new("invite.ics".into())
                .body(ics, ContentType::parse("text/calendar").unwrap());

            attachments.push(ics);
        }

        Ok(attachments)
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
