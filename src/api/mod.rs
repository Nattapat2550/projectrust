use axum::{middleware, Extension, Router};

use crate::config::{db::DB, env::Env};
use crate::core::middleware::{api_key, cors, jwt_auth};

pub mod admin;
pub mod auth;
pub mod carousel;
pub mod homepage;
pub mod internal;
pub mod root;
pub mod users;

pub fn router(db: DB, env: Env) -> Router {
    // เก็บไว้ใช้กับ cors_layer(&Env) แบบไม่กิน ownership ของ env
    let env_for_cors = env.clone();

    // --- 1. Public Routes ---
    let auth_routes = auth::routes::routes(db.clone(), env.clone());
    let root_routes = root::routes::routes();
    let carousel_routes = carousel::routes::routes(db.clone());
    let homepage_routes = homepage::routes::routes(db.clone());

    // --- 2. Protected Routes (User) ---
    let users_routes = users::routes::routes(db.clone())
        .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth));

    // --- 3. Protected Routes (Admin) ---
    let admin_routes = admin::routes::routes(db.clone())
        .route_layer(middleware::from_fn(jwt_auth::mw_jwt_auth));

    // --- 4. Internal Routes (API Key) ---
    let internal_routes = internal::routes::routes(db.clone())
        .route_layer(middleware::from_fn(api_key::mw_api_key_auth));

    // --- 5. Merge API Routes ---
    let api_routes = Router::new()
        .nest("/auth", auth_routes)
        .nest("/users", users_routes)
        .nest("/carousel", carousel_routes)
        .nest("/homepage", homepage_routes)
        .nest("/admin", admin_routes)
        .nest("/internal", internal_routes);

    // --- 6. Final Router Assembly ---
    Router::new()
        // ✅ สำคัญ: ห้าม nest("/") ให้ merge แทน
        .merge(root_routes)
        .nest("/api", api_routes)
        // ✅ แก้ตรงนี้: cors_layer ต้องรับ &Env
        .layer(cors::cors_layer(&env_for_cors))
        .layer(Extension(db))
        .layer(Extension(env))
}
