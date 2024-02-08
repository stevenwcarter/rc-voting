use axum::response::IntoResponse;
use axum::routing::get;
use axum::{response::Response, routing::post, Extension, Json, Router};
use std::sync::Arc;

use super::{middleware, SessionContext};
use crate::{
    context::GraphQLContext,
    models::{Ballot, Vote},
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
    SessionContext(context): SessionContext,
    Json(ballot): Json<Ballot>,
) -> Response {
    let user = context.session.as_ref().unwrap().get_user(&context).unwrap();
    log::info!("{} voted", user.email);

    Vote::save_ballot(&context, &ballot);
    Json(ballot).into_response()
}

#[cfg(feature="ssr")]
async fn handle_election(
    Extension(context): Extension<Arc<GraphQLContext>>,
    election_uuid: String,
) -> impl IntoResponse {
    Json(Vote::run_election(&context, &election_uuid))
}

#[cfg(feature="ssr")]
async fn handle_second_election(
    Extension(context): Extension<Arc<GraphQLContext>>,
    election_uuid: String,
) -> impl IntoResponse {
    let item = Vote::run_election(&context, &election_uuid);
    Json(Vote::run_second_election(&context, &election_uuid, &item))
}
