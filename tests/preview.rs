// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use insta::assert_snapshot;
use rstest::rstest;
use smtp_mailer::{
    preview::{OutputVariant, TemplateVariant},
    settings::Settings,
};

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

    let example_mail = template
        .render_template(&settings, output, lang)
        .expect("Rendering must succeed");

    assert_snapshot!(snapshot_name(template, lang, output), example_mail);
}
