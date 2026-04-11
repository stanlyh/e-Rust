use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::dashboard::{AgentStats, DashboardKPIs, FunnelStep, MonthlySales},
};

pub struct DashboardRepo;

impl DashboardRepo {
    pub async fn kpis(pool: &PgPool, user_id: Uuid, is_manager: bool) -> AppResult<DashboardKPIs> {
        let leads_this_month = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM leads
            WHERE DATE_TRUNC('month', created_at) = DATE_TRUNC('month', NOW())
              AND ($1::bool = TRUE OR assigned_to = $2)
            "#,
            is_manager, user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        let leads_last_month = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM leads
            WHERE DATE_TRUNC('month', created_at) = DATE_TRUNC('month', NOW() - INTERVAL '1 month')
              AND ($1::bool = TRUE OR assigned_to = $2)
            "#,
            is_manager, user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        let leads_growth_pct = if leads_last_month == 0 {
            0.0
        } else {
            ((leads_this_month - leads_last_month) as f64 / leads_last_month as f64) * 100.0
        };

        let opportunities_open = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM opportunities
            WHERE status NOT IN ('closed_won', 'closed_lost')
              AND ($1::bool = TRUE OR assigned_to = $2)
            "#,
            is_manager, user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        let pipeline_value: f64 = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(offered_price::float8), 0.0) FROM opportunities
            WHERE status NOT IN ('closed_won', 'closed_lost')
              AND ($1::bool = TRUE OR assigned_to = $2)
            "#,
            is_manager, user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0.0);

        let revenue_row = sqlx::query!(
            r#"
            SELECT
                COALESCE(SUM(final_price::float8), 0.0) AS revenue,
                COUNT(*) AS sales
            FROM opportunities
            WHERE status = 'closed_won'
              AND DATE_TRUNC('month', closed_at) = DATE_TRUNC('month', NOW())
              AND ($1::bool = TRUE OR assigned_to = $2)
            "#,
            is_manager, user_id
        )
        .fetch_one(pool)
        .await?;

        let revenue_this_month = revenue_row.revenue.unwrap_or(0.0);
        let sales_this_month = revenue_row.sales.unwrap_or(0);

        let total_leads = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM leads WHERE ($1::bool = TRUE OR assigned_to = $2)"#,
            is_manager, user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        let total_won = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM opportunities WHERE status = 'closed_won'
               AND ($1::bool = TRUE OR assigned_to = $2)"#,
            is_manager, user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        let conversion_rate = if total_leads == 0 {
            0.0
        } else {
            (total_won as f64 / total_leads as f64) * 100.0
        };

        let avg_deal_days: f64 = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(AVG(EXTRACT(EPOCH FROM (closed_at - created_at)) / 86400), 0.0)
            FROM opportunities
            WHERE status = 'closed_won'
              AND ($1::bool = TRUE OR assigned_to = $2)
            "#,
            is_manager, user_id
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0.0);

        Ok(DashboardKPIs {
            leads_this_month,
            leads_growth_pct,
            opportunities_open,
            pipeline_value,
            revenue_this_month,
            sales_this_month,
            conversion_rate,
            avg_deal_days,
        })
    }

    pub async fn monthly_sales(pool: &PgPool, user_id: Uuid, is_manager: bool) -> AppResult<Vec<MonthlySales>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                TO_CHAR(DATE_TRUNC('month', closed_at), 'YYYY-MM') AS month,
                COALESCE(SUM(final_price::float8), 0.0)            AS revenue,
                COUNT(*)                                            AS count
            FROM opportunities
            WHERE status = 'closed_won'
              AND closed_at >= NOW() - INTERVAL '12 months'
              AND ($1::bool = TRUE OR assigned_to = $2)
            GROUP BY DATE_TRUNC('month', closed_at)
            ORDER BY DATE_TRUNC('month', closed_at) ASC
            "#,
            is_manager, user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| MonthlySales {
                month: r.month.unwrap_or_default(),
                revenue: r.revenue.unwrap_or(0.0),
                count: r.count.unwrap_or(0),
            })
            .collect())
    }

    pub async fn funnel(pool: &PgPool, user_id: Uuid, is_manager: bool) -> AppResult<Vec<FunnelStep>> {
        let stages = vec![
            ("leads_total",    "Leads totales"),
            ("contacted",      "Contactados"),
            ("qualified",      "Calificados"),
            ("opportunity",    "Oportunidad abierta"),
            ("closed_won",     "Ventas cerradas"),
        ];

        let leads_total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM leads WHERE ($1::bool = TRUE OR assigned_to = $2)",
            is_manager, user_id
        ).fetch_one(pool).await?.unwrap_or(0);

        let contacted = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM leads WHERE status != 'new' AND ($1::bool = TRUE OR assigned_to = $2)",
            is_manager, user_id
        ).fetch_one(pool).await?.unwrap_or(0);

        let qualified = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM leads WHERE status IN ('qualified','converted') AND ($1::bool = TRUE OR assigned_to = $2)",
            is_manager, user_id
        ).fetch_one(pool).await?.unwrap_or(0);

        let opportunity = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM opportunities WHERE ($1::bool = TRUE OR assigned_to = $2)",
            is_manager, user_id
        ).fetch_one(pool).await?.unwrap_or(0);

        let closed_won = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM opportunities WHERE status = 'closed_won' AND ($1::bool = TRUE OR assigned_to = $2)",
            is_manager, user_id
        ).fetch_one(pool).await?.unwrap_or(0);

        let counts = vec![leads_total, contacted, qualified, opportunity, closed_won];

        Ok(stages
            .into_iter()
            .zip(counts)
            .map(|((stage, label), count)| FunnelStep {
                stage: stage.to_string(),
                label: label.to_string(),
                count,
                value: 0.0,
            })
            .collect())
    }

    pub async fn agent_stats(pool: &PgPool) -> AppResult<Vec<AgentStats>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                u.id::text                                                       AS user_id,
                u.full_name,
                COUNT(DISTINCT l.id)                                            AS leads,
                COUNT(DISTINCT o.id) FILTER (WHERE o.status NOT IN ('closed_won','closed_lost')) AS opportunities_open,
                COUNT(DISTINCT o.id) FILTER (
                    WHERE o.status = 'closed_won'
                    AND DATE_TRUNC('month', o.closed_at) = DATE_TRUNC('month', NOW())
                )                                                               AS sales_this_month,
                COALESCE(SUM(o.final_price::float8) FILTER (
                    WHERE o.status = 'closed_won'
                    AND DATE_TRUNC('month', o.closed_at) = DATE_TRUNC('month', NOW())
                ), 0.0)                                                         AS revenue_this_month
            FROM users u
            LEFT JOIN leads l        ON l.assigned_to = u.id
            LEFT JOIN opportunities o ON o.assigned_to = u.id
            WHERE u.is_active = TRUE AND u.role = 'sales_agent'
            GROUP BY u.id, u.full_name
            ORDER BY revenue_this_month DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| AgentStats {
                user_id: r.user_id.unwrap_or_default(),
                full_name: r.full_name,
                leads: r.leads.unwrap_or(0),
                opportunities_open: r.opportunities_open.unwrap_or(0),
                sales_this_month: r.sales_this_month.unwrap_or(0),
                revenue_this_month: r.revenue_this_month.unwrap_or(0.0),
            })
            .collect())
    }
}
