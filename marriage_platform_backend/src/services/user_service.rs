use crate::db::user::{create_user, get_user_by_email};
use sqlx::Pool;
use sqlx::Postgres;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::auth::jwt::create_jwt;
use crate::error::AppError;

pub async fn register_user(pool: &Pool<Postgres>, name: &str, email: &str, password: &str) -> Result<String, AppError> {
    // Check if email is already registered
    if get_user_by_email(pool, email).await?.is_some() {
        return Err(AppError::AuthError);
    }

    // Hash the password
    let password_hash = hash(password, DEFAULT_COST)?;

    // Create user in the database
    let user = create_user(pool, name, email, &password_hash, "user").await?;

    // Generate JWT token
    let token = create_jwt(&user.id.to_string(), "your_secret_key");

    Ok(token)
}

pub async fn login_user(pool: &Pool<Postgres>, email: &str, password: &str) -> Result<String, AppError> {
    // Get the user by email
    let user = match get_user_by_email(pool, email).await? {
        Some(user) => user,
        None => return Err(AppError::AuthError),
    };

    // Verify the password
    if !verify(password, &user.password_hash)? {
        return Err(AppError::AuthError);
    }

    // Generate JWT token
    let token = create_jwt(&user.id.to_string(), "your_secret_key");

    Ok(token)
}
