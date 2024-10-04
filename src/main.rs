// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::{fmt::Write, process::exit};

use clap::{Parser, Subcommand};
use smtp_mailer::{
    preview::{preview_send_mail, OutputVariant, TemplateVariant},
    run, settings,
};

#[derive(Parser, Debug)]
#[clap(author, about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
pub(crate) struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value = "config.toml")]
    config: String,

    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(long, short('V'))]
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
        language: String,
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.version {
        println!("{}", version_info());

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
        let example_mail = template.render_template(&settings, *type_, language)?;
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

fn profile_to_human(profile: &str) -> &str {
    // Copied from https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level
    match profile {
        "0" => "0, no optimizations",
        "1" => "1, basic optimizations",
        "2" => "2, some optimizations",
        "3" => "3, all optimizations",
        "s" => "'s', optimize for binary size",
        "z" => "'z', optimize for binary size, but also turn off loop vectorization.",
        profile => profile,
    }
}

fn build_info() -> [(&'static str, Option<&'static str>); 10] {
    [
        ("Build Timestamp", Some(env!("VERGEN_BUILD_TIMESTAMP"))),
        ("Build Version", Some(env!("CARGO_PKG_VERSION"))),
        ("Commit SHA", option_env!("VERGEN_GIT_SHA")),
        ("Commit Date", option_env!("VERGEN_GIT_COMMIT_TIMESTAMP")),
        ("Commit Branch", option_env!("VERGEN_GIT_BRANCH")),
        ("rustc Version", Some(env!("VERGEN_RUSTC_SEMVER"))),
        ("rustc Channel", Some(env!("VERGEN_RUSTC_CHANNEL"))),
        ("rustc Host Triple", Some(env!("VERGEN_RUSTC_HOST_TRIPLE"))),
        (
            "cargo Target Triple",
            Some(env!("VERGEN_CARGO_TARGET_TRIPLE")),
        ),
        (
            "cargo Profile",
            Some(env!("VERGEN_CARGO_OPT_LEVEL")).map(profile_to_human),
        ),
    ]
}

fn version_info() -> String {
    let mut version_message = String::new();
    if let Some(bin_name) = option_env!("CARGO_BIN_NAME") {
        writeln!(version_message, "{}:", bin_name).expect("Writing to string buffer must succeed");
    }
    for (label, value) in build_info() {
        writeln!(
            version_message,
            "{label}: {value}",
            value = value.unwrap_or("N/A")
        )
        .expect("Writing to string buffer must succeed");
    }
    version_message
}
