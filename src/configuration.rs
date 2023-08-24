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
    addr: String,
    port: usize,
}

impl ApplicationSettings {
    fn new(addr: String, port: usize) -> Self {
        Self { addr, port }
    }
}
#[derive(Debug)]
pub struct AuthSettings {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub auth_url: String,
    pub token_url: String,
}

impl AuthSettings {
    pub fn new() -> Self {
        Self {
            client_id: std::env::var("CLIENT_ID").expect("CLIENT_ID must be set"),
            client_secret: std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set"),
            redirect_url: std::env::var("REDIRECT_URL").expect("REDIRECT_URL must be set"),
            auth_url: std::env::var("AUTH_URL").expect("AUTH_URL must be set"),
            token_url: std::env::var("TOKEN_URL").expect("TOKEN_URL must be set"),
        }
    }
}

#[derive(Debug)]
struct Configuration {
    database_settings: DatabaseSettings,
    app_settings: ApplicationSettings,
    auth_settings: AuthSettings,
}

impl Configuration {
    fn new(database_settings: DatabaseSettings, app_settings: ApplicationSettings) -> Self {
        Self {
            database_settings,
            app_settings,
            auth_settings: AuthSettings::new(),
        }
    }
}
