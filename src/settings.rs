use config::{Config, ConfigError, Environment, File, FileFormat};
use percent_encoding::percent_decode_str;
use serde::{de, Deserialize, Deserializer};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub rabbit_mq: RabbitMqConfig,

    #[serde(default)]
    pub smtp: SmtpConfig,

    #[serde(default)]
    pub templates: Templates,

    #[serde(default)]
    pub template_builder: TemplateBuilder,

    #[serde(default)]
    pub frontend: Frontend,

    #[serde(default)]
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

/// RabbitMQ Config containing the url to connect to
/// and the queue_name for incoming mail tasks
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

/// A SMTP URI type
///
/// # Examples
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
        let res = if username.is_empty() {
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

/// SMTP Config
///
/// Configure with which SMTP server mails should be send.
#[derive(Debug, Clone, Deserialize)]
pub struct SmtpConfig {
    #[serde(
        deserialize_with = "smtp_uri_desirializer",
        default = "smtp_default_server"
    )]
    pub smtp_server: SmtpUri,
}

impl Default for SmtpConfig {
    fn default() -> Self {
        Self {
            smtp_server: smtp_default_server(),
        }
    }
}

fn smtp_default_server() -> SmtpUri {
    SmtpUri::from_str("smtp://localhost:25").unwrap()
}
/// MailTemplate Config
///
/// Paths to a set of txt and html templates for a specific mail task
#[derive(Debug, Clone, Deserialize)]
struct MailTemplate {
    txt: PathBuf,
    html: PathBuf,
}

/// Mail Templates config
///
/// Uses MailTemplate to point to sets of templates for specific mail tasks
#[derive(Debug, Clone, Deserialize)]
pub struct Templates {
    #[serde(default = "template_default_registered_invite")]
    registered_invite: MailTemplate,
    #[serde(default = "template_default_unregistered_invite")]
    unregistered_invite: MailTemplate,
}

impl Templates {
    pub fn iter(&self) -> impl Iterator<Item = (&Path, &str)> {
        vec![
            (self.registered_invite.txt.as_ref(), "registered_invite.txt"),
            (
                self.registered_invite.html.as_ref(),
                "registered_invite.html",
            ),
            (
                self.unregistered_invite.txt.as_ref(),
                "registered_invite.txt",
            ),
            (
                self.unregistered_invite.html.as_ref(),
                "registered_invite.html",
            ),
        ]
        .into_iter()
    }
}

impl Default for Templates {
    fn default() -> Self {
        Self {
            registered_invite: template_default_registered_invite(),
            unregistered_invite: template_default_unregistered_invite(),
        }
    }
}

fn template_default_registered_invite() -> MailTemplate {
    MailTemplate {
        txt: "resources/templates/registered_invite.txt".into(),
        html: "resources/templates/registered_invite.html".into(),
    }
}

fn template_default_unregistered_invite() -> MailTemplate {
    MailTemplate {
        txt: "resources/templates/unregistered_invite.txt".into(),
        html: "resources/templates/unregistered_invite.html".into(),
    }
}

/// Language config
///
/// Currently only setting the default_language (a fallback) is suported
#[derive(Debug, Clone, Deserialize)]
pub struct Frontend {
    #[serde(default = "frontend_default_base_url")]
    pub base_url: String,
}

impl Default for Frontend {
    fn default() -> Self {
        Self {
            base_url: frontend_default_base_url(),
        }
    }
}

fn frontend_default_base_url() -> String {
    "https://opentalk.example.org/".into()
}

/// Template Builder Config
///
/// Some information is currently not present in the mail tasks, e.g. baseURLs
/// This can be configured here.
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
    "https://{base_url}dashboard/{event_id}".into()
}
fn email_default_join_link_builder() -> String {
    "https://{base_url}lobby/{room_id}".into()
}

fn email_default_guest_link_builder() -> String {
    "https://{base_url}invite/{invite_id}".into()
}

/// Language config
///
/// Currently only setting the default_language (a fallback) is suported
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
