use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Debug, Deserialize)]
pub struct AuthSettings {
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) redirect_url: String,
    pub(crate) token_url: String,
    pub(crate) auth_url: String,
    pub(crate) introspection_url: String,
    pub(crate) revocation_url: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub application_name: String,
}

impl From<DatabaseSettings> for PgConnectOptions {
    fn from(value: DatabaseSettings) -> Self {
        PgConnectOptions::new()
            .host(value.host.as_ref())
            .port(value.port)
            .database(value.database.as_ref())
            .username(value.user.as_ref())
            .application_name(value.application_name.as_ref())
            .ssl_mode(PgSslMode::VerifyCa)
            .ssl_root_cert("./certs/ca.crt")
            .ssl_client_key("./certs/client.key")
            .ssl_client_cert("./certs/client.crt")
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_settings: DatabaseSettings,
    // pub aws_endpoint_url: String,
    // pub aws_region: String,
    // pub aws_bucket_name: String,
    pub auth_settings: AuthSettings,
    pub app_settings: ApplicationSettings,
    pub redis_url: String,
}

impl Settings {
    pub fn load_config() -> Result<Self> {
        let base_path = std::env::current_dir()?;
        let configuration_directory = base_path.join("config");

        let environment: Environment = std::env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .unwrap_or(Environment::Local);

        let environment_filename = format!("{}.yaml", &environment.as_str());
        let settings = config::Config::builder()
            .add_source(config::File::from(
                configuration_directory.join("base.yaml"),
            ))
            .add_source(config::File::from(
                configuration_directory.join(environment_filename),
            ))
            // Add in settings from environment variables (with a prefix of APP and '__' as separator)
            // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;

        Ok(settings.try_deserialize::<Self>()?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationSettings {
    pub addr: [u8; 4],
    pub port: u16,
}

pub enum Environment {
    Development,
    Local,
    Staging,
    Production,
}

impl Environment {
    fn as_str(&self) -> &str {
        match self {
            Environment::Development => "dev",
            Environment::Local => "local",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
}

impl From<&Environment> for String {
    fn from(val: &Environment) -> Self {
        match &val {
            Environment::Development => "development".into(),
            Environment::Local => "local".into(),
            Environment::Staging => "staging".into(),
            Environment::Production => "production".into(),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "local" => Ok(Environment::Local),
            "development" => Ok(Environment::Development),
            "staging" => Ok(Environment::Staging),
            "production" => Ok(Environment::Production),
            _ => Err(format!("Failed to parse{}", value)),
        }
    }
}
