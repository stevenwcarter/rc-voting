use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use r2d2::Pool;
#[cfg(feature = "ssr")]
pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

#[cfg(feature = "ssr")]
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