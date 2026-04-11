use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "opportunity_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OpportunityStatus {
    Prospecting,
    NeedsAnalysis,
    Proposal,
    Negotiation,
    ClosedWon,
    ClosedLost,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OpportunityRow {
    pub id: Uuid,
    pub lead_id: Option<Uuid>,
    pub client_id: Uuid,
    pub vehicle_id: Option<Uuid>,
    pub assigned_to: Uuid,
    pub status: OpportunityStatus,
    pub title: String,
    pub offered_price: Option<sqlx::types::BigDecimal>,
    pub discount: Option<sqlx::types::BigDecimal>,
    pub final_price: Option<sqlx::types::BigDecimal>,
    pub probability: i16,
    pub expected_close: Option<NaiveDate>,
    pub closed_at: Option<DateTime<Utc>>,
    pub lost_reason: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct OpportunityResponse {
    pub id: Uuid,
    pub lead_id: Option<Uuid>,
    pub client_id: Uuid,
    pub vehicle_id: Option<Uuid>,
    pub assigned_to: Uuid,
    pub status: OpportunityStatus,
    pub title: String,
    pub offered_price: Option<String>,
    pub discount: Option<String>,
    pub final_price: Option<String>,
    pub probability: i16,
    pub expected_close: Option<NaiveDate>,
    pub closed_at: Option<DateTime<Utc>>,
    pub lost_reason: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<OpportunityRow> for OpportunityResponse {
    fn from(r: OpportunityRow) -> Self {
        Self {
            id: r.id,
            lead_id: r.lead_id,
            client_id: r.client_id,
            vehicle_id: r.vehicle_id,
            assigned_to: r.assigned_to,
            status: r.status,
            title: r.title,
            offered_price: r.offered_price.map(|v| v.to_string()),
            discount: r.discount.map(|v| v.to_string()),
            final_price: r.final_price.map(|v| v.to_string()),
            probability: r.probability,
            expected_close: r.expected_close,
            closed_at: r.closed_at,
            lost_reason: r.lost_reason,
            notes: r.notes,
            created_at: r.created_at,
        }
    }
}

/// Una columna del pipeline con sus oportunidades agrupadas
#[derive(Debug, Serialize)]
pub struct PipelineColumn {
    pub status: OpportunityStatus,
    pub opportunities: Vec<OpportunityResponse>,
    pub count: i64,
    pub total_value: f64,
}

#[derive(Debug, Serialize)]
pub struct PipelineResponse {
    pub columns: Vec<PipelineColumn>,
}

/// Oportunidad proxima a vencer (para alertas del calendario)
#[derive(Debug, Serialize)]
pub struct ExpiringOpportunity {
    pub id: Uuid,
    pub title: String,
    pub expected_close: NaiveDate,
    pub days_remaining: i64,
    pub probability: i16,
    pub offered_price: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct OpportunityCreate {
    pub client_id: Uuid,
    pub lead_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
    #[validate(length(min = 2, max = 255))]
    pub title: String,
    pub offered_price: Option<f64>,
    pub probability: Option<i16>,
    pub expected_close: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StatusUpdate {
    pub status: OpportunityStatus,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CloseWon {
    pub final_price: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CloseLost {
    #[validate(length(min = 1))]
    pub lost_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct OpportunityFilters {
    pub status: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
