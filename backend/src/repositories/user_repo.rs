use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::user::UserRow;

pub struct UserRepo;

impl UserRepo {
    pub async fn find_by_email(pool: &PgPool, email: &str) -> AppResult<Option<UserRow>> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, password_hash, full_name,
                   role AS "role: _", phone, is_active, created_at, updated_at
            FROM users
            WHERE email = $1 AND is_active = TRUE
            "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<UserRow>> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, password_hash, full_name,
                   role AS "role: _", phone, is_active, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn create(
        pool: &PgPool,
        email: &str,
        password_hash: &str,
        full_name: &str,
        role: &crate::models::user::UserRole,
        phone: Option<&str>,
    ) -> AppResult<UserRow> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            INSERT INTO users (email, password_hash, full_name, role, phone)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, email, password_hash, full_name,
                      role AS "role: _", phone, is_active, created_at, updated_at
            "#,
            email,
            password_hash,
            full_name,
            role as &crate::models::user::UserRole,
            phone
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate key") {
                AppError::Conflict("El email ya esta registrado".to_string())
            } else {
                AppError::Database(e)
            }
        })?;

        Ok(row)
    }

    pub async fn save_refresh_token(
        pool: &PgPool,
        user_id: Uuid,
        token_hash: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
            VALUES ($1, $2, $3)
            "#,
            user_id,
            token_hash,
            expires_at
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_refresh_token(
        pool: &PgPool,
        token_hash: &str,
    ) -> AppResult<Option<(Uuid, chrono::DateTime<chrono::Utc>)>> {
        let row = sqlx::query!(
            r#"
            SELECT user_id, expires_at
            FROM refresh_tokens
            WHERE token_hash = $1 AND revoked_at IS NULL
            "#,
            token_hash
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| (r.user_id, r.expires_at)))
    }

    pub async fn revoke_refresh_token(pool: &PgPool, token_hash: &str) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW()
            WHERE token_hash = $1
            "#,
            token_hash
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn revoke_all_user_tokens(pool: &PgPool, user_id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW()
            WHERE user_id = $1 AND revoked_at IS NULL
            "#,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
