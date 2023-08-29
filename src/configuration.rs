use serde::{Deserialize, Serialize};

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
