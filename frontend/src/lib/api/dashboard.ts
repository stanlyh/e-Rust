import { api } from './client';

export interface DashboardKPIs {
  leads_this_month: number;
  leads_growth_pct: number;
  opportunities_open: number;
  pipeline_value: number;
  revenue_this_month: number;
  sales_this_month: number;
  conversion_rate: number;
  avg_deal_days: number;
}

export interface MonthlySales {
  month: string;       // "2026-03"
  revenue: number;
  count: number;
}

export interface FunnelStep {
  stage: string;
  label: string;
  count: number;
  value: number;
}

export interface AgentStats {
  user_id: string;
  full_name: string;
  leads: number;
  opportunities_open: number;
  sales_this_month: number;
  revenue_this_month: number;
}

export interface DashboardReport {
  kpis: DashboardKPIs;
  monthly_sales: MonthlySales[];
  funnel: FunnelStep[];
  agents: AgentStats[];
}

export const dashboardApi = {
  report: () => api.get<DashboardReport>('/api/dashboard'),
};
