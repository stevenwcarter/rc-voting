use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Response, routing::post, Extension, Json, Router};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use std::sync::Arc;

use super::{middleware, SessionContext};
use crate::{context::GraphQLContext, models::User};
#[allow(unused_imports)]
use log::*;

#[cfg(feature="ssr")]
pub fn login_routes(context: Arc<GraphQLContext>) -> Router {
    Router::new()
        .route("/", post(handle_login))
        .layer(Extension(context.clone()))
        .layer(middleware())
}

#[cfg(feature="ssr")]
async fn handle_login(
    Extension(context): Extension<Arc<GraphQLContext>>,
    SessionContext(login): SessionContext,
    cookies: CookieJar,
) -> Response {
    use cookie::time::Duration;
    let u = User::login(&context, &login).expect("Unauthorized");

    let session_cookie = Cookie::build(("X-Login", u.username.clone()))
        .path("/")
        .secure(true)
        .max_age(Duration::days(14))
        .build();

    (StatusCode::CREATED, cookies.add(session_cookie), Json(u)).into_response()
}
