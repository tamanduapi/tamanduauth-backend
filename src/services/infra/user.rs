use std::sync::Arc;

use async_trait::async_trait;
use color_eyre::Result;
use sqlx::PgPool;

use crate::services::domain::user::{User, UserService};

pub struct SqlxUserService {
    pool: Arc<PgPool>,
}

impl SqlxUserService {
    pub fn new(pool: Arc<PgPool>) -> SqlxUserService {
        SqlxUserService { pool }
    }
}

#[async_trait]
impl UserService for SqlxUserService {
    async fn user_exists(&self, email: &str) -> Result<bool> {
        let row = sqlx::query!("SELECT user_id FROM Users WHERE email=$1", email)
            .fetch_one(&*self.pool)
            .await;

        match row {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn save_user(&self, user: &User) -> Result<()> {
        sqlx::query!(
            "INSERT INTO Users VALUES ($1, $2, $3, $4, NOW())",
            user.id,
            user.password,
            user.ra,
            user.email,
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
