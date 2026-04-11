use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::activity::{ActivityCreate, ActivityRow, ActivityStatus, ActivityUpdate},
};

pub struct ActivityRepo;

impl ActivityRepo {
    pub async fn find_by_range(
        pool: &PgPool,
        user_id: Uuid,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> AppResult<Vec<ActivityRow>> {
        let rows = sqlx::query_as!(
            ActivityRow,
            r#"
            SELECT id, title, description,
                   type AS "type: _", status AS "status: _",
                   scheduled_start, scheduled_end, completed_at,
                   outcome, next_action, assigned_to,
                   client_id, lead_id, opportunity_id, vehicle_id,
                   created_at, updated_at
            FROM activities
            WHERE assigned_to = $1
              AND scheduled_start < $3
              AND scheduled_end > $2
            ORDER BY scheduled_start ASC
            "#,
            user_id, from, to
        )
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    pub async fn count_overdue(pool: &PgPool, user_id: Uuid) -> AppResult<i64> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM activities
            WHERE assigned_to = $1
              AND status = 'scheduled'
              AND scheduled_end < NOW()
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;
        Ok(count.unwrap_or(0))
    }

    pub async fn find_upcoming(pool: &PgPool, user_id: Uuid) -> AppResult<Vec<ActivityRow>> {
        let rows = sqlx::query_as!(
            ActivityRow,
            r#"
            SELECT id, title, description,
                   type AS "type: _", status AS "status: _",
                   scheduled_start, scheduled_end, completed_at,
                   outcome, next_action, assigned_to,
                   client_id, lead_id, opportunity_id, vehicle_id,
                   created_at, updated_at
            FROM activities
            WHERE assigned_to = $1
              AND status = 'scheduled'
              AND scheduled_start >= NOW()
              AND scheduled_start <= NOW() + INTERVAL '7 days'
            ORDER BY scheduled_start ASC
            LIMIT 20
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    pub async fn find_overdue(pool: &PgPool, user_id: Uuid) -> AppResult<Vec<ActivityRow>> {
        let rows = sqlx::query_as!(
            ActivityRow,
            r#"
            SELECT id, title, description,
                   type AS "type: _", status AS "status: _",
                   scheduled_start, scheduled_end, completed_at,
                   outcome, next_action, assigned_to,
                   client_id, lead_id, opportunity_id, vehicle_id,
                   created_at, updated_at
            FROM activities
            WHERE assigned_to = $1
              AND status = 'scheduled'
              AND scheduled_end < NOW()
            ORDER BY scheduled_end ASC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<ActivityRow>> {
        let row = sqlx::query_as!(
            ActivityRow,
            r#"
            SELECT id, title, description,
                   type AS "type: _", status AS "status: _",
                   scheduled_start, scheduled_end, completed_at,
                   outcome, next_action, assigned_to,
                   client_id, lead_id, opportunity_id, vehicle_id,
                   created_at, updated_at
            FROM activities WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        req: &ActivityCreate,
    ) -> AppResult<ActivityRow> {
        let row = sqlx::query_as!(
            ActivityRow,
            r#"
            INSERT INTO activities
                (title, description, type, scheduled_start, scheduled_end,
                 assigned_to, client_id, lead_id, opportunity_id, vehicle_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, title, description,
                      type AS "type: _", status AS "status: _",
                      scheduled_start, scheduled_end, completed_at,
                      outcome, next_action, assigned_to,
                      client_id, lead_id, opportunity_id, vehicle_id,
                      created_at, updated_at
            "#,
            req.title,
            req.description,
            req.activity_type as _,
            req.scheduled_start,
            req.scheduled_end,
            user_id,
            req.client_id,
            req.lead_id,
            req.opportunity_id,
            req.vehicle_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(row)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        req: &ActivityUpdate,
    ) -> AppResult<Option<ActivityRow>> {
        let row = sqlx::query_as!(
            ActivityRow,
            r#"
            UPDATE activities SET
                title           = COALESCE($2, title),
                description     = COALESCE($3, description),
                type            = COALESCE($4, type),
                scheduled_start = COALESCE($5, scheduled_start),
                scheduled_end   = COALESCE($6, scheduled_end),
                client_id       = COALESCE($7, client_id),
                lead_id         = COALESCE($8, lead_id),
                opportunity_id  = COALESCE($9, opportunity_id),
                vehicle_id      = COALESCE($10, vehicle_id)
            WHERE id = $1
            RETURNING id, title, description,
                      type AS "type: _", status AS "status: _",
                      scheduled_start, scheduled_end, completed_at,
                      outcome, next_action, assigned_to,
                      client_id, lead_id, opportunity_id, vehicle_id,
                      created_at, updated_at
            "#,
            id,
            req.title,
            req.description,
            req.activity_type as _,
            req.scheduled_start,
            req.scheduled_end,
            req.client_id,
            req.lead_id,
            req.opportunity_id,
            req.vehicle_id,
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn complete(
        pool: &PgPool,
        id: Uuid,
        outcome: &str,
        next_action: Option<&str>,
    ) -> AppResult<Option<ActivityRow>> {
        let row = sqlx::query_as!(
            ActivityRow,
            r#"
            UPDATE activities SET
                status       = 'completed',
                completed_at = NOW(),
                outcome      = $2,
                next_action  = $3
            WHERE id = $1 AND status = 'scheduled'
            RETURNING id, title, description,
                      type AS "type: _", status AS "status: _",
                      scheduled_start, scheduled_end, completed_at,
                      outcome, next_action, assigned_to,
                      client_id, lead_id, opportunity_id, vehicle_id,
                      created_at, updated_at
            "#,
            id, outcome, next_action
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn reschedule(
        pool: &PgPool,
        id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AppResult<Option<ActivityRow>> {
        let row = sqlx::query_as!(
            ActivityRow,
            r#"
            UPDATE activities SET
                scheduled_start = $2,
                scheduled_end   = $3,
                status          = 'rescheduled'
            WHERE id = $1
            RETURNING id, title, description,
                      type AS "type: _", status AS "status: _",
                      scheduled_start, scheduled_end, completed_at,
                      outcome, next_action, assigned_to,
                      client_id, lead_id, opportunity_id, vehicle_id,
                      created_at, updated_at
            "#,
            id, start, end
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<bool> {
        let result = sqlx::query!("DELETE FROM activities WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
