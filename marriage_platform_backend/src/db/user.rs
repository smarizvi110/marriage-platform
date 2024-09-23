use sqlx::{Pool, Postgres};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

pub async fn create_user(pool: &Pool<Postgres>, name: &str, email: &str, password_hash: &str, role: &str) -> Result<User, sqlx:Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, email, password_hash, role)"
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, email, password_hash, role
        "#,
        name,
        email,
        password_hash,
        role
    )
    .fetch_one(pool)
    .await?;;

    Ok(user)
}

pub async fn get_user_by_email(pool: &Pool<Postgres>, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, password_hash, role
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
