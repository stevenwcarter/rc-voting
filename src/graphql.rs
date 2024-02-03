use std::pin::Pin;

use crate::context::GraphQLContext;

use futures_util::Stream;
use juniper::{graphql_subscription, FieldError, FieldResult, RootNode};

pub struct Query;

#[juniper::graphql_object(Context = GraphQLContext)]
#[cfg(feature="ssr")]
impl Query {
    #[graphql(name = "echo")]
    pub fn echo(_context: &GraphQLContext, room: String) -> FieldResult<String> {
        graphql_translate(Ok(room))
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = GraphQLContext)]
#[cfg(feature="ssr")]
impl Mutation {
    #[graphql(name = "test")]
    pub async fn test(_context: &GraphQLContext, input: String) -> FieldResult<String> {
        graphql_translate(Ok(input))
    }
}

pub struct Subscription;
type StringStream = Pin<Box<dyn Stream<Item = Result<String, FieldError>> + Send>>;

#[graphql_subscription(context = GraphQLContext)]
#[cfg(feature="ssr")]
impl Subscription {
    async fn hello_world() -> StringStream {
        let stream =
            futures::stream::iter(vec![Ok(String::from("Hello")), Ok(String::from("World!"))]);
        Box::pin(stream)
    }
}

pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

#[cfg(feature="ssr")]
pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, Subscription)
}

#[cfg(feature="ssr")]
pub fn graphql_translate<T>(res: Result<T, anyhow::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(FieldError::from(e)),
    }
}
