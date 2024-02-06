use crate::models::Session;

use super::db::SqlitePool;

#[derive(Clone)]
#[cfg(feature = "ssr")]
pub struct GraphQLContext {
    pub pool: SqlitePool,
    pub session: Option<Session>,
}

impl GraphQLContext {
    pub fn attach_session(&self, session: &Session) -> Self {
        Self {
            session: Some(session.clone()),
            pool: self.pool.clone(),
        }
    }
}

#[cfg(feature = "ssr")]
impl juniper::Context for GraphQLContext {}
