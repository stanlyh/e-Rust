use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::vehicle::{VehicleCreate, VehicleFilters, VehicleRow, VehicleUpdate},
};

pub struct VehicleRepo;

impl VehicleRepo {
    pub async fn list(
        pool: &PgPool,
        filters: &VehicleFilters,
    ) -> AppResult<(Vec<VehicleRow>, i64)> {
        let page = filters.page.unwrap_or(1).max(1);
        let per_page = filters.per_page.unwrap_or(20).min(100);
        let offset = (page - 1) * per_page;

        let rows = sqlx::query_as!(
            VehicleRow,
            r#"
            SELECT id, vin, stock_number, make, model, year, trim,
                   color_exterior, color_interior,
                   fuel_type AS "fuel_type: _",
                   transmission AS "transmission: _",
                   mileage,
                   condition AS "condition: _",
                   list_price, cost_price, is_available,
                   description, images, features,
                   created_at, updated_at
            FROM vehicles
            WHERE ($1::text IS NULL OR make ILIKE $1)
              AND ($2::text IS NULL OR model ILIKE $2)
              AND ($3::smallint IS NULL OR year = $3)
              AND ($4::bool IS NULL OR is_available = $4)
              AND ($5::float8 IS NULL OR list_price >= $5)
              AND ($6::float8 IS NULL OR list_price <= $6)
            ORDER BY created_at DESC
            LIMIT $7 OFFSET $8
            "#,
            filters.make.as_deref().map(|m| format!("%{m}%")),
            filters.model.as_deref().map(|m| format!("%{m}%")),
            filters.year,
            filters.available_only,
            filters.min_price,
            filters.max_price,
            per_page,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM vehicles
            WHERE ($1::text IS NULL OR make ILIKE $1)
              AND ($2::text IS NULL OR model ILIKE $2)
              AND ($3::smallint IS NULL OR year = $3)
              AND ($4::bool IS NULL OR is_available = $4)
              AND ($5::float8 IS NULL OR list_price >= $5)
              AND ($6::float8 IS NULL OR list_price <= $6)
            "#,
            filters.make.as_deref().map(|m| format!("%{m}%")),
            filters.model.as_deref().map(|m| format!("%{m}%")),
            filters.year,
            filters.available_only,
            filters.min_price,
            filters.max_price,
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        Ok((rows, total))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<VehicleRow>> {
        let row = sqlx::query_as!(
            VehicleRow,
            r#"
            SELECT id, vin, stock_number, make, model, year, trim,
                   color_exterior, color_interior,
                   fuel_type AS "fuel_type: _",
                   transmission AS "transmission: _",
                   mileage,
                   condition AS "condition: _",
                   list_price, cost_price, is_available,
                   description, images, features,
                   created_at, updated_at
            FROM vehicles WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn create(pool: &PgPool, req: &VehicleCreate) -> AppResult<VehicleRow> {
        let fuel = req.fuel_type.as_ref()
            .unwrap_or(&crate::models::vehicle::FuelType::Gasoline);
        let trans = req.transmission.as_ref()
            .unwrap_or(&crate::models::vehicle::TransmissionType::Automatic);
        let cond = req.condition.as_ref()
            .unwrap_or(&crate::models::vehicle::VehicleCondition::New);

        let row = sqlx::query_as!(
            VehicleRow,
            r#"
            INSERT INTO vehicles
                (vin, stock_number, make, model, year, trim,
                 color_exterior, color_interior,
                 fuel_type, transmission, mileage, condition,
                 list_price, description, images, features)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16)
            RETURNING id, vin, stock_number, make, model, year, trim,
                      color_exterior, color_interior,
                      fuel_type AS "fuel_type: _",
                      transmission AS "transmission: _",
                      mileage,
                      condition AS "condition: _",
                      list_price, cost_price, is_available,
                      description, images, features,
                      created_at, updated_at
            "#,
            req.vin, req.stock_number, req.make, req.model, req.year, req.trim,
            req.color_exterior, req.color_interior,
            fuel as _, trans as _, req.mileage.unwrap_or(0), cond as _,
            req.list_price as f64,
            req.description,
            req.images.clone().unwrap_or(serde_json::json!([])),
            req.features.clone().unwrap_or(serde_json::json!({})),
        )
        .fetch_one(pool)
        .await?;
        Ok(row)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        req: &VehicleUpdate,
    ) -> AppResult<Option<VehicleRow>> {
        let row = sqlx::query_as!(
            VehicleRow,
            r#"
            UPDATE vehicles SET
                vin            = COALESCE($2, vin),
                stock_number   = COALESCE($3, stock_number),
                make           = COALESCE($4, make),
                model          = COALESCE($5, model),
                year           = COALESCE($6, year),
                trim           = COALESCE($7, trim),
                color_exterior = COALESCE($8, color_exterior),
                color_interior = COALESCE($9, color_interior),
                fuel_type      = COALESCE($10, fuel_type),
                transmission   = COALESCE($11, transmission),
                mileage        = COALESCE($12, mileage),
                condition      = COALESCE($13, condition),
                list_price     = COALESCE($14, list_price),
                description    = COALESCE($15, description),
                updated_at     = NOW()
            WHERE id = $1
            RETURNING id, vin, stock_number, make, model, year, trim,
                      color_exterior, color_interior,
                      fuel_type AS "fuel_type: _",
                      transmission AS "transmission: _",
                      mileage, condition AS "condition: _",
                      list_price, cost_price, is_available,
                      description, images, features,
                      created_at, updated_at
            "#,
            id,
            req.vin, req.stock_number, req.make, req.model, req.year, req.trim,
            req.color_exterior, req.color_interior,
            req.fuel_type as _,
            req.transmission as _,
            req.mileage,
            req.condition as _,
            req.list_price.map(|v| sqlx::types::BigDecimal::from(v as i64)),
            req.description,
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn set_availability(
        pool: &PgPool,
        id: Uuid,
        available: bool,
    ) -> AppResult<Option<VehicleRow>> {
        let row = sqlx::query_as!(
            VehicleRow,
            r#"
            UPDATE vehicles SET is_available = $2 WHERE id = $1
            RETURNING id, vin, stock_number, make, model, year, trim,
                      color_exterior, color_interior,
                      fuel_type AS "fuel_type: _",
                      transmission AS "transmission: _",
                      mileage, condition AS "condition: _",
                      list_price, cost_price, is_available,
                      description, images, features,
                      created_at, updated_at
            "#,
            id, available
        )
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<bool> {
        let result = sqlx::query!("DELETE FROM vehicles WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
