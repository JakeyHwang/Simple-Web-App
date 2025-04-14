use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreateProduct, CreateUser, LoginUser, UpdateProduct};
use crate::services::{register_user, login_user, create_product, get_products, get_product, update_product, delete_product};
use crate::utils::AppError;
use crate::config::Config;

pub async fn register(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUser>,
) -> HttpResponse {
    match register_user(&pool, &user_data).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => e.into(),
    }
}

pub async fn login(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    login_data: web::Json<LoginUser>,
) -> HttpResponse {
    match login_user(&pool, &login_data, &config).await {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "token": token })),
        Err(e) => e.into(),
    }
}

pub async fn create_product_handler(
    pool: web::Data<PgPool>,
    product_data: web::Json<CreateProduct>,
) -> HttpResponse {
    match create_product(&pool, &product_data).await {
        Ok(product) => HttpResponse::Created().json(product),
        Err(e) => e.into(),
    }
}

pub async fn get_products_handler(pool: web::Data<PgPool>) -> HttpResponse {
    match get_products(&pool).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => e.into(),
    }
}

pub async fn get_product_handler(
    pool: web::Data<PgPool>,
    product_id: web::Path<Uuid>,
) -> HttpResponse {
    match get_product(&pool, *product_id).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => e.into(),
    }
}

pub async fn update_product_handler(
    pool: web::Data<PgPool>,
    product_id: web::Path<Uuid>,
    product_data: web::Json<UpdateProduct>,
) -> HttpResponse {
    match update_product(&pool, *product_id, &product_data).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => e.into(),
    }
}

pub async fn delete_product_handler(
    pool: web::Data<PgPool>,
    product_id: web::Path<Uuid>,
) -> HttpResponse {
    match delete_product(&pool, *product_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.into(),
    }
} 