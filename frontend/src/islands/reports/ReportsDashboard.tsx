import { useEffect, useState } from 'react';
import {
  BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer,
  FunnelChart, Funnel, LabelList, Cell,
} from 'recharts';
import { dashboardApi, type DashboardReport } from '../../lib/api/dashboard';

// ── KPI Card ─────────────────────────────────────────────────────────────────
function KPICard({
  title, value, subtitle, trend, format = 'number',
}: {
  title: string;
  value: number;
  subtitle?: string;
  trend?: number;
  format?: 'number' | 'currency' | 'percent' | 'days';
}) {
  const formatted =
    format === 'currency' ? `$${value.toLocaleString('es-MX', { maximumFractionDigits: 0 })}`
    : format === 'percent' ? `${value.toFixed(1)}%`
    : format === 'days'    ? `${value.toFixed(0)} dias`
    : value.toLocaleString('es-MX');

  return (
    <div className="bg-white rounded-xl border border-gray-200 shadow-sm p-5">
      <p className="text-xs text-gray-500 font-medium mb-1">{title}</p>
      <p className="text-2xl font-bold text-gray-900">{formatted}</p>
      {trend !== undefined && (
        <p className={`text-xs mt-1 font-medium ${trend >= 0 ? 'text-green-600' : 'text-red-500'}`}>
          {trend >= 0 ? '▲' : '▼'} {Math.abs(trend).toFixed(1)}% vs mes anterior
        </p>
      )}
      {subtitle && <p className="text-xs text-gray-400 mt-1">{subtitle}</p>}
    </div>
  );
}

// ── Grafico de ventas mensuales ───────────────────────────────────────────────
function SalesChart({ data }: { data: { month: string; revenue: number; count: number }[] }) {
  const formatted = data.map(d => ({
    ...d,
    label: d.month.slice(5), // "04" de "2026-04"
  }));

  return (
    <div className="bg-white rounded-xl border border-gray-200 shadow-sm p-5">
      <h2 className="text-sm font-semibold text-gray-700 mb-4">Ventas por mes (ultimos 12 meses)</h2>
      {formatted.length === 0 ? (
        <p className="text-sm text-gray-400 text-center py-8">Sin datos de ventas aun</p>
      ) : (
        <ResponsiveContainer width="100%" height={220}>
          <BarChart data={formatted} margin={{ top: 4, right: 8, left: 0, bottom: 0 }}>
            <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
            <XAxis dataKey="label" tick={{ fontSize: 11 }} />
            <YAxis tickFormatter={v => `$${(v / 1000).toFixed(0)}k`} tick={{ fontSize: 11 }} />
            <Tooltip
              formatter={(value: number) => [`$${value.toLocaleString('es-MX')}`, 'Revenue']}
              labelFormatter={label => `Mes ${label}`}
            />
            <Bar dataKey="revenue" fill="#3b82f6" radius={[4, 4, 0, 0]} />
          </BarChart>
        </ResponsiveContainer>
      )}
    </div>
  );
}

// ── Funnel de conversion ──────────────────────────────────────────────────────
const FUNNEL_COLORS = ['#3b82f6', '#6366f1', '#8b5cf6', '#a855f7', '#22c55e'];

