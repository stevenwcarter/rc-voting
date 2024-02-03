use super::db::SqlitePool;

#[derive(Clone)]
#[cfg(feature = "ssr")]
pub struct GraphQLContext {
    pub pool: SqlitePool,
}

#[cfg(feature = "ssr")]
impl juniper::Context for GraphQLContext {}
