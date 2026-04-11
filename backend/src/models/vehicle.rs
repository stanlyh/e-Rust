use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "fuel_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum FuelType {
    Gasoline,
    Diesel,
    Hybrid,
    Electric,
    Other,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "transmission_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TransmissionType {
    Manual,
    Automatic,
    Cvt,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "vehicle_condition", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum VehicleCondition {
    New,
    Used,
    CertifiedUsed,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct VehicleRow {
    pub id: Uuid,
    pub vin: Option<String>,
    pub stock_number: Option<String>,
    pub make: String,
    pub model: String,
    pub year: i16,
    pub trim: Option<String>,
    pub color_exterior: Option<String>,
    pub color_interior: Option<String>,
    pub fuel_type: FuelType,
    pub transmission: TransmissionType,
    pub mileage: i32,
    pub condition: VehicleCondition,
    pub list_price: sqlx::types::BigDecimal,
    pub cost_price: Option<sqlx::types::BigDecimal>,
    pub is_available: bool,
    pub description: Option<String>,
    pub images: Value,
    pub features: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct VehicleResponse {
    pub id: Uuid,
    pub vin: Option<String>,
    pub stock_number: Option<String>,
    pub make: String,
    pub model: String,
    pub year: i16,
    pub trim: Option<String>,
    pub color_exterior: Option<String>,
    pub color_interior: Option<String>,
    pub fuel_type: FuelType,
    pub transmission: TransmissionType,
    pub mileage: i32,
    pub condition: VehicleCondition,
    pub list_price: String,
    pub is_available: bool,
    pub description: Option<String>,
    pub images: Value,
    pub features: Value,
    pub created_at: DateTime<Utc>,
}

impl From<VehicleRow> for VehicleResponse {
    fn from(r: VehicleRow) -> Self {
        Self {
            id: r.id,
            vin: r.vin,
            stock_number: r.stock_number,
            make: r.make,
            model: r.model,
            year: r.year,
            trim: r.trim,
            color_exterior: r.color_exterior,
            color_interior: r.color_interior,
            fuel_type: r.fuel_type,
            transmission: r.transmission,
            mileage: r.mileage,
            condition: r.condition,
            list_price: r.list_price.to_string(),
            is_available: r.is_available,
            description: r.description,
            images: r.images,
            features: r.features,
            created_at: r.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct VehicleCreate {
    pub vin: Option<String>,
    pub stock_number: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub make: String,
    #[validate(length(min = 1, max = 100))]
    pub model: String,
    pub year: i16,
    pub trim: Option<String>,
    pub color_exterior: Option<String>,
    pub color_interior: Option<String>,
    pub fuel_type: Option<FuelType>,
    pub transmission: Option<TransmissionType>,
    pub mileage: Option<i32>,
    pub condition: Option<VehicleCondition>,
    #[validate(range(min = 0.0))]
    pub list_price: f64,
    pub description: Option<String>,
    pub images: Option<Value>,
    pub features: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct VehicleFilters {
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<i16>,
    pub condition: Option<String>,
    pub available_only: Option<bool>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
