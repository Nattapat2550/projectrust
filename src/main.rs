mod api;
mod config;
mod core;

use config::env::Env;
use config::db::DB;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time;
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

    // 🌟 3.5 สั่งให้ระบบทำความสะอาดข้อมูลเริ่มทำงาน (Lazy Cleanup)
    // ส่ง clone ของ connection pool ไปทำงานเบื้องหลัง
    start_cleanup_job(db.pool.clone());

    // 4. Setup Router
    let app = api::router(db, env.clone())
        .layer(TraceLayer::new_for_http());

    // 5. Server Setup
    let addr = SocketAddr::from(([0, 0, 0, 0], env.port));
    let listener = TcpListener::bind(addr).await?;
    
    tracing::info!("🚀 Server running on http://{}", addr);

    // 6. Run Server with Graceful Shutdown
    // ✅ เพิ่ม .into_make_service_with_connect_info::<SocketAddr>() 
    // เพื่อให้ GovernorLayer (Rate Limit) สามารถหา IP Address เจอและไม่พ่น Error 500
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// 🌟 ฟังก์ชันจัดการทำความสะอาดข้อมูลเบื้องหลัง
fn start_cleanup_job(pool: sqlx::PgPool) {
    tokio::spawn(async move {
        // ตั้งเวลาให้ทำงานทุกๆ 24 ชั่วโมง (24 * 60 * 60 วินาที)
        let mut interval = time::interval(Duration::from_secs(24 * 60 * 60));
        
        loop {
            interval.tick().await;
            tracing::info!("⏳ [Scheduler] Checking for expired deleted users...");

            let query = r#"
                DELETE FROM users
                WHERE status = 'deleted'
                  AND updated_at <= NOW() - INTERVAL '30 days';
            "#;

            match sqlx::query(query).execute(&pool).await {
                Ok(result) => {
                    if result.rows_affected() > 0 {
                        tracing::info!("✅ [Scheduler] Permanently deleted {} expired user(s).", result.rows_affected());
                    }
                }
                Err(e) => tracing::error!("❌ [Scheduler] Failed to clean up expired users: {}", e),
            }
        }
    });
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