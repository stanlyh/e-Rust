use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "lead_source", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LeadSource {
    Web,
    Referral,
    WalkIn,
    Phone,
    SocialMedia,
    Other,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "lead_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LeadStatus {
    New,
    Contacted,
    Qualified,
    Unqualified,
    Converted,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LeadRow {
    pub id: Uuid,
    pub client_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub source: LeadSource,
    pub status: LeadStatus,
    pub interest_make: Option<String>,
    pub interest_model: Option<String>,
    pub interest_year: Option<i16>,
    pub budget_min: Option<sqlx::types::BigDecimal>,
    pub budget_max: Option<sqlx::types::BigDecimal>,
    pub notes: Option<String>,
    pub contacted_at: Option<DateTime<Utc>>,
    pub qualified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct LeadResponse {
    pub id: Uuid,
    pub client_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub source: LeadSource,
    pub status: LeadStatus,
    pub interest_make: Option<String>,
    pub interest_model: Option<String>,
    pub interest_year: Option<i16>,
    pub budget_min: Option<String>,
    pub budget_max: Option<String>,
    pub notes: Option<String>,
    pub contacted_at: Option<DateTime<Utc>>,
    pub qualified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<LeadRow> for LeadResponse {
    fn from(r: LeadRow) -> Self {
        Self {
            id: r.id,
            client_id: r.client_id,
            assigned_to: r.assigned_to,
            source: r.source,
            status: r.status,
            interest_make: r.interest_make,
            interest_model: r.interest_model,
            interest_year: r.interest_year,
            budget_min: r.budget_min.map(|v| v.to_string()),
            budget_max: r.budget_max.map(|v| v.to_string()),
            notes: r.notes,
            contacted_at: r.contacted_at,
            qualified_at: r.qualified_at,
            created_at: r.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct LeadCreate {
    pub client_id: Option<Uuid>,
    pub source: LeadSource,
    pub interest_make: Option<String>,
    pub interest_model: Option<String>,
    pub interest_year: Option<i16>,
    pub budget_min: Option<f64>,
    pub budget_max: Option<f64>,
    #[validate(length(max = 2000))]
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LeadUpdate {
    pub source: Option<LeadSource>,
    pub status: Option<LeadStatus>,
    pub interest_make: Option<String>,
    pub interest_model: Option<String>,
    pub interest_year: Option<i16>,
    pub budget_min: Option<f64>,
    pub budget_max: Option<f64>,
    #[validate(length(max = 2000))]
    pub notes: Option<String>,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct LeadFilters {
    pub status: Option<String>,
    pub source: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
