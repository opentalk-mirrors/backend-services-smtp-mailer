// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::process::exit;

use clap::{Parser, Subcommand};
use smtp_mailer::{
    preview::{preview_send_mail, OutputVariant, TemplateVariant},
    run, settings,
};
use types_common::users::Language;

#[derive(Parser, Debug)]
#[clap(author, about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
pub(crate) struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value = "config.toml")]
    config: String,

    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(long, short('V'), help = "Print version information")]
    version: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Return a preview of an email
    Preview {
        /// Output type
        #[clap(value_enum)]
        type_: OutputVariant,
        /// Template to preview
        #[clap(value_enum)]
        template: TemplateVariant,
        /// Language Code
        #[clap()]
        language: Language,
    },
    PreviewSend {
        /// Template to preview
        #[clap(value_enum)]
        template: TemplateVariant,
        /// To Email
        to: String,
        /// Delay between invite and cancellation in seconds
        #[clap(default_value_t = 15)]
        cancellation_delay: u64,
    },
}

opentalk_version::build_info!();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.version {
        println!("{}", build_info::BuildInfo::new());

        return Ok(());
    }

    env_logger::init();

    let settings = settings::Settings::load(&args.config)?;

    if let Some(Commands::Preview {
        type_,
        language,
        template,
    }) = &args.command
    {
        let example_mail = template.render_template(&settings, *type_, language.clone())?;
        print!("{}", example_mail);

        exit(0);
    } else if let Some(Commands::PreviewSend {
        template,
        to,
        cancellation_delay,
    }) = &args.command
    {
        preview_send_mail(&settings, *template, to.clone(), *cancellation_delay).await?;

        exit(0);
    }

    run(settings).await?;
    Ok(())
}
