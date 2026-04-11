import { useEffect, useState } from 'react';
import { leadsApi, SOURCE_LABELS, STATUS_COLORS, STATUS_LABELS, type Lead } from '../../lib/api/leads';

export default function LeadTable() {
  const [leads, setLeads] = useState<Lead[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [loading, setLoading] = useState(true);
  const [statusFilter, setStatusFilter] = useState('');

  const load = async (p = 1) => {
    setLoading(true);
    try {
      const params: Record<string, string> = { page: String(p), per_page: '20' };
      if (statusFilter) params.status = statusFilter;
      const res = await leadsApi.list(params);
      setLeads(res.data);
      setTotal(res.total);
      setPage(p);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { load(1); }, [statusFilter]);

  return (
    <div className="bg-white rounded-xl border border-gray-200 shadow-sm">
      {/* Toolbar */}
      <div className="flex items-center justify-between p-4 border-b border-gray-100">
        <div className="flex items-center gap-3">
          <select
            value={statusFilter}
            onChange={e => setStatusFilter(e.target.value)}
            className="border border-gray-300 rounded-lg px-3 py-1.5 text-sm"
          >
            <option value="">Todos los estados</option>
            {Object.entries(STATUS_LABELS).map(([v, l]) => (
              <option key={v} value={v}>{l}</option>
            ))}
          </select>
          <span className="text-sm text-gray-500">{total} leads</span>
        </div>
        <a
          href="/leads/new"
          className="bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium px-4 py-1.5 rounded-lg"
        >
          + Nuevo Lead
        </a>
      </div>

      {/* Tabla */}
      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-gray-100 text-left text-xs text-gray-500 uppercase">
              <th className="px-4 py-3">Estado</th>
              <th className="px-4 py-3">Fuente</th>
              <th className="px-4 py-3">Interes</th>
              <th className="px-4 py-3">Presupuesto</th>
              <th className="px-4 py-3">Fecha</th>
              <th className="px-4 py-3"></th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-50">
            {loading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <tr key={i}>
                  {Array.from({ length: 6 }).map((_, j) => (
                    <td key={j} className="px-4 py-3">
                      <div className="h-4 bg-gray-100 rounded animate-pulse" />
                    </td>
                  ))}
                </tr>
              ))
            ) : leads.length === 0 ? (
              <tr>
                <td colSpan={6} className="px-4 py-12 text-center text-gray-400">
                  No hay leads que mostrar
                </td>
              </tr>
            ) : (
              leads.map(lead => (
                <tr key={lead.id} className="hover:bg-gray-50">
                  <td className="px-4 py-3">
                    <span className={`text-xs px-2 py-1 rounded-full font-medium ${STATUS_COLORS[lead.status]}`}>
                      {STATUS_LABELS[lead.status]}
                    </span>
                  </td>
                  <td className="px-4 py-3 text-gray-600">{SOURCE_LABELS[lead.source]}</td>
                  <td className="px-4 py-3 text-gray-800">
                    {[lead.interest_year, lead.interest_make, lead.interest_model]
                      .filter(Boolean).join(' ') || '—'}
                  </td>
                  <td className="px-4 py-3 text-gray-600">
                    {lead.budget_min || lead.budget_max
                      ? `$${lead.budget_min ?? '?'} – $${lead.budget_max ?? '?'}`
                      : '—'}
                  </td>
                  <td className="px-4 py-3 text-gray-500">
                    {new Date(lead.created_at).toLocaleDateString('es-MX')}
                  </td>
                  <td className="px-4 py-3">
                    <a href={`/leads/${lead.id}`} className="text-blue-600 hover:underline text-xs">
                      Ver
                    </a>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Paginacion */}
      {total > 20 && (
        <div className="flex items-center justify-between p-4 border-t border-gray-100">
          <button
            onClick={() => load(page - 1)}
            disabled={page === 1}
            className="text-sm text-gray-600 disabled:opacity-40 hover:text-blue-600"
          >
            ← Anterior
          </button>
          <span className="text-sm text-gray-500">Pagina {page}</span>
          <button
            onClick={() => load(page + 1)}
            disabled={page * 20 >= total}
            className="text-sm text-gray-600 disabled:opacity-40 hover:text-blue-600"
          >
            Siguiente →
          </button>
        </div>
      )}
    </div>
  );
}
