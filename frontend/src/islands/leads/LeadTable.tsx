import { useEffect, useState } from 'react';
import { leadsApi, SOURCE_LABELS, STATUS_COLORS, STATUS_LABELS, type Lead } from '../../lib/api/leads';
import { TableSkeleton } from '../ui/Skeleton';
import { ErrorState } from '../ui/ErrorState';
import { EmptyState } from '../ui/EmptyState';
import { Pagination } from '../ui/Pagination';
import { toast } from '../../lib/toast';

const PER_PAGE = 20;

export default function LeadTable() {
  const [leads, setLeads] = useState<Lead[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  // Filtros
  const [statusFilter, setStatusFilter] = useState('');
  const [sourceFilter, setSourceFilter] = useState('');

  const load = async (p = 1) => {
    setLoading(true);
    setError('');
    try {
      const params: Record<string, string> = { page: String(p), per_page: String(PER_PAGE) };
      if (statusFilter) params.status = statusFilter;
      if (sourceFilter) params.source = sourceFilter;
      const res = await leadsApi.list(params);
      setLeads(res.data);
      setTotal(res.total);
      setPage(p);
    } catch {
      setError('No se pudieron cargar los leads.');
      toast.error('Error cargando leads');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { load(1); }, [statusFilter, sourceFilter]);

  return (
    <div className="bg-white rounded-xl border border-gray-200 shadow-sm">
      {/* Toolbar */}
      <div className="flex flex-wrap items-center justify-between gap-3 p-4 border-b border-gray-100">
        <div className="flex flex-wrap items-center gap-2">
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

          <select
            value={sourceFilter}
            onChange={e => setSourceFilter(e.target.value)}
            className="border border-gray-300 rounded-lg px-3 py-1.5 text-sm"
          >
            <option value="">Todas las fuentes</option>
            {Object.entries(SOURCE_LABELS).map(([v, l]) => (
              <option key={v} value={v}>{l}</option>
            ))}
          </select>

          {(statusFilter || sourceFilter) && (
            <button
              onClick={() => { setStatusFilter(''); setSourceFilter(''); }}
              className="text-xs text-gray-500 hover:text-blue-600 underline"
            >
              Limpiar filtros
            </button>
          )}

          <span className="text-sm text-gray-400">{total} leads</span>
        </div>

        <a
          href="/leads/new"
          className="bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium px-4 py-1.5 rounded-lg whitespace-nowrap"
        >
          + Nuevo Lead
        </a>
      </div>

      {/* Contenido */}
      {error ? (
        <ErrorState message={error} onRetry={() => load(page)} />
      ) : (
        <>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-gray-100 text-left text-xs text-gray-500 uppercase">
                  <th className="px-4 py-3">Estado</th>
                  <th className="px-4 py-3">Fuente</th>
                  <th className="px-4 py-3">Interes</th>
                  <th className="px-4 py-3 hidden sm:table-cell">Presupuesto</th>
                  <th className="px-4 py-3 hidden md:table-cell">Fecha</th>
                  <th className="px-4 py-3"></th>
                </tr>
              </thead>
              {loading ? (
                <TableSkeleton rows={8} cols={6} />
              ) : leads.length === 0 ? (
                <tbody>
                  <tr>
                    <td colSpan={6}>
                      <EmptyState
                        icon="🎯"
                        title="No hay leads"
                        description={statusFilter || sourceFilter
                          ? 'Prueba cambiando los filtros de busqueda.'
                          : 'Crea tu primer lead para empezar a hacer seguimiento.'}
                        action={{ label: '+ Nuevo Lead', href: '/leads/new' }}
                      />
                    </td>
                  </tr>
                </tbody>
              ) : (
                <tbody className="divide-y divide-gray-50">
                  {leads.map(lead => (
                    <tr key={lead.id} className="hover:bg-gray-50 transition-colors">
                      <td className="px-4 py-3">
                        <span className={`text-xs px-2 py-1 rounded-full font-medium ${STATUS_COLORS[lead.status]}`}>
                          {STATUS_LABELS[lead.status]}
                        </span>
                      </td>
                      <td className="px-4 py-3 text-gray-600">{SOURCE_LABELS[lead.source]}</td>
                      <td className="px-4 py-3 text-gray-800">
                        {[lead.interest_year, lead.interest_make, lead.interest_model].filter(Boolean).join(' ') || '—'}
                      </td>
                      <td className="px-4 py-3 text-gray-600 hidden sm:table-cell">
                        {lead.budget_min || lead.budget_max
                          ? `$${lead.budget_min ?? '?'} – $${lead.budget_max ?? '?'}`
                          : '—'}
                      </td>
                      <td className="px-4 py-3 text-gray-400 text-xs hidden md:table-cell">
                        {new Date(lead.created_at).toLocaleDateString('es-MX')}
                      </td>
                      <td className="px-4 py-3">
                        <a href={`/leads/${lead.id}`} className="text-blue-600 hover:underline text-xs font-medium">
                          Ver →
                        </a>
                      </td>
                    </tr>
                  ))}
                </tbody>
              )}
            </table>
          </div>
          <Pagination page={page} total={total} perPage={PER_PAGE} onPage={load} />
        </>
      )}
    </div>
  );
}
