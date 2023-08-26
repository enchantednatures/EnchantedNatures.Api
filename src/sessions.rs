use crate::auth::User;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SessionManager {
    session: HashMap<String, User>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            session: HashMap::new(),
        }
    }

    pub(crate) fn insert(&mut self, p0: &str, user: &User) -> Result<()> {
        self.session.insert(p0.into(), user.clone());
        Ok(())
    }
    pub(crate) async fn load_session(&mut self, p0: String) -> Result<Option<&User>> {
        Ok(self.session.get(&p0))
    }
}
