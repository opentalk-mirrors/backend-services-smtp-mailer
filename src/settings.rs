// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use config::{Config, ConfigError, Environment, File, FileFormat};
use percent_encoding::percent_decode_str;
use serde::{de, Deserialize, Deserializer, Serialize};
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

    pub support_contact: Option<SupportContact>,
}

impl Settings {
    /// Creates a new Settings instance from the provided TOML file.
    /// Specific fields can be set or overwritten with environment variables (See struct level docs for more details).
    pub fn load(file_name: &str) -> Result<Settings, ConfigError> {
        let settings = Config::builder()
            .add_source(File::new(file_name, FileFormat::Toml))
            .add_source(
                Environment::with_prefix("MAILER")
                    .prefix_separator("_")
                    .separator("__"),
            )
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
/// SMTP Cleartext: smtp://user:pass@mailserver.example.org:1234?disable_starttls=true
/// SMTP with StartTLS: smtp://user:pass@mailserver.example.org:1234
/// SMTP with implicit TLS: smtps://user:pass@mailserver.example.org:1234
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

fn smtp_uri_deserializer<'de, D>(d: D) -> Result<SmtpUri, D::Error>
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
        deserialize_with = "smtp_uri_deserializer",
        default = "smtp_default_server"
    )]
    pub smtp_server: SmtpUri,
    #[serde(default = "smtp_default_from_name")]
    pub from_name: String,
    #[serde(default = "smtp_default_from_email")]
    pub from_email: String,
}

impl Default for SmtpConfig {
    fn default() -> Self {
        Self {
            smtp_server: smtp_default_server(),
            from_name: smtp_default_from_name(),
            from_email: smtp_default_from_email(),
        }
    }
}

fn smtp_default_server() -> SmtpUri {
    SmtpUri::from_str("smtp://localhost:25").unwrap()
}

fn smtp_default_from_name() -> String {
    "OpenTalk".to_string()
}

fn smtp_default_from_email() -> String {
    "no-reply@example.org".to_string()
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
    #[serde(default = "template_default_external_invite")]
    external_invite: MailTemplate,
    //
    #[serde(default = "template_default_event_update")]
    registered_event_update: MailTemplate,
    #[serde(default = "template_default_unregistered_event_update")]
    unregistered_event_update: MailTemplate,
    #[serde(default = "template_default_external_event_update")]
    external_event_update: MailTemplate,
    //
    #[serde(default = "template_default_event_cancellation")]
    registered_event_cancellation: MailTemplate,
    #[serde(default = "template_default_unregistered_event_cancellation")]
    unregistered_event_cancellation: MailTemplate,
    #[serde(default = "template_default_external_event_cancellation")]
    external_event_cancellation: MailTemplate,
    //
    #[serde(default = "template_default_uninvite")]
    registered_uninvite: MailTemplate,
    #[serde(default = "template_default_unregistered_uninvite")]
    unregistered_uninvite: MailTemplate,
    #[serde(default = "template_default_external_uninvite")]
    external_uninvite: MailTemplate,
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
                "unregistered_invite.txt",
            ),
            (
                self.unregistered_invite.html.as_ref(),
                "unregistered_invite.html",
            ),
            (self.external_invite.txt.as_ref(), "external_invite.txt"),
            (self.external_invite.html.as_ref(), "external_invite.html"),
            //
            (
                self.registered_event_update.txt.as_ref(),
                "registered_event_update.txt",
            ),
            (
                self.registered_event_update.html.as_ref(),
                "registered_event_update.html",
            ),
            (
                self.unregistered_event_update.txt.as_ref(),
                "unregistered_event_update.txt",
            ),
            (
                self.unregistered_event_update.html.as_ref(),
                "unregistered_event_update.html",
            ),
            (
                self.external_event_update.txt.as_ref(),
                "external_event_update.txt",
            ),
            (
                self.external_event_update.html.as_ref(),
                "external_event_update.html",
            ),
            //
            (
                self.registered_event_cancellation.txt.as_ref(),
                "registered_event_cancellation.txt",
            ),
            (
                self.registered_event_cancellation.html.as_ref(),
                "registered_event_cancellation.html",
            ),
            (
                self.unregistered_event_cancellation.txt.as_ref(),
                "unregistered_event_cancellation.txt",
            ),
            (
                self.unregistered_event_cancellation.html.as_ref(),
                "unregistered_event_cancellation.html",
            ),
            (
                self.external_event_cancellation.txt.as_ref(),
                "external_event_cancellation.txt",
            ),
            (
                self.external_event_cancellation.html.as_ref(),
                "external_event_cancellation.html",
            ),
            //
            (
                self.registered_uninvite.txt.as_ref(),
                "registered_uninvite.txt",
            ),
            (
                self.registered_uninvite.html.as_ref(),
                "registered_uninvite.html",
            ),
            (
                self.unregistered_uninvite.txt.as_ref(),
                "unregistered_uninvite.txt",
            ),
            (
                self.unregistered_uninvite.html.as_ref(),
                "unregistered_uninvite.html",
            ),
            (self.external_uninvite.txt.as_ref(), "external_uninvite.txt"),
            (
                self.external_uninvite.html.as_ref(),
                "external_uninvite.html",
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
            external_invite: template_default_external_invite(),
            //
            registered_event_update: template_default_event_update(),
            unregistered_event_update: template_default_unregistered_event_update(),
            external_event_update: template_default_external_event_update(),
            //
            registered_event_cancellation: template_default_event_cancellation(),
            unregistered_event_cancellation: template_default_unregistered_event_cancellation(),
            external_event_cancellation: template_default_external_event_cancellation(),
            //
            registered_uninvite: template_default_uninvite(),
            unregistered_uninvite: template_default_unregistered_uninvite(),
            external_uninvite: template_default_external_uninvite(),
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

fn template_default_external_invite() -> MailTemplate {
    MailTemplate {
        txt: "resources/templates/external_invite.txt".into(),
        html: "resources/templates/external_invite.html".into(),
    }
}

fn template_default_event_update() -> MailTemplate {
    MailTemplate {
        txt: "resources/templates/registered_event_update.txt".into(),
        html: "resources/templates/registered_event_update.html".into(),
    }
}

fn template_default_unregistered_event_update() -> MailTemplate {
    MailTemplate {
        txt: "resources/templates/unregistered_event_update.txt".into(),
        html: "resources/templates/unregistered_event_update.html".into(),
    }
}

fn template_default_external_event_update() -> MailTemplate {
    MailTemplate {
        txt: "resources/templates/external_event_update.txt".into(),
        html: "resources/templates/external_event_update.html".into(),
    }
}

fn template_default_event_cancellation() -> MailTemplate {
    MailTemplate {
        txt: "resources/templates/registered_event_cancellation.txt".into(),
        html: "resources/templates/registered_event_cancellation.html".into(),
    }
}

fn template_default_unregistered_event_cancellation() -> MailTemplate {
    MailTemplate {
        txt: "resources/templates/unregistered_event_cancellation.txt".into(),
        html: "resources/templates/unregistered_event_cancellation.html".into(),
    }
}

fn template_default_external_event_cancellation() -> MailTemplate {
    MailTemplate {
        txt: "resources/templates/external_event_cancellation.txt".into(),
        html: "resources/templates/external_event_cancellation.html".into(),
    }
}

fn template_default_uninvite() -> MailTemplate {
    MailTemplate {
        txt: "resources/templates/registered_uninvite.txt".into(),
        html: "resources/templates/registered_uninvite.html".into(),
    }
}

fn template_default_unregistered_uninvite() -> MailTemplate {
    MailTemplate {
        txt: "resources/templates/unregistered_uninvite.txt".into(),
        html: "resources/templates/unregistered_uninvite.html".into(),
    }
}

fn template_default_external_uninvite() -> MailTemplate {
    MailTemplate {
        txt: "resources/templates/external_uninvite.txt".into(),
        html: "resources/templates/external_uninvite.html".into(),
    }
}

/// Language config
///
/// Currently only setting the default_language (a fallback) is supported
#[derive(Debug, Clone, Deserialize)]
pub struct Frontend {
    #[serde(default = "frontend_default_base_url")]
    pub base_url: String,
    #[serde(default = "frontend_default_data_protection_url")]
    pub data_protection_url: String,
}

impl Default for Frontend {
    fn default() -> Self {
        Self {
            base_url: frontend_default_base_url(),
            data_protection_url: frontend_default_data_protection_url(),
        }
    }
}

fn frontend_default_base_url() -> String {
    "https://opentalk.example.org".into()
}

fn frontend_default_data_protection_url() -> String {
    "https://opentalk.example.org/dataprotection".into()
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
    "{base_url}/dashboard/meetings/{event_id}".into()
}

fn email_default_join_link_builder() -> String {
    "{base_url}/room/{room_id}".into()
}

fn email_default_guest_link_builder() -> String {
    "{base_url}/invite/{invite_id}".into()
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportContact {
    pub phone: String,
    pub mail: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    #[test]
    fn settings_env_vars_overwite_config() -> Result<(), ConfigError> {
        // Sanity check
        let settings = Settings::load("./extra/example.toml")?;
        let support_contact = settings.support_contact.unwrap();

        assert_eq!(support_contact.phone, "+49123321123".to_string());
        assert_eq!(support_contact.mail, "support@example.com".to_string());

        // Set environment variables to overwrite default config file
        let env_support_phone = "+49777666555".to_string();
        let env_support_mail = "support@newexample.com".to_string();
        env::set_var("MAILER_SUPPORT_CONTACT__PHONE", &env_support_phone);
        env::set_var("MAILER_SUPPORT_CONTACT__MAIL", &env_support_mail);

        let settings = Settings::load("./extra/example.toml")?;
        let support_contact = settings.support_contact.unwrap();

        assert_eq!(support_contact.phone, env_support_phone);
        assert_eq!(support_contact.mail, env_support_mail);

        Ok(())
    }
}
