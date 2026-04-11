use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ClientRow {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub id_document: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub notes: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ClientResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub id_document: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub notes: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl From<ClientRow> for ClientResponse {
    fn from(r: ClientRow) -> Self {
        let full_name = format!("{} {}", r.first_name, r.last_name);
        Self {
            id: r.id,
            full_name,
            first_name: r.first_name,
            last_name: r.last_name,
            email: r.email,
            phone: r.phone,
            mobile: r.mobile,
            id_document: r.id_document,
            address: r.address,
            city: r.city,
            notes: r.notes,
            assigned_to: r.assigned_to,
            created_at: r.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct ClientCreate {
    #[validate(length(min = 1, max = 100))]
    pub first_name: String,
    #[validate(length(min = 1, max = 100))]
    pub last_name: String,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(max = 30))]
    pub phone: Option<String>,
    #[validate(length(max = 30))]
    pub mobile: Option<String>,
    pub id_document: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ClientUpdate {
    #[validate(length(min = 1, max = 100))]
    pub first_name: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(max = 30))]
    pub phone: Option<String>,
    #[validate(length(max = 30))]
    pub mobile: Option<String>,
    pub id_document: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClientFilters {
    pub search: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
