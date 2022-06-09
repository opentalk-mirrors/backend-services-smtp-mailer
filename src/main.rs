use clap::{ArgEnum, Parser, Subcommand};
use smtp_mailer::{preview, run, settings};
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(author, about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
#[clap(global_setting(clap::AppSettings::NoAutoVersion))]
pub(crate) struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value = "config.toml")]
    config: String,

    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(long)]
    version: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Return a preview of an email
    #[clap()]
    Preview {
        /// Output type
        #[clap(arg_enum)]
        type_: OutputVariant,
        /// Template to preview
        #[clap(arg_enum)]
        template: TemplateVariate,
        /// Language Code
        #[clap()]
        language: String,
    },
}

#[derive(Debug, Clone, Copy, ArgEnum)]
enum OutputVariant {
    Html,
    Plain,
}

impl From<&OutputVariant> for bool {
    fn from(val: &OutputVariant) -> Self {
        match val {
            OutputVariant::Html => true,
            OutputVariant::Plain => false,
        }
    }
}

#[derive(Debug, Clone, ArgEnum)]
enum TemplateVariate {
    RegisteredInvite,
    UnregisteredInvite,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.version {
        print_version_info();
        exit(0);
    }

    env_logger::init();

    let settings = settings::Settings::load(&args.config)?;

    if let Some(Commands::Preview {
        type_,
        language,
        template,
    }) = &args.command
    {
        match template {
            TemplateVariate::RegisteredInvite => {
                println!(
                    "{}",
                    preview::preview_registered_invite(&settings, type_.into(), language)
                );
            }
            TemplateVariate::UnregisteredInvite => {
                println!(
                    "{}",
                    preview::preview_unregistered_invite(&settings, type_.into(), language)
                );
            }
        }

        exit(0);
    }

    run(settings).await?;
    Ok(())
}

const BUILD_INFO: [(&str, Option<&str>); 10] = [
    ("Build Timestamp", option_env!("VERGEN_BUILD_TIMESTAMP")),
    ("Build Version", option_env!("VERGEN_BUILD_SEMVER")),
    ("Commit SHA", option_env!("VERGEN_GIT_SHA")),
    ("Commit Date", option_env!("VERGEN_GIT_COMMIT_TIMESTAMP")),
    ("Commit Branch", option_env!("VERGEN_GIT_BRANCH")),
    ("rustc Version", option_env!("VERGEN_RUSTC_SEMVER")),
    ("rustc Channel", option_env!("VERGEN_RUSTC_CHANNEL")),
    ("rustc Host Triple", option_env!("VERGEN_RUSTC_HOST_TRIPLE")),
    (
        "cargo Target Triple",
        option_env!("VERGEN_CARGO_TARGET_TRIPLE"),
    ),
    ("cargo Profile", option_env!("VERGEN_CARGO_PROFILE")),
];

fn print_version_info() {
    for (label, value) in BUILD_INFO {
        println!("{label}: {value}", value = value.unwrap_or("N/A"));
    }
}
