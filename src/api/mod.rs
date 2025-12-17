use axum::{middleware, Router, Extension};
use crate::config::{db::DB, env::Env};
use crate::core::middleware::{api_key, cors, jwt_auth};

pub mod auth;
pub mod users;
pub mod carousel;
pub mod homepage;
pub mod internal;
pub mod root;
pub mod admin;

pub fn router(db: DB, env: Env) -> Router {
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
        .nest("/", root_routes)
        .nest("/api", api_routes)
        .layer(cors::cors_layer())
        .layer(Extension(db))
        .layer(Extension(env))
}