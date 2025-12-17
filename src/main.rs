mod api;
mod config;
mod core;

use config::env::Env;
use config::db::DB;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let env = Env::load();
    let db = DB::connect(&env.database_url).await;

    let app = api::router(db, env.clone())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], env.port));
    tracing::info!("🚀 Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}