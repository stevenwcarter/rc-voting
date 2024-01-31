use axum::response::IntoResponse;
use axum::Json;
use axum::{http::StatusCode, routing::get, Router};
use serde::Serialize;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

use self::voting::voting_routes;
use crate::context::GraphQLContext;

pub mod voting;
//

pub fn middleware() -> tower::ServiceBuilder<
    tower::layer::util::Stack<
        tower_http::compression::CompressionLayer,
        tower::layer::util::Identity,
    >,
> {
    ServiceBuilder::new().compression()
}

pub fn api_routes(context: Arc<GraphQLContext>) -> Router {
    Router::new()
        .route("/test", get(get_test))
        .nest("/vote", voting_routes(context.clone()))
}

pub async fn get_test() -> &'static str {
    " hello world"
}

pub fn err_wrapper<T: Serialize>(result: anyhow::Result<T>) -> impl IntoResponse {
    Json(
        result
            .map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))
            .unwrap(),
    )
}

// pub struct SessionContext(pub GraphQLContext);

// #[async_trait]
// impl<S> FromRequestParts<S> for SessionContext
// where
//     S: Send + Sync,
// {
//     type Rejection = Response;

//     async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//         let cookie_jar = parts.extract::<CookieJar>().await.unwrap();

//         let session_id = cookie_jar.get("X-Login-Session-ID");

//         if let Some(session_id) = session_id {
//             let session_id = session_id.value();
//             trace!("Found session_id: {session_id}");
//             let Extension(context) = parts
//                 .extract::<Extension<Arc<GraphQLContext>>>()
//                 .await
//                 .map_err(|err| {
//                     error!("Error retrieving extension");
//                     err.into_response()
//                 })?;

//             let session = verify_auth_cookie(&context, session_id);
//             if let Some(session) = session {
//                 let context = context.attach_session(&session);
//                 Ok(Self(context.clone()))
//             } else {
//                 Err((
//                     StatusCode::UNAUTHORIZED,
//                     "No session found for provided session id",
//                 )
//                     .into_response())
//             }
//         } else {
//             Err((StatusCode::UNAUTHORIZED, "No session id found in request").into_response())
//         }
//     }
// }
