use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DashboardKPIs {
    /// Leads creados en el mes actual
    pub leads_this_month: i64,
    /// Crecimiento vs mes anterior (porcentaje)
    pub leads_growth_pct: f64,
    /// Oportunidades abiertas (no cerradas)
    pub opportunities_open: i64,
    /// Valor total del pipeline abierto
    pub pipeline_value: f64,
    /// Ventas cerradas este mes (closed_won)
    pub revenue_this_month: f64,
    /// Numero de ventas cerradas este mes
    pub sales_this_month: i64,
    /// Tasa de conversion lead -> closed_won (%)
    pub conversion_rate: f64,
    /// Promedio de dias desde creacion hasta cierre
    pub avg_deal_days: f64,
}

#[derive(Debug, Serialize)]
pub struct MonthlySales {
    pub month: String,
    pub revenue: f64,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct FunnelStep {
    pub stage: String,
    pub label: String,
    pub count: i64,
    pub value: f64,
}

#[derive(Debug, Serialize)]
pub struct AgentStats {
    pub user_id: String,
    pub full_name: String,
    pub leads: i64,
    pub opportunities_open: i64,
    pub sales_this_month: i64,
    pub revenue_this_month: f64,
}

#[derive(Debug, Serialize)]
pub struct DashboardReport {
    pub kpis: DashboardKPIs,
    pub monthly_sales: Vec<MonthlySales>,
    pub funnel: Vec<FunnelStep>,
    pub agents: Vec<AgentStats>,
}
