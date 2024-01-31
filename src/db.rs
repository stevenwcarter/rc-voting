use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use r2d2::Pool;
use std::env;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub fn get_pool() -> SqlitePool {
    dotenv().ok();
    let url = env::var("DATABASE_URL").unwrap_or("db/db.sql".to_string());
    let mgr = ConnectionManager::<SqliteConnection>::new(url);
    r2d2::Pool::builder()
        .min_idle(Some(2))
        .max_size(40)
        .build(mgr)
        .expect("could not build connection pool")
}
