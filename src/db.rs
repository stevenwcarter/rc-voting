use std::error::Error;

use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::Pool;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub fn get_pool() -> SqlitePool {
    use dotenvy::dotenv;
    use std::env;
    dotenv().ok();
    let url = env::var("DATABASE_URL").unwrap_or("db/db.sql".to_string());
    let mgr = ConnectionManager::<SqliteConnection>::new(url);
    r2d2::Pool::builder()
        .min_idle(Some(2))
        .max_size(40)
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
