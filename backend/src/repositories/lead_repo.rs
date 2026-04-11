use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::lead::{LeadCreate, LeadFilters, LeadRow, LeadUpdate},
};

pub struct LeadRepo;

impl LeadRepo {
    pub async fn list(
        pool: &PgPool,
        filters: &LeadFilters,
        user_id: Uuid,
        is_manager: bool,
    ) -> AppResult<(Vec<LeadRow>, i64)> {
        let page = filters.page.unwrap_or(1).max(1);
        let per_page = filters.per_page.unwrap_or(20).min(100);
        let offset = (page - 1) * per_page;

        let rows = sqlx::query_as!(
            LeadRow,
            r#"
            SELECT id, client_id, assigned_to,
                   source AS "source: _", status AS "status: _",
                   interest_make, interest_model, interest_year,
                   budget_min, budget_max, notes,
                   contacted_at, qualified_at,
                   created_at, updated_at
            FROM leads
            WHERE ($1::uuid IS NULL OR assigned_to = $1)
              AND ($2::text IS NULL OR status::text = $2)
              AND ($3::text IS NULL OR source::text = $3)
              AND ($4::bool = TRUE OR assigned_to = $5)
            ORDER BY created_at DESC
            LIMIT $6 OFFSET $7
            "#,
            filters.assigned_to,
            filters.status,
            filters.source,
            is_manager,
            user_id,
            per_page,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM leads
            WHERE ($1::uuid IS NULL OR assigned_to = $1)
              AND ($2::text IS NULL OR status::text = $2)
              AND ($3::text IS NULL OR source::text = $3)
              AND ($4::bool = TRUE OR assigned_to = $5)
            "#,
            filters.assigned_to,
            filters.status,
            filters.source,
            is_manager,
            user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        Ok((rows, total))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<LeadRow>> {
        let row = sqlx::query_as!(
            LeadRow,
            r#"
            SELECT id, client_id, assigned_to,
                   source AS "source: _", status AS "status: _",
                   interest_make, interest_model, interest_year,
                   budget_min, budget_max, notes,
                   contacted_at, qualified_at,
                   created_at, updated_at
            FROM leads WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn create(
        pool: &PgPool,
        req: &LeadCreate,
        assigned_to: Uuid,
    ) -> AppResult<LeadRow> {
        let row = sqlx::query_as!(
            LeadRow,
            r#"
            INSERT INTO leads
                (client_id, assigned_to, source,
                 interest_make, interest_model, interest_year,
                 budget_min, budget_max, notes)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
            RETURNING id, client_id, assigned_to,
                      source AS "source: _", status AS "status: _",
                      interest_make, interest_model, interest_year,
                      budget_min, budget_max, notes,
                      contacted_at, qualified_at,
                      created_at, updated_at
            "#,
            req.client_id,
            assigned_to,
            req.source as _,
            req.interest_make,
            req.interest_model,
            req.interest_year,
            req.budget_min.map(|v| sqlx::types::BigDecimal::from(v as i64)),
            req.budget_max.map(|v| sqlx::types::BigDecimal::from(v as i64)),
            req.notes,
        )
        .fetch_one(pool)
        .await?;
        Ok(row)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        req: &LeadUpdate,
    ) -> AppResult<Option<LeadRow>> {
        let row = sqlx::query_as!(
            LeadRow,
            r#"
            UPDATE leads SET
                source         = COALESCE($2, source),
                status         = COALESCE($3, status),
                interest_make  = COALESCE($4, interest_make),
                interest_model = COALESCE($5, interest_model),
                interest_year  = COALESCE($6, interest_year),
                budget_min     = COALESCE($7, budget_min),
                budget_max     = COALESCE($8, budget_max),
                notes          = COALESCE($9, notes),
                assigned_to    = COALESCE($10, assigned_to),
                contacted_at   = CASE WHEN $3::lead_status = 'contacted' AND contacted_at IS NULL
                                      THEN NOW() ELSE contacted_at END,
                qualified_at   = CASE WHEN $3::lead_status = 'qualified' AND qualified_at IS NULL
                                      THEN NOW() ELSE qualified_at END
            WHERE id = $1
            RETURNING id, client_id, assigned_to,
                      source AS "source: _", status AS "status: _",
                      interest_make, interest_model, interest_year,
                      budget_min, budget_max, notes,
                      contacted_at, qualified_at,
                      created_at, updated_at
            "#,
            id,
            req.source as _,
            req.status as _,
            req.interest_make,
            req.interest_model,
            req.interest_year,
            req.budget_min.map(|v| sqlx::types::BigDecimal::from(v as i64)),
            req.budget_max.map(|v| sqlx::types::BigDecimal::from(v as i64)),
            req.notes,
            req.assigned_to,
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<bool> {
        let result = sqlx::query!("DELETE FROM leads WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
