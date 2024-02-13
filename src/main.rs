// use std::sync::Arc;

// use axum::Extension;
// use juniper_axum::{extract::JuniperRequest, response::JuniperResponse};
// use rc_voting_leptos::{context::GraphQLContext, graphql::Schema};
// use leptos::*;


#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

use rc_voting_leptos::context::GraphQLContext;
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use rc_voting_leptos::app::*;
    use rc_voting_leptos::fileserv::file_and_error_handler;
    // use rc_voting_leptos::api::api_routes;
    use rc_voting_leptos::db::get_pool;
    // use rc_voting_leptos::graphql::create_schema;
    
    // use axum::routing::{get, on, MethodFilter};
    use axum::Extension;
    // use juniper_axum::playground;
    #[allow(unused_imports)]
    use log::*;
    #[warn(unused_imports)]
    use std::sync::Arc;
    use tower::ServiceBuilder;
    use tower_http::ServiceBuilderExt;

    let env_result = dotenvy::dotenv();
    match env_result {
        Err(_) => println!(".env file did not exist, ignoring"),
        Ok(_) => println!("Loaded environment"),
    }

    tracing_subscriber::fmt::init();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // let qm_schema = create_schema();

    let pool = get_pool();

    let mut conn = pool.get().expect("Could not get connections for migrations");
    let migration_result = rc_voting_leptos::db::run_migrations(&mut conn);
    match migration_result {
        Ok(_) => info!("Migrations completed"),
        Err(e) => error!("Could not run migrations {:?}", e)
    };


    let context = Arc::new(GraphQLContext { pool: pool.clone(), session: None });

    // let routes = create_routes(&context);

    let middleware = ServiceBuilder::new().compression();
    // let graphql_routes = Router::new()
    //     .route(
    //         "/",
    //         on(MethodFilter::GET.or(MethodFilter::POST), custom_graphql),
    //     )
    //     // .route("/subscriptions", get(custom_subscriptions))
    //     // .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
    //     .route("/playground", get(playground("/graphql", "/subscriptions")))
    //     .nest("/api/v1", rc_voting_leptos::api::api_routes(context.clone()))
    //     .layer(Extension(context.clone()))
    //     .layer(Extension(Arc::new(qm_schema)))
    //     .layer(middleware.clone());

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        // .nest("/graphql", graphql_routes)
        // .nest("/api/v1", api_routes(context.clone()))
        .fallback(file_and_error_handler)
        .layer(Extension(context.clone()))
        .layer(middleware)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}

// async fn custom_graphql(
//     Extension(schema): Extension<Arc<Schema>>,
//     Extension(context): Extension<Arc<GraphQLContext>>,
//     JuniperRequest(request): JuniperRequest,
// ) -> JuniperResponse {
//     JuniperResponse(request.execute(&*schema, &context).await)
// }
