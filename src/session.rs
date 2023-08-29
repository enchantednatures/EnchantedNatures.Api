use anyhow::Result;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Serialize, Deserialize)]
struct Session;

#[derive(Debug)]
struct SessionManager {
    client: redis::Client,
}

impl SessionManager {

    fn new(client: redis::Client) -> Self {
        Self { client }
    }

    fn create_session(self, &sesh: Session) -> Result<()> {
        let mut con = self.client.get_connection()?;
        let _: () = con.set("FOO", "42")?;
        Ok(())
    }
}
