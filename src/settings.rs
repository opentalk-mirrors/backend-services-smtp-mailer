use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use config::{Config, ConfigError, Environment, File, FileFormat};
use percent_encoding::percent_decode_str;
use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub rabbit_mq: RabbitMqConfig,

    pub smtp: SmtpConfig,

    // #[serde(default)]
    pub templates: Templates,
    pub template_builder: TemplateBuilder,
    pub languages: Languages,
}

impl Settings {
    /// Creates a new Settings instance from the provided TOML file.
    /// Specific fields can be set or overwritten with environment variables (See struct level docs for more details).
    pub fn load(file_name: &str) -> Result<Settings, ConfigError> {
        let settings = Config::builder()
            .add_source(File::new(file_name, FileFormat::Toml))
            .add_source(Environment::with_prefix("MAILER").separator("__"))
            .build()?
            .try_deserialize()?;

        Ok(settings)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RabbitMqConfig {
    #[serde(default = "rabbitmq_default_url")]
    pub url: String,
    #[serde(default = "rabbitmq_default_queue_name")]
    pub queue_name: String,
}

impl Default for RabbitMqConfig {
    fn default() -> Self {
        Self {
            url: rabbitmq_default_url(),
            queue_name: rabbitmq_default_queue_name(),
        }
    }
}

fn rabbitmq_default_url() -> String {
    "amqp://guest:guest@localhost:5672".to_owned()
}

fn rabbitmq_default_queue_name() -> String {
    "opentalk_mailer".to_owned()
}

#[derive(Debug, Clone, Deserialize)]
pub enum SmtpSecurity {
    StartTls,
    Tls,
}

/// SMTP Cleartext: smtp://foo:bar@my-mailserver:1234/?disable_starttls=true
/// SMTP with StartTLS: smtp://foo:bar@my-mailserver:1234/
/// SMTP with implicit TLS: smtps://foo:bar@my-mailserver:1234/
#[derive(Debug, Clone)]
pub struct SmtpUri(url::Url);
impl FromStr for SmtpUri {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match url::Url::parse(s) {
            Ok(res) => match res.scheme() {
                "smtp" | "smtps" => Ok(SmtpUri(res)),
                _ => Err(anyhow::anyhow!("Invalid smtp scheme")),
            },
            Err(_) => Err(anyhow::anyhow!("Invalid smtp scheme")),
        }
    }
}

impl SmtpUri {
    pub fn disable_starttls(&self) -> bool {
        self.0
            .query_pairs()
            .any(|x| x.0 == "disable_starttls" && x.1 == "true")
    }

    pub fn port(&self) -> Option<u16> {
        self.0.port()
    }

    pub fn host(&self) -> Option<&str> {
        self.0.host_str()
    }

    pub fn scheme(&self) -> &str {
        self.0.scheme()
    }

    pub fn credentials(&self) -> anyhow::Result<(Option<String>, Option<String>)> {
        let username = self.0.username();
        let res = if username.len() == 0 {
            (
                None,
                self.0
                    .password()
                    .map(percent_decode_str)
                    .map(|x| x.decode_utf8())
                    .transpose()?
                    .map(|x| x.into_owned()),
            )
        } else {
            (
                Some(percent_decode_str(username).decode_utf8()?.into_owned()),
                self.0
                    .password()
                    .map(percent_decode_str)
                    .map(|x| x.decode_utf8())
                    .transpose()?
                    .map(|x| x.into_owned()),
            )
        };

        Ok(res)
    }
}

fn smtp_uri_desirializer<'de, D>(d: D) -> Result<SmtpUri, D::Error>
where
    D: Deserializer<'de>,
{
    let string: String = Deserialize::deserialize(d)?;
    SmtpUri::from_str(&string).map_err(de::Error::custom)
}

#[derive(Debug, Clone, Deserialize)]
pub struct SmtpConfig {
    #[serde(deserialize_with = "smtp_uri_desirializer")]
    pub smtp_server: SmtpUri,
}

#[derive(Debug, Clone, Deserialize)]
struct MailTemplate {
    txt: PathBuf,
    html: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Templates {
    #[serde(default = "template_default_invite")]
    invite: MailTemplate,
}

impl Templates {
    pub fn iter(&self) -> impl Iterator<Item = (&Path, &str)> {
        vec![
            (self.invite.txt.as_ref(), "invite.txt"),
            (self.invite.html.as_ref(), "invite.html"),
        ]
        .into_iter()
    }
}

impl Default for Templates {
    fn default() -> Self {
        Self {
            invite: template_default_invite(),
        }
    }
}

fn template_default_invite() -> MailTemplate {
    MailTemplate {
        txt: "resources/templates/invite.txt".into(),
        html: "resources/templates/invite.html".into(),
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateBuilder {
    #[serde(default = "email_default_dashboard_event_link_builder")]
    pub dashboard_event_link_builder: String,
    /// {invite_id} will be replaced
    #[serde(default = "email_default_join_link_builder")]
    pub join_link_builder: String,
    /// {invite_id} will be replaced
    #[serde(default = "email_default_guest_link_builder")]
    pub guest_link_builder: String,
}

impl Default for TemplateBuilder {
    fn default() -> Self {
        Self {
            dashboard_event_link_builder: email_default_dashboard_event_link_builder(),
            join_link_builder: email_default_join_link_builder(),
            guest_link_builder: email_default_guest_link_builder(),
        }
    }
}

fn email_default_dashboard_event_link_builder() -> String {
    "https://opentalk.example.org/dashboard/{event_id}".into()
}
fn email_default_join_link_builder() -> String {
    "https://opentalk.example.org/lobby/{room_id}".into()
}

fn email_default_guest_link_builder() -> String {
    "https://opentalk.example.org/invite/{invite_id}".into()
}

#[derive(Debug, Clone, Deserialize)]
pub struct Languages {
    #[serde(default = "languages_default_default_language")]
    pub default_language: String,
}

impl Default for Languages {
    fn default() -> Self {
        Self {
            default_language: languages_default_default_language(),
        }
    }
}

fn languages_default_default_language() -> String {
    "en-US".into()
}
