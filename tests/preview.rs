// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::str::FromStr as _;

use insta::assert_snapshot;
use rstest::rstest;
use smtp_mailer::{
    preview::{OutputVariant, TemplateVariant},
    settings::Settings,
};
use types_common::users::Language;

fn snapshot_name(template: TemplateVariant, lang: &str, output: OutputVariant) -> String {
    format!("{:?}-{}-{:?}", template, lang, output)
}

#[rstest]
fn preview(
    #[values("de", "en")] lang: &str,
    #[values(OutputVariant::Html, OutputVariant::Plain)] output: OutputVariant,
    #[values(
        TemplateVariant::RegisteredInvite,
        TemplateVariant::RegisteredEventUpdate,
        TemplateVariant::RegisteredCancellation,
        TemplateVariant::RegisteredUninvite,
        TemplateVariant::UnregisteredInvite,
        TemplateVariant::UnregisteredCancellation,
        TemplateVariant::ExternalInvite,
        TemplateVariant::ExternalCancellation
    )]
    template: TemplateVariant,
) {
    let settings = Settings::default();
    let lang = Language::from_str(lang).unwrap();

    let example_mail = template
        .render_template(&settings, output, lang.clone())
        .expect("Rendering must succeed");

    assert_snapshot!(snapshot_name(template, lang.as_str(), output), example_mail);
}
