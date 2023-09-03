use anyhow::Result;
use async_session::Session;
use redis::AsyncCommands;

#[derive(Debug, Clone)]
pub struct SessionManager {
    redis: redis::Client,
}

impl SessionManager {
    pub fn new(redis: redis::Client) -> Self {
        Self { redis }
    }

    pub(crate) async fn get_session<'a>(&self, session_id: &'a str) -> Result<Option<Session>> {
        let mut con = self.redis.get_async_connection().await?;
        let session: String = con.get(session_id).await?;
        let session: Session = serde_json::from_str(&session)?;
        Ok(Some(session))
    }

    pub(crate) async fn set_session(&self, session: &Session) -> Result<String> {
        let mut con = self.redis.get_async_connection().await?;
        con
            .set(session.id(), serde_json::to_string(session)?)
            .await?;
        Ok(session.id().to_string())
    }
}
