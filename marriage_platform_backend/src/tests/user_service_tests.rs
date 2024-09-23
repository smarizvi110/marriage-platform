#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Pool;
    use sqlx::Postgres;

    #[tokio::test]
    async fn test_register_user() {
        let pool = Pool::<Postgres>::connect("postgres://test:test@localhost/test_db").await.unwrap();

        let result = register_user(&pool, "John Doe", "john@example.com", "password123").await;
        assert!(result.is_ok());
    }
}
