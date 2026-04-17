use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::opportunity::{
        CloseWon, CloseLost, ExpiringOpportunity, OpportunityCreate,
        OpportunityFilters, OpportunityRow, OpportunityStatus, PipelineColumn,
        PipelineResponse,
    },
};

pub struct OpportunityRepo;

impl OpportunityRepo {
    pub async fn pipeline(pool: &PgPool, user_id: Uuid, is_manager: bool) -> AppResult<PipelineResponse> {
        let rows = sqlx::query_as!(
            OpportunityRow,
            r#"
            SELECT id, lead_id, client_id, vehicle_id, assigned_to,
                   status AS "status: _", title,
                   offered_price, discount, final_price,
                   probability, expected_close, closed_at,
                   lost_reason, notes, created_at, updated_at
            FROM opportunities
            WHERE status NOT IN ('closed_won', 'closed_lost')
              AND ($1::bool = TRUE OR assigned_to = $2)
            ORDER BY created_at DESC
            "#,
            is_manager,
            user_id
        )
        .fetch_all(pool)
        .await?;

        use crate::models::opportunity::OpportunityResponse;

        let stage_order: Vec<OpportunityStatus> = vec![
            OpportunityStatus::Prospecting,
            OpportunityStatus::NeedsAnalysis,
            OpportunityStatus::Proposal,
            OpportunityStatus::Negotiation,
            OpportunityStatus::ClosedWon,
            OpportunityStatus::ClosedLost,
        ];

        let columns = stage_order
            .into_iter()
            .map(|status| {
                let opps: Vec<OpportunityResponse> = rows
                    .iter()
                    .filter(|r| r.status == status)
                    .map(|r| OpportunityResponse::from(r.clone()))
                    .collect();

                let total_value: f64 = opps
                    .iter()
                    .filter_map(|o| o.offered_price.as_ref())
                    .filter_map(|p| p.parse::<f64>().ok())
                    .sum();

                let count = opps.len() as i64;

                PipelineColumn { status, opportunities: opps, count, total_value }
            })
            .collect();

        Ok(PipelineResponse { columns })
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<OpportunityRow>> {
        let row = sqlx::query_as!(
            OpportunityRow,
            r#"
            SELECT id, lead_id, client_id, vehicle_id, assigned_to,
                   status AS "status: _", title,
                   offered_price, discount, final_price,
                   probability, expected_close, closed_at,
                   lost_reason, notes, created_at, updated_at
            FROM opportunities WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn create(pool: &PgPool, req: &OpportunityCreate, assigned_to: Uuid) -> AppResult<OpportunityRow> {
        let row = sqlx::query_as!(
            OpportunityRow,
            r#"
            INSERT INTO opportunities
                (client_id, lead_id, vehicle_id, assigned_to, title,
                 offered_price, probability, expected_close, notes)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
            RETURNING id, lead_id, client_id, vehicle_id, assigned_to,
                      status AS "status: _", title,
                      offered_price, discount, final_price,
                      probability, expected_close, closed_at,
                      lost_reason, notes, created_at, updated_at
            "#,
            req.client_id,
            req.lead_id,
            req.vehicle_id,
            assigned_to,
            req.title,
            req.offered_price.map(|v| sqlx::types::BigDecimal::from(v as i64)),
            req.probability.unwrap_or(20),
            req.expected_close,
            req.notes,
        )
        .fetch_one(pool)
        .await?;
        Ok(row)
    }

    pub async fn update_status(pool: &PgPool, id: Uuid, status: &OpportunityStatus) -> AppResult<Option<OpportunityRow>> {
        let row = sqlx::query_as!(
            OpportunityRow,
            r#"
            UPDATE opportunities SET status = $2
            WHERE id = $1
            RETURNING id, lead_id, client_id, vehicle_id, assigned_to,
                      status AS "status: _", title,
                      offered_price, discount, final_price,
                      probability, expected_close, closed_at,
                      lost_reason, notes, created_at, updated_at
            "#,
            id,
            status as _
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn close_won(pool: &PgPool, id: Uuid, req: &CloseWon) -> AppResult<Option<OpportunityRow>> {
        let row = sqlx::query_as!(
            OpportunityRow,
            r#"
            UPDATE opportunities SET
                status      = 'closed_won',
                final_price = $2,
                closed_at   = NOW(),
                probability = 100,
                notes       = COALESCE($3, notes)
            WHERE id = $1
            RETURNING id, lead_id, client_id, vehicle_id, assigned_to,
                      status AS "status: _", title,
                      offered_price, discount, final_price,
                      probability, expected_close, closed_at,
                      lost_reason, notes, created_at, updated_at
            "#,
            id,
            sqlx::types::BigDecimal::from(req.final_price as i64),
            req.notes,
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn close_lost(pool: &PgPool, id: Uuid, req: &CloseLost) -> AppResult<Option<OpportunityRow>> {
        let row = sqlx::query_as!(
            OpportunityRow,
            r#"
            UPDATE opportunities SET
                status      = 'closed_lost',
                lost_reason = $2,
                closed_at   = NOW(),
                probability = 0
            WHERE id = $1
            RETURNING id, lead_id, client_id, vehicle_id, assigned_to,
                      status AS "status: _", title,
                      offered_price, discount, final_price,
                      probability, expected_close, closed_at,
                      lost_reason, notes, created_at, updated_at
            "#,
            id,
            req.lost_reason,
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    /// Oportunidades cuyo expected_close vence en los proximos N dias
    pub async fn find_expiring(pool: &PgPool, user_id: Uuid, is_manager: bool, days: i32) -> AppResult<Vec<ExpiringOpportunity>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, title, expected_close, probability,
                   offered_price,
                   (expected_close - CURRENT_DATE) AS days_remaining
            FROM opportunities
            WHERE status NOT IN ('closed_won', 'closed_lost')
              AND expected_close IS NOT NULL
              AND expected_close <= CURRENT_DATE + $3::int * INTERVAL '1 day'
              AND expected_close >= CURRENT_DATE
              AND ($1::bool = TRUE OR assigned_to = $2)
            ORDER BY expected_close ASC
            "#,
            is_manager,
            user_id,
            days
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .filter_map(|r| {
                Some(ExpiringOpportunity {
                    id: r.id,
                    title: r.title,
                    expected_close: r.expected_close?,
                    days_remaining: r.days_remaining? as i64,
                    probability: r.probability,
                    offered_price: r.offered_price.map(|v| v.to_string()),
                })
            })
            .collect())
    }
}
