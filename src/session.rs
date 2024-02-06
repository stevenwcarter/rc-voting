#![allow(non_snake_case)]

use crate::{context::GraphQLContext, models::Session, schema::sessions};

use anyhow::{Context, Result};
use diesel::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SessionSvc {}

impl SessionSvc {
    pub fn list_sessions_for_user(
        context: &GraphQLContext,
        user_uuid: &str,
    ) -> Result<Vec<Session>, anyhow::Error> {
        let mut conn = context.pool.get().expect("Could not get DB conn");

        let sessions: Vec<Session> = sessions::table
            .filter(sessions::user_uuid.like(user_uuid))
            .load::<Session>(&mut conn)
            .context("Could not list sessions for user")?;

        Ok(sessions)
    }

    pub fn get_session(context: &GraphQLContext, session_uuid: &str) -> Result<Session> {
        let mut conn = context.pool.get().expect("Could not get DB conn");

        let session: Session = sessions::table
            .filter(sessions::uuid.like(session_uuid))
            .first::<Session>(&mut conn)
            .context("Could not find sessions for user")?;

        Ok(session)
    }

    pub fn clean_expired(context: &GraphQLContext) {
        let mut conn = context.pool.get().expect("Could not get DB conn");
        use super::schema::sessions::dsl::*;

        let start = SystemTime::now();
        let now = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let now = now.as_millis() as i64;

        let _ = diesel::delete(sessions.filter(expires.lt(now)))
            .execute(&mut conn)
            .context("Could not delete old records");
    }
}
