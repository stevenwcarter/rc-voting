use crate::api::api_routes;
use crate::context::GraphQLContext;
use crate::db::get_pool;
use crate::graphql::{create_schema, Schema};

use axum::routing::{get, on, MethodFilter};
use axum::{Extension, Router};
use juniper_axum::extract::JuniperRequest;
use juniper_axum::playground;
use juniper_axum::response::JuniperResponse;
#[allow(unused_imports)]
use log::*;
#[warn(unused_imports)]
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::ServiceBuilderExt;

pub fn app() -> Router {
    let qm_schema = create_schema();

    let pool = get_pool();

    let context = Arc::new(GraphQLContext { pool: pool.clone() });

    // let routes = create_routes(&context);

    let middleware = ServiceBuilder::new().compression();
    let graphql_routes = Router::new()
        .route(
            "/",
            on(MethodFilter::GET.or(MethodFilter::POST), custom_graphql),
        )
        // .route("/subscriptions", get(custom_subscriptions))
        // .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
        .route("/playground", get(playground("/graphql", "/subscriptions")))
        .route("/test", get(root))
        .layer(Extension(context.clone()))
        .layer(Extension(Arc::new(qm_schema)))
        .layer(middleware.clone());

    Router::new()
        // .route("/socket.io", get(ws_handler))
        // .route("/sockets", get(get_sockets))
        .nest("/graphql", graphql_routes)
        .nest("/api/v1", api_routes(context.clone()))
        .nest_service(
            "/",
            ServeDir::new("dnd-react/build")
                .not_found_service(ServeFile::new("dnd-react/build/index.html")),
        )
        // .fallback(serve_index)
        .layer(Extension(context.clone()))
        .layer(middleware)
}

async fn root() -> &'static str {
    "Hello world!"
}

// async fn custom_subscriptions(
//     Extension(schema): Extension<Arc<Schema>>,
//     Extension(context): Extension<Arc<GraphQLContext>>,
//     ws: WebSocketUpgrade,
// ) -> Response {
//     let context = context.clone();
//     ws.protocols(["graphql-transport-ws", "graphql-ws"])
//         .max_frame_size(1024)
//         .max_message_size(1024)
//         .max_write_buffer_size(100)
//         .on_upgrade(move |socket| {
//             subscriptions::serve_ws(
//                 socket,
//                 schema,
//                 ConnectionConfig::new(context.clone()).with_max_in_flight_operations(10),
//             )
//         })
// }

async fn custom_graphql(
    Extension(schema): Extension<Arc<Schema>>,
    Extension(context): Extension<Arc<GraphQLContext>>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    JuniperResponse(request.execute(&*schema, &context).await)
}

// async fn ws_handler(
//     SessionContext(context): SessionContext,
//     ws: WebSocketUpgrade,
// ) -> impl IntoResponse {
//     println!(
//         "{} connected to websocket",
//         context.session.as_ref().unwrap().email.clone().unwrap()
//     );
//     ws.on_upgrade(move |ws: WebSocket| handle_ws_client(ws, context.clone()))
// }
// async fn handle_ws_client(websocket: WebSocket, context: GraphQLContext) {
//     // receiver - this server, from websocket client
//     // sender - diff clients connected to this server
//     info!("Handling ws client");
//     let socket_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
//     let (mut sender, mut receiver) = websocket.split();
//     let (tx, rx) = mpsc::unbounded_channel::<Message>();
//     let mut rx = UnboundedReceiverStream::new(rx);

//     tokio::task::spawn(async move {
//         while let Some(message) = rx.next().await {
//             sender
//                 .send(message)
//                 .unwrap_or_else(|e| {
//                     error!("websocket send error: {}", e);
//                 })
//                 .await;
//         }
//     });
//     context.socket_sessions.write().await.insert(socket_id, tx);
//     context
//         .user_sockets
//         .write()
//         .await
//         .entry(context.session.as_ref().unwrap().playerId.clone().unwrap())
//         .or_default()
//         .insert(socket_id);

//     info!("waiting for user messages");
//     while let Some(result) = receiver.next().await {
//         let msg = match result {
//             Ok(msg) => msg,
//             Err(e) => {
//                 error!("websocket error(uid={}): {}", socket_id, e);
//                 break;
//             }
//         };
//         user_message(socket_id, msg, &context).await;
//     }

//     info!("User disconnected");
//     // user_ws_rx stream will keep processing as long as the user stays
//     // connected. Once they disconnect, then...
//     user_disconnected(socket_id, &context).await;
// }

// async fn user_disconnected(my_id: usize, context: &GraphQLContext) {
//     // Stream closed up, so remove from the user list
//     context.socket_sessions.write().await.remove(&my_id);
//     context
//         .user_sockets
//         .write()
//         .await
//         .entry(context.session.as_ref().unwrap().playerId.clone().unwrap())
//         .or_default()
//         .remove(&my_id);
// }
// pub async fn get_sockets(Extension(context): Extension<Arc<GraphQLContext>>) -> Response {
//     let user_sockets = context.user_sockets.read().await.clone();
//     let user_sockets = serde_json::to_value(&user_sockets).unwrap();

//     let socket_sessions = context.socket_sessions.read().await.len();

//     let user_sockets = json!({
//       "user_sockets": user_sockets,
//       "socket_count": socket_sessions,
//     });
//     let user_sockets: serde_json::Value = serde_json::to_value(user_sockets).unwrap();

//     Json(user_sockets).into_response()
// }
