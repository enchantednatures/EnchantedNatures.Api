use anyhow::Result;
use serde::{Deserialize, Serialize};



struct Settings;

impl Settings {
    fn load_config() -> Result<Self> {
        let base_path = std::env::current_dir()?;
        let configuration_directory = base_path.join("configuration");

        let environment: Environment = std::env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .unwrap_or(Environment::Local);

        let environment_filename = format!("{}.yaml", &environment.as_str());
        let _settings = config::Config::builder()
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

        Ok(Self {})
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationSettings {
    pub addr: [u8; 4],
    pub port: u16,
}

impl ApplicationSettings {
    fn new(addr: [u8; 4], port: u16) -> Self {
        Self { addr, port }
    }
}
#[derive(Debug)]
pub struct AuthSettings {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub revocation_url: String,
    pub introspection_url: String,
    pub auth_url: String,
    pub token_url: String,
}

impl Default for AuthSettings {
    fn default() -> Self {
        Self {
            client_id: std::env::var("CLIENT_ID").expect("CLIENT_ID must be set"),
            client_secret: std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set"),
            redirect_url: std::env::var("REDIRECT_URL").expect("REDIRECT_URL must be set"),
            auth_url: std::env::var("AUTH_URL").expect("AUTH_URL must be set"),
            token_url: std::env::var("TOKEN_URL").expect("TOKEN_URL must be set"),
            revocation_url: std::env::var("REVOCATION_URL").expect("REVOCATION_URL must be set"),
            introspection_url: std::env::var("INTROSPECTION_URL")
                .expect("INTROSPECTION_URL must be set"),
        }
    }
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self::new([0, 0, 0, 0], 6969)
    }
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
