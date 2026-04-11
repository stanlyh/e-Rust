use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::client::{ClientCreate, ClientFilters, ClientRow, ClientUpdate},
};

pub struct ClientRepo;

impl ClientRepo {
    pub async fn list(
        pool: &PgPool,
        filters: &ClientFilters,
        user_id: Uuid,
        is_manager: bool,
    ) -> AppResult<(Vec<ClientRow>, i64)> {
        let page = filters.page.unwrap_or(1).max(1);
        let per_page = filters.per_page.unwrap_or(20).min(100);
        let offset = (page - 1) * per_page;
        let search = filters.search.as_deref().map(|s| format!("%{s}%"));

        let rows = sqlx::query_as!(
            ClientRow,
            r#"
            SELECT id, first_name, last_name, email, phone, mobile,
                   id_document, address, city, notes, assigned_to,
                   created_at, updated_at
            FROM clients
            WHERE ($1::uuid IS NULL OR assigned_to = $1)
              AND ($2::text IS NULL OR (
                    first_name ILIKE $2 OR last_name ILIKE $2 OR email ILIKE $2
              ))
              AND ($3::bool = TRUE OR assigned_to = $4)
            ORDER BY first_name, last_name
            LIMIT $5 OFFSET $6
            "#,
            filters.assigned_to,
            search,
            is_manager,
            user_id,
            per_page,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM clients
            WHERE ($1::uuid IS NULL OR assigned_to = $1)
              AND ($2::text IS NULL OR (
                    first_name ILIKE $2 OR last_name ILIKE $2 OR email ILIKE $2
              ))
              AND ($3::bool = TRUE OR assigned_to = $4)
            "#,
            filters.assigned_to,
            search,
            is_manager,
            user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        Ok((rows, total))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<ClientRow>> {
        let row = sqlx::query_as!(
            ClientRow,
            r#"
            SELECT id, first_name, last_name, email, phone, mobile,
                   id_document, address, city, notes, assigned_to,
                   created_at, updated_at
            FROM clients WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn create(
        pool: &PgPool,
        req: &ClientCreate,
        assigned_to: Uuid,
    ) -> AppResult<ClientRow> {
        let row = sqlx::query_as!(
            ClientRow,
            r#"
            INSERT INTO clients
                (first_name, last_name, email, phone, mobile,
                 id_document, address, city, notes, assigned_to)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
            RETURNING id, first_name, last_name, email, phone, mobile,
                      id_document, address, city, notes, assigned_to,
                      created_at, updated_at
            "#,
            req.first_name, req.last_name, req.email, req.phone, req.mobile,
            req.id_document, req.address, req.city, req.notes, assigned_to
        )
        .fetch_one(pool)
        .await?;
        Ok(row)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        req: &ClientUpdate,
    ) -> AppResult<Option<ClientRow>> {
        let row = sqlx::query_as!(
            ClientRow,
            r#"
            UPDATE clients SET
                first_name  = COALESCE($2, first_name),
                last_name   = COALESCE($3, last_name),
                email       = COALESCE($4, email),
                phone       = COALESCE($5, phone),
                mobile      = COALESCE($6, mobile),
                id_document = COALESCE($7, id_document),
                address     = COALESCE($8, address),
                city        = COALESCE($9, city),
                notes       = COALESCE($10, notes)
            WHERE id = $1
            RETURNING id, first_name, last_name, email, phone, mobile,
                      id_document, address, city, notes, assigned_to,
                      created_at, updated_at
            "#,
            id, req.first_name, req.last_name, req.email, req.phone, req.mobile,
            req.id_document, req.address, req.city, req.notes
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<bool> {
        let result = sqlx::query!("DELETE FROM clients WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