function ConversionFunnel({ data }: { data: { label: string; count: number }[] }) {
  return (
    <div className="bg-white rounded-xl border border-gray-200 shadow-sm p-5">
      <h2 className="text-sm font-semibold text-gray-700 mb-4">Funnel de conversion</h2>
      <div className="space-y-2">
        {data.map((step, i) => {
          const maxCount = data[0]?.count ?? 1;
          const pct = maxCount > 0 ? (step.count / maxCount) * 100 : 0;
          return (
            <div key={step.label}>
              <div className="flex justify-between text-xs text-gray-600 mb-1">
                <span>{step.label}</span>
                <span className="font-medium">{step.count}</span>
              </div>
              <div className="h-6 bg-gray-100 rounded-full overflow-hidden">
                <div
                  className="h-full rounded-full flex items-center pl-2 transition-all"
                  style={{ width: `${Math.max(pct, 2)}%`, backgroundColor: FUNNEL_COLORS[i] ?? '#6b7280' }}
                >
                  {pct > 15 && (
                    <span className="text-white text-xs font-medium">{pct.toFixed(0)}%</span>
                  )}
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}

// ── Tabla de agentes ──────────────────────────────────────────────────────────
function AgentsTable({ agents }: { agents: DashboardReport['agents'] }) {
  return (
    <div className="bg-white rounded-xl border border-gray-200 shadow-sm">
      <div className="p-4 border-b border-gray-100">
        <h2 className="text-sm font-semibold text-gray-700">Performance por agente</h2>
      </div>
      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead>
            <tr className="text-left text-xs text-gray-500 uppercase border-b border-gray-100">
              <th className="px-4 py-3">Agente</th>
              <th className="px-4 py-3">Leads</th>
              <th className="px-4 py-3">Opps. abiertas</th>
              <th className="px-4 py-3">Ventas mes</th>
              <th className="px-4 py-3">Revenue mes</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-50">
            {agents.length === 0 ? (
              <tr>
                <td colSpan={5} className="px-4 py-8 text-center text-gray-400">Sin datos de agentes</td>
              </tr>
            ) : agents.map(a => (
              <tr key={a.user_id} className="hover:bg-gray-50">
                <td className="px-4 py-3 font-medium text-gray-800">{a.full_name}</td>
                <td className="px-4 py-3 text-gray-600">{a.leads}</td>
                <td className="px-4 py-3 text-gray-600">{a.opportunities_open}</td>
                <td className="px-4 py-3 text-gray-600">{a.sales_this_month}</td>
                <td className="px-4 py-3 font-semibold text-blue-600">
                  ${a.revenue_this_month.toLocaleString('es-MX', { maximumFractionDigits: 0 })}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

// ── Dashboard principal ───────────────────────────────────────────────────────
export default function ReportsDashboard() {
  const [report, setReport] = useState<DashboardReport | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    dashboardApi.report()
      .then(setReport)
      .catch(() => setError('Error cargando los reportes'))
      .finally(() => setLoading(false));
  }, []);

  if (loading) return (
    <div className="space-y-4">
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
        {Array.from({ length: 4 }).map((_, i) => (
          <div key={i} className="bg-white rounded-xl border border-gray-200 p-5 animate-pulse">
            <div className="h-3 bg-gray-100 rounded mb-3 w-1/2" />
            <div className="h-7 bg-gray-100 rounded" />
          </div>
        ))}
      </div>
      <div className="h-64 bg-white rounded-xl border border-gray-200 animate-pulse" />
    </div>
  );

  if (error || !report) return <p className="text-red-500 text-sm">{error}</p>;

  const { kpis, monthly_sales, funnel, agents } = report;

  return (
    <div className="space-y-6">
      {/* KPIs */}
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
        <KPICard
          title="Leads este mes"
          value={kpis.leads_this_month}
          trend={kpis.leads_growth_pct}
        />
        <KPICard
          title="Oportunidades abiertas"
          value={kpis.opportunities_open}
          subtitle={`$${kpis.pipeline_value.toLocaleString('es-MX', { maximumFractionDigits: 0 })} en pipeline`}
        />
        <KPICard
          title="Revenue del mes"
          value={kpis.revenue_this_month}
          format="currency"
          subtitle={`${kpis.sales_this_month} ventas cerradas`}
        />
        <KPICard
          title="Tasa de conversion"
          value={kpis.conversion_rate}
          format="percent"
          subtitle={`Promedio ${kpis.avg_deal_days.toFixed(0)} dias por venta`}
        />
      </div>

      {/* Graficos */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <div className="lg:col-span-2">
          <SalesChart data={monthly_sales} />
        </div>
        <ConversionFunnel data={funnel} />
      </div>

      {/* Tabla de agentes */}
      <AgentsTable agents={agents} />
    </div>
  );
}
