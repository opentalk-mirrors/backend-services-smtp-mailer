// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use super::{create_ics_attachments, generate_mailbox_name, MailTemplate};
use crate::{
    i18n,
    ics::{create_ics_v1, EventStatus},
};
use fluent_templates::{fluent_bundle::FluentValue, Loader};
use lettre::message::{Mailbox, SinglePart};
use mail_worker_protocol as protocol;
use protocol::v1::UnregisteredEventInvite;
use std::collections::HashMap;

fn language(obj: &UnregisteredEventInvite) -> &String {
    &obj.inviter.language
}

fn build_template_context(
    obj: &UnregisteredEventInvite,
    builder: &super::MailBuilder,
) -> tera::Context {
    let mut context = tera::Context::new();

    let language = language(obj);
    let language = if !language.is_empty() {
        language
    } else {
        &builder.default_language
    };
    context.insert("language", &language);
    context.insert("invitee", &obj.invitee);
    context.insert("inviter", &obj.inviter);
    context.insert("event", &obj.event);
    context.insert("join_link", &builder.create_join_link(&obj.event));
    context.insert(
        "event_link",
        &builder.create_dashboard_event_link(&obj.event),
    );
    context.insert("support", &builder.support_contact);
    context.insert("data_protection_url", &builder.frontend.data_protection_url);
    context
}

impl MailTemplate for UnregisteredEventInvite {
    fn generate_email_plain(&self, builder: &super::MailBuilder) -> anyhow::Result<String> {
        let context = build_template_context(self, builder);

        builder
            .tera
            .render("unregistered_invite.txt", &context)
            .map_err(Into::into)
    }

    fn generate_email_html(&self, builder: &super::MailBuilder) -> anyhow::Result<String> {
        let context = build_template_context(self, builder);

        let html = builder.tera.render("unregistered_invite.html", &context)?;

        let inliner = css_inline::CSSInliner::options().build();
        inliner.inline(&html).map_err(Into::into)
    }

    fn generate_subject(&self, builder: &super::MailBuilder) -> anyhow::Result<String> {
        let subject_args = subject_args(&self.event, &self.invitee, &self.inviter);

        let language = language(self);
        let lang = if !language.is_empty() {
            language.parse()?
        } else {
            builder.default_language.parse()?
        };

        Ok(i18n::LOCALES.lookup_complete(
            &lang,
            "unregistered-event-invite-subject",
            Some(&subject_args),
        ))
    }

    fn generate_reply_to_mbox(
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
                "",
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
        context.insert("data_protection_url", &builder.frontend.data_protection_url);

        let description = builder.tera.render("ics_description.txt", &context)?;

        let name = format!("{} {}", &self.invitee.first_name, &self.invitee.last_name);
        let invitee = crate::ics::Invitee::WithName {
            email: self.invitee.email.as_ref(),
            name: &name,
        };

        let ics = create_ics_v1(
            &self.inviter,
            &self.event,
            invitee,
            &description,
            EventStatus::Created,
        )?;

        if let Some(ics) = ics {
            return Ok(create_ics_attachments(ics, EventStatus::Created));
        }

        Ok(vec![])
    }
}

fn subject_args(
    event: &protocol::v1::Event,
    invitee: &protocol::v1::UnregisteredUser,
    inviter: &protocol::v1::RegisteredUser,
) -> HashMap<String, FluentValue<'static>> {
    super::unregistered_invitee_subject_args(event, invitee, inviter)
}
