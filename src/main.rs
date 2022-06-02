use clap::{ArgEnum, Parser, Subcommand};
use smtp_mailer::{preview, run, settings};
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(author, version, about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
pub(crate) struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value = "config.toml")]
    config: String,

    #[clap(subcommand)]
    command: Option<Commands>,
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
    Invite,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    env_logger::init();

    let settings = settings::Settings::load(&args.config)?;

    if let Some(Commands::Preview {
        type_,
        language,
        template,
    }) = &args.command
    {
        match template {
            TemplateVariate::Invite => {
                println!(
                    "{}",
                    preview::preview_invite(&settings, type_.into(), language)
                );
            }
        }

        exit(0);
    }

    run(settings).await?;
    Ok(())
}
