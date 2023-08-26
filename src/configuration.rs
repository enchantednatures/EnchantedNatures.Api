use std::net::SocketAddr;

#[derive(Debug)]
pub struct DatabaseSettings {
    pub url: String,
}

impl DatabaseSettings {
    fn new(url: String) -> Self {
        Self { url }
    }
}

#[derive(Debug)]
pub struct ApplicationSettings {
    pub addr: String,
    pub port: usize,
}

impl ApplicationSettings {
    fn new(addr: String, port: usize) -> Self {
        Self { addr, port }
    }
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self {
            addr: "localhost".to_string(),
            port: 6969,
        }
    }
}

#[derive(Debug)]
struct Configuration {
    database_settings: DatabaseSettings,
    app_settings: ApplicationSettings,
}

impl Configuration {
    fn new(database_settings: DatabaseSettings, app_settings: ApplicationSettings) -> Self {
        Self {
            database_settings,
            app_settings,
        }
    }
}
