mod api;
mod config;
mod core;

use config::env::Env;
use config::db::DB;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Init Tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // 2. Load Env
    let env = Env::load();

    // 3. Connect Database (Handle Error gracefully)
    let db = match DB::connect(&env.database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("🔥 Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // 4. Setup Router
    let app = api::router(db, env.clone())
        .layer(TraceLayer::new_for_http());

    // 5. Server Setup
    let addr = SocketAddr::from(([0, 0, 0, 0], env.port));
    let listener = TcpListener::bind(addr).await?;
    
    tracing::info!("🚀 Server running on http://{}", addr);

    // 6. Run Server with Graceful Shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// ฟังก์ชันดักจับ Signal (Ctrl+C) เพื่อปิด Server อย่างปลอดภัย
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("🛑 Shutting down gracefully...");
}