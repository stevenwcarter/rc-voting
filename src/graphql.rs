use crate::context::GraphQLContext;

use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};

pub struct Query;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Query {
    #[graphql(name = "echo")]
    pub fn echo(_context: &GraphQLContext, room: String) -> FieldResult<String> {
        graphql_translate(Ok(room))
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Mutation {
    #[graphql(name = "test")]
    pub async fn test(_context: &GraphQLContext, input: String) -> FieldResult<String> {
        graphql_translate(Ok(input))
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

pub fn graphql_translate<T>(res: Result<T, anyhow::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(FieldError::from(e)),
    }
}
