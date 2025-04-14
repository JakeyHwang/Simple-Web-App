use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub price: f64,
    pub stock: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateProduct {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(min = 1))]
    pub description: String,
    #[validate(url)]
    pub image_url: String,
    #[validate(range(min = 0.0))]
    pub price: f64,
    #[validate(range(min = 0))]
    pub stock: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateProduct {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub description: Option<String>,
    #[validate(url)]
    pub image_url: Option<String>,
    #[validate(range(min = 0.0))]
    pub price: Option<f64>,
    #[validate(range(min = 0))]
    pub stock: Option<i32>,
} 