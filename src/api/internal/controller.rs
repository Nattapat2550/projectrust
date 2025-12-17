use axum::{extract::{State, Path}, Json};
use crate::config::db::DB;
use crate::core::errors::AppError;
use super::{service, schema::*};

pub async fn find_user(State(db): State<DB>, Json(body): Json<FindUserBody>) -> Result<Json<UserLite>, AppError> {
    let user = service::find_user(&db, body).await?;
    Ok(Json(user))
}

pub async fn create_user_email(State(db): State<DB>, Json(body): Json<CreateUserEmailBody>) -> Result<Json<UserLite>, AppError> {
    let user = service::create_user_email(&db, body).await?;
    Ok(Json(user))
}

pub async fn set_oauth_user(State(db): State<DB>, Json(body): Json<SetOAuthUserBody>) -> Result<Json<UserLite>, AppError> {
    let user = service::set_oauth_user(&db, body).await?;
    Ok(Json(user))
}

pub async fn store_verification_code(State(db): State<DB>, Json(body): Json<StoreVerificationCodeBody>) -> Result<Json<()>, AppError> {
    service::store_verification_code(&db, body).await?;
    Ok(Json(()))
}

pub async fn verify_code(State(db): State<DB>, Json(body): Json<VerifyCodeBody>) -> Result<Json<VerifyCodeResponse>, AppError> {
    let res = service::verify_code(&db, body).await?;
    Ok(Json(res))
}

pub async fn set_username_password(State(db): State<DB>, Json(body): Json<SetUsernamePasswordBody>) -> Result<Json<UserLite>, AppError> {
    let user = service::set_username_password(&db, body).await?;
    Ok(Json(user))
}

pub async fn create_reset_token(State(db): State<DB>, Json(body): Json<CreateResetTokenBody>) -> Result<Json<()>, AppError> {
    service::create_reset_token(&db, body).await?;
    Ok(Json(()))
}

pub async fn consume_reset_token(State(db): State<DB>, Json(body): Json<ConsumeResetTokenBody>) -> Result<Json<UserLite>, AppError> {
    let user = service::consume_reset_token(&db, body).await?;
    Ok(Json(user))
}

pub async fn set_password(State(db): State<DB>, Json(body): Json<SetPasswordBody>) -> Result<Json<()>, AppError> {
    service::set_password(&db, body).await?;
    Ok(Json(()))
}

// --- Controllers เดิม ---
pub async fn get_verification_token(State(db): State<DB>, Path(email): Path<String>) -> Result<Json<String>, AppError> {
    let token = service::get_verification_token(&db, email).await?;
    Ok(Json(token))
}
pub async fn get_reset_token(State(db): State<DB>, Path(email): Path<String>) -> Result<Json<String>, AppError> {
    let token = service::get_reset_token(&db, email).await?;
    Ok(Json(token))
}
pub async fn list_users(State(db): State<DB>) -> Result<Json<Vec<UserLite>>, AppError> {
    let users = service::list_users(&db).await?;
    Ok(Json(users))
}
pub async fn list_clients(State(db): State<DB>) -> Result<Json<Vec<ClientRow>>, AppError> {
    let clients = service::list_clients(&db).await?;
    Ok(Json(clients))
}
pub async fn set_client_active(State(db): State<DB>, Path(id): Path<i32>, Json(body): Json<serde_json::Value>) -> Result<Json<()>, AppError> {
    let active = body.get("is_active").and_then(|v| v.as_bool()).unwrap_or(true);
    service::set_client_active(&db, id, active).await?;
    Ok(Json(()))
}
pub async fn get_homepage_hero(State(db): State<DB>) -> Result<Json<HomepageHero>, AppError> {
    let hero = service::get_homepage_hero(&db).await?;
    Ok(Json(hero))
}
pub async fn put_homepage_hero(State(db): State<DB>, Json(body): Json<HomepageHeroBody>) -> Result<Json<HomepageHero>, AppError> {
    let hero = service::put_homepage_hero(&db, body).await?;
    Ok(Json(hero))
}
pub async fn get_carousel(State(db): State<DB>) -> Result<Json<Vec<CarouselItem>>, AppError> {
    let items = service::get_carousel(&db).await?;
    Ok(Json(items))
}
pub async fn create_carousel(State(db): State<DB>, Json(body): Json<CreateCarouselBody>) -> Result<Json<CarouselItem>, AppError> {
    let item = service::create_carousel(&db, body).await?;
    Ok(Json(item))
}
pub async fn update_carousel(State(db): State<DB>, Path(id): Path<i32>, Json(body): Json<UpdateCarouselBody>) -> Result<Json<CarouselItem>, AppError> {
    let item = service::update_carousel(&db, id, body).await?;
    Ok(Json(item))
}
pub async fn delete_carousel(State(db): State<DB>, Path(id): Path<i32>) -> Result<Json<()>, AppError> {
    service::delete_carousel(&db, id).await?;
    Ok(Json(()))
}