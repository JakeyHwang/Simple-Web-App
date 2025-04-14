use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreateProduct, CreateUser, LoginUser, Product, UpdateProduct, User};
use crate::utils::AppError;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
}

pub async fn register_user(pool: &PgPool, user_data: &CreateUser) -> Result<User, AppError> {
    // Check if user already exists
    let existing_user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        user_data.email
    )
    .fetch_optional(pool)
    .await?;

    if existing_user.is_some() {
        return Err(AppError::Validation("User already exists".to_string()));
    }

    // Hash password
    let password_hash = hash(user_data.password.as_bytes(), DEFAULT_COST)?;

    // Create user
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password_hash)
        VALUES ($1, $2)
        RETURNING *
        "#,
        user_data.email,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn login_user(pool: &PgPool, login_data: &LoginUser, config: &Config) -> Result<String, AppError> {
    // Get user
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        login_data.email
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::Auth("Invalid credentials".to_string()))?;

    // Verify password
    if !verify(&login_data.password, &user.password_hash)? {
        return Err(AppError::Auth("Invalid credentials".to_string()));
    }

    // Generate JWT
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )?;

    Ok(token)
}

pub async fn create_product(
    pool: &PgPool,
    product_data: &CreateProduct,
) -> Result<Product, AppError> {
    let product = sqlx::query_as!(
        Product,
        r#"
        INSERT INTO products (title, description, image_url, price, stock)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        product_data.title,
        product_data.description,
        product_data.image_url,
        product_data.price,
        product_data.stock
    )
    .fetch_one(pool)
    .await?;

    Ok(product)
}

pub async fn get_products(pool: &PgPool) -> Result<Vec<Product>, AppError> {
    let products = sqlx::query_as!(
        Product,
        "SELECT * FROM products ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(products)
}

pub async fn get_product(pool: &PgPool, product_id: Uuid) -> Result<Product, AppError> {
    let product = sqlx::query_as!(
        Product,
        "SELECT * FROM products WHERE id = $1",
        product_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

    Ok(product)
}

pub async fn update_product(
    pool: &PgPool,
    product_id: Uuid,
    product_data: &UpdateProduct,
) -> Result<Product, AppError> {
    let product = sqlx::query_as!(
        Product,
        r#"
        UPDATE products
        SET title = COALESCE($1, title),
            description = COALESCE($2, description),
            image_url = COALESCE($3, image_url),
            price = COALESCE($4, price),
            stock = COALESCE($5, stock),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $6
        RETURNING *
        "#,
        product_data.title,
        product_data.description,
        product_data.image_url,
        product_data.price,
        product_data.stock,
        product_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

    Ok(product)
}

pub async fn delete_product(pool: &PgPool, product_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM products WHERE id = $1", product_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Product not found".to_string()));
    }

    Ok(())
} 