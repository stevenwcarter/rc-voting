use std::error::Error;

use diesel::dsl::sql_query;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::CustomizeConnection;
use diesel::r2d2::Pool;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::time::Duration;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
pub struct ConnectionOptions {
    pub busy_timeout: Option<Duration>,
}

impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error>
    for ConnectionOptions
{
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        (|| {
            if let Some(d) = self.busy_timeout {
                sql_query(&format!("PRAGMA busy_timeout = {};", d.as_millis())).execute(conn)?;
            }
            Ok(())
        })()
        .map_err(diesel::r2d2::Error::QueryError)
    }
}

pub fn get_pool() -> SqlitePool {
    use dotenvy::dotenv;
    use std::env;
    dotenv().ok();
    let url = env::var("DATABASE_URL").unwrap_or("db/db.sql".to_string());
    let mgr = ConnectionManager::<SqliteConnection>::new(url);
    r2d2::Pool::builder()
        .min_idle(Some(3))
        .max_size(20)
        .connection_customizer(Box::new(ConnectionOptions {
            busy_timeout: Some(Duration::from_secs(2)),
        }))
        .build(mgr)
        .expect("could not build connection pool")
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
pub fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::sqlite::Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
