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
struct ApplicationSettings {
    addr: String,
    port: usize,
}

impl ApplicationSettings {
    fn new(addr: String, port: usize) -> Self {
        Self { addr, port }
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
