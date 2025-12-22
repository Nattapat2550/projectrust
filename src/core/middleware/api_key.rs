use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    Extension,
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, PgPool};

use crate::config::db::DB;
use crate::core::errors::AppError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiClient {
    pub id: i32,
    pub name: String,
    pub api_key: String,
    pub is_active: bool,
}

/// ตรวจว่าเป็น error แบบ "table ไม่มี" หรือ "column ไม่มี" ใน Postgres ไหม
fn is_pg_undefined_table_or_column(err: &sqlx::Error) -> bool {
    match err {
        sqlx::Error::Database(db_err) => {
            // 42P01 = undefined_table, 42703 = undefined_column
            match db_err.code().map(|c| c.to_string()) {
                Some(code) => code == "42P01" || code == "42703",
                None => false,
            }
        }
        _ => false,
    }
}

/// ทำให้ schema ของ api_clients พร้อมใช้งานเสมอ (ปลอดภัยต่อของเดิม)
async fn ensure_api_clients_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 1) สร้างตารางถ้ายังไม่มี
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS api_clients (
          id         SERIAL PRIMARY KEY,
          name       VARCHAR(100) NOT NULL,
          api_key    VARCHAR(255) NOT NULL UNIQUE,
          is_active  BOOLEAN NOT NULL DEFAULT TRUE,
          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    // 2) กันกรณีตารางมีอยู่แล้วแต่ขาดคอลัมน์ (เพราะ schema เก่า)
    sqlx::query(r#"ALTER TABLE api_clients ADD COLUMN IF NOT EXISTS name VARCHAR(100);"#)
        .execute(pool)
        .await?;
    sqlx::query(r#"ALTER TABLE api_clients ADD COLUMN IF NOT EXISTS api_key VARCHAR(255);"#)
        .execute(pool)
        .await?;
    sqlx::query(r#"ALTER TABLE api_clients ADD COLUMN IF NOT EXISTS is_active BOOLEAN NOT NULL DEFAULT TRUE;"#)
        .execute(pool)
        .await?;
    sqlx::query(r#"ALTER TABLE api_clients ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();"#)
        .execute(pool)
        .await?;

    // 3) index (ถ้ายังไม่มี)
    sqlx::query(r#"CREATE INDEX IF NOT EXISTS idx_api_clients_active ON api_clients(is_active);"#)
        .execute(pool)
        .await?;

    // 4) seed key มาตรฐานแบบ "ไม่ทับของเดิม"
    sqlx::query(
        r#"
        INSERT INTO api_clients (name, api_key, is_active)
        VALUES
          ('angular-web', 'angular-key-123', TRUE),
          ('react-web',   'react-key-123',   TRUE),
          ('android-app', 'android-key-123', TRUE),
          ('docker',      'docker-key-123',  TRUE)
        ON CONFLICT (api_key) DO NOTHING;
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Middleware: require x-api-key (เหมือน pure-api1: app.use("/api", apiKeyAuth))
pub async fn mw_api_key_auth(
    Extension(db): Extension<DB>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let key = req
        .headers()
        .get("x-api-key")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.trim().to_string());

    let key = match key {
        Some(k) if !k.is_empty() => k,
        _ => return Err(AppError::unauthorized("API_KEY_MISSING", "Missing x-api-key")),
    };

    // ---- Query api_clients (ถ้าตาราง/คอลัมน์ไม่มี ให้ auto-fix แล้วลองใหม่) ----
    let row = match sqlx::query(
        r#"
        SELECT id, name, api_key, is_active
        FROM api_clients
        WHERE api_key = $1
        LIMIT 1
        "#,
    )
    .bind(&key)
    .fetch_optional(&db.pool)
    .await
    {
        Ok(r) => r,
        Err(e) => {
            // ✅ ถ้า schema ยังไม่พร้อม ให้สร้าง/ปรับ แล้ว retry 1 ครั้ง
            if is_pg_undefined_table_or_column(&e) {
                tracing::error!(
                    "api_clients schema missing, auto-creating/upgrading... (original error: {})",
                    e
                );

                ensure_api_clients_schema(&db.pool).await.map_err(AppError::from)?;

                // retry
                sqlx::query(
                    r#"
                    SELECT id, name, api_key, is_active
                    FROM api_clients
                    WHERE api_key = $1
                    LIMIT 1
                    "#,
                )
                .bind(&key)
                .fetch_optional(&db.pool)
                .await?
            } else {
                // error อื่น ๆ โยนตามจริง
                return Err(AppError::from(e));
            }
        }
    };

    let Some(row) = row else {
        return Err(AppError::unauthorized("API_KEY_INVALID", "Invalid x-api-key"));
    };

    let is_active: bool = row.get("is_active");
    if !is_active {
        return Err(AppError::unauthorized("API_KEY_INACTIVE", "API key is inactive"));
    }

    let client = ApiClient {
        id: row.get("id"),
        name: row.get("name"),
        api_key: row.get("api_key"),
        is_active,
    };

    req.extensions_mut().insert(client);

    Ok(next.run(req).await)
}
