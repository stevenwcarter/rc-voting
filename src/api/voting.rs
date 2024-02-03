use axum::response::IntoResponse;
use axum::routing::get;
use axum::{http::StatusCode, response::Response, routing::post, Extension, Json, Router};
use std::sync::Arc;

use super::{middleware, SessionContext};
use crate::{
    context::GraphQLContext,
    models::{Ballot, User, Vote},
};

#[cfg(feature="ssr")]
pub fn voting_routes(context: Arc<GraphQLContext>) -> Router {
    Router::new()
        .route("/", post(handle_vote))
        .route("/election", get(handle_election))
        .route("/runner_up", get(handle_second_election))
        .layer(Extension(context.clone()))
        .layer(middleware())
}

#[cfg(feature="ssr")]
async fn handle_vote(
    Extension(context): Extension<Arc<GraphQLContext>>,
    SessionContext(login): SessionContext,
    Json(ballot): Json<Ballot>,
) -> Response {
    log::info!("{login} voted");
    let user = User::get(&context, &login);
    if user.is_err() {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    let user = user.unwrap();
    Vote::save_ballot(user.id, &ballot, &context);
    Json(ballot).into_response()
}

#[cfg(feature="ssr")]
async fn handle_election(
    Extension(context): Extension<Arc<GraphQLContext>>,
) -> impl IntoResponse {
    Json(Vote::run_election(&context))
}

#[cfg(feature="ssr")]
async fn handle_second_election(
    Extension(context): Extension<Arc<GraphQLContext>>,
) -> impl IntoResponse {
    let item = Vote::run_election(&context);
    Json(Vote::run_second_election(&context, &item))
}
