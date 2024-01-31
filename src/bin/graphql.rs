#![allow(non_snake_case)]
#[warn(unused_imports)]
use aem_voting::get_env_typed;
use aem_voting::routes::app;
#[allow(unused_imports)]
use log::*;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let env_result = dotenvy::dotenv();
    match env_result {
        Err(_) => println!(".env file did not exist, ignoring"),
        Ok(_) => println!("Loaded environment"),
    }

    let port = get_env_typed::<u16>("PORT", 7000);

    // pretty_env_logger::init();
    tracing_subscriber::fmt::init();

    let app = app();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen for shutdown signal");
        })
        .await
        .expect("Could not keep server open");
}
