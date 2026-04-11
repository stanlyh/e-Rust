use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "activity_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    Call,
    Email,
    Visit,
    Whatsapp,
    Meeting,
    TestDrive,
    Delivery,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "activity_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ActivityStatus {
    Scheduled,
    Completed,
    Cancelled,
    Rescheduled,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ActivityRow {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub r#type: ActivityType,
    pub status: ActivityStatus,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub outcome: Option<String>,
    pub next_action: Option<String>,
    pub assigned_to: Uuid,
    pub client_id: Option<Uuid>,
    pub lead_id: Option<Uuid>,
    pub opportunity_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ActivityResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub status: ActivityStatus,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub outcome: Option<String>,
    pub next_action: Option<String>,
    pub assigned_to: Uuid,
    pub client_id: Option<Uuid>,
    pub lead_id: Option<Uuid>,
    pub opportunity_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl From<ActivityRow> for ActivityResponse {
    fn from(r: ActivityRow) -> Self {
        Self {
            id: r.id,
            title: r.title,
            description: r.description,
            activity_type: r.r#type,
            status: r.status,
            scheduled_start: r.scheduled_start,
            scheduled_end: r.scheduled_end,
            completed_at: r.completed_at,
            outcome: r.outcome,
            next_action: r.next_action,
            assigned_to: r.assigned_to,
            client_id: r.client_id,
            lead_id: r.lead_id,
            opportunity_id: r.opportunity_id,
            vehicle_id: r.vehicle_id,
            created_at: r.created_at,
        }
    }
}

/// Evento para el calendario (formato FullCalendar)
#[derive(Debug, Serialize)]
pub struct CalendarEvent {
    pub id: Uuid,
    pub title: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub status: ActivityStatus,
    #[serde(rename = "extendedProps")]
    pub extended_props: ActivityResponse,
}

#[derive(Debug, Serialize)]
pub struct CalendarResponse {
    pub events: Vec<CalendarEvent>,
    pub overdue_count: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ActivityCreate {
    #[validate(length(min = 2, max = 255))]
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
    pub client_id: Option<Uuid>,
    pub lead_id: Option<Uuid>,
    pub opportunity_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ActivityUpdate {
    #[validate(length(min = 2, max = 255))]
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub activity_type: Option<ActivityType>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub client_id: Option<Uuid>,
    pub lead_id: Option<Uuid>,
    pub opportunity_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ActivityComplete {
    #[validate(length(min = 1, message = "El resultado no puede estar vacio"))]
    pub outcome: String,
    pub next_action: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ActivityReschedule {
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CalendarQuery {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
