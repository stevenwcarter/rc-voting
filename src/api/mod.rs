use axum::extract::FromRequestParts;
use log::*;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::{Extension, RequestPartsExt};
use axum::{async_trait, Json};
use axum::{http::StatusCode, routing::get, Router};
use crate::{context::GraphQLContext, models::Session, session::SessionSvc};
use axum_extra::extract::CookieJar;
use serde::Serialize;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;


#[cfg(feature="ssr")]
pub mod login;
#[cfg(feature="ssr")]
pub mod voting;

#[cfg(feature="ssr")]
pub fn middleware() -> tower::ServiceBuilder<
    tower::layer::util::Stack<
        tower_http::compression::CompressionLayer,
        tower::layer::util::Identity,
    >,
> {
    ServiceBuilder::new().compression()
}

#[cfg(feature="ssr")]
pub fn api_routes(context: Arc<GraphQLContext>) -> Router {
    use self::login::login_routes;
    use self::voting::voting_routes;
    Router::new()
        .route("/test", get(get_test))
        .nest("/vote", voting_routes(context.clone()))
        .nest("/login", login_routes(context.clone()))
}

#[cfg(feature="ssr")]
pub async fn get_test() -> &'static str {
    " hello world"
}

#[cfg(feature="ssr")]
pub fn err_wrapper<T: Serialize>(result: anyhow::Result<T>) -> impl IntoResponse {
    Json(
        result
            .map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))
            .unwrap(),
    )
}

pub struct SessionContext(pub GraphQLContext);

#[async_trait]
#[cfg(feature="ssr")]
impl<S> FromRequestParts<S> for SessionContext
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookie_jar = parts.extract::<CookieJar>().await.unwrap();

        let session_id = cookie_jar.get("X-Login");

        if let Some(session_id) = session_id {
            let session_id = session_id.value();
            trace!("Found session_id: {session_id}");
            let Extension(context) = parts
                .extract::<Extension<Arc<GraphQLContext>>>()
                .await
                .map_err(|err| {
                    error!("Error retrieving extension");
                    err.into_response()
                })?;

            let session = verify_auth_cookie(&context, session_id);
            if let Some(session) = session {
                let context = context.attach_session(&session);
                Ok(Self(context.clone()))
            } else {
                Err((
                    StatusCode::UNAUTHORIZED,
                    "No session found for provided session id",
                )
                    .into_response())
            }
        } else {
            Err((StatusCode::UNAUTHORIZED, "No session id found in request").into_response())
        }
    }
}

pub fn verify_auth_cookie(context: &GraphQLContext, session_id: &str) -> Option<Session> {
    let session = SessionSvc::get_session(context, session_id);

    if let Ok(session) = session {
        Some(session)
    } else {
        None
    }
}
