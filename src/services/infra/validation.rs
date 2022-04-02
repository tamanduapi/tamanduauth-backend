use std::sync::Arc;

use async_trait::async_trait;
use color_eyre::Result;
use sqlx::PgPool;

use crate::services::domain::validation::{ValidationRequest, ValidationRequestService};

pub struct SqlxValidationRequestService {
    pool: Arc<PgPool>,
}

impl SqlxValidationRequestService {
    pub fn new(pool: Arc<PgPool>) -> SqlxValidationRequestService {
        SqlxValidationRequestService { pool }
    }
}

#[async_trait]
impl ValidationRequestService for SqlxValidationRequestService {
    async fn has_in_flight(&self, email: &str) -> Result<bool> {
        let max_age = std::env::var("TAMANDUAUTH_VALIDATION_REQUEST_MAX_AGE")
            .map(|i| i.parse::<u64>().unwrap())
            .unwrap_or(60);

        #[allow(deprecated)]
        let t =
            sqlx::types::time::PrimitiveDateTime::now() - std::time::Duration::from_secs(max_age);

        let row = sqlx::query!(
            "SELECT validation_id FROM Validations WHERE email=$1 AND created_at > $2",
            email,
            t,
        )
        .fetch_one(&*self.pool)
        .await;

        match row {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    #[allow(deprecated)]
    async fn save(&self, request: &ValidationRequest) -> Result<()> {
        sqlx::query!(
            "INSERT INTO Validations (validation_id, email, hashed_code, created_at) VALUES ($1, $2, $3, $4)",
            request.id,
            request.email,
            request.hashed_code,
            sqlx::types::time::PrimitiveDateTime::now(), // TODO use the saved created_at
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn validate(&self, email: &str, code: &str) -> Result<bool> {
        let row = sqlx::query!(
            "SELECT validation_id FROM Validations WHERE email=$1 AND hashed_code=$2",
            email,
            code,
        )
        .fetch_one(&*self.pool)
        .await;

        match row {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
