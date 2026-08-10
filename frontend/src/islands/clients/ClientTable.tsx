import { useEffect, useState } from 'react';
import { clientsApi, type Client } from '../../lib/api/clients';
import { TableSkeleton } from '../ui/Skeleton';
import { ErrorState } from '../ui/ErrorState';
import { EmptyState } from '../ui/EmptyState';
import { Pagination } from '../ui/Pagination';
import { toast } from '../../lib/toast';

const PER_PAGE = 20;

export default function ClientTable() {
  const [clients, setClients] = useState<Client[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState('');
  const [searchInput, setSearchInput] = useState('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  const load = async (p = 1) => {
    setLoading(true);
    setError('');
    try {
      const params: Record<string, string> = { page: String(p), per_page: String(PER_PAGE) };
      if (search) params.search = search;
      const res = await clientsApi.list(params);
      setClients(res.data);
      setTotal(res.total);
      setPage(p);
    } catch {
      setError('No se pudieron cargar los clientes.');
      toast.error('Error cargando clientes');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { load(1); }, [search]);

  // Debounce busqueda 400ms
  useEffect(() => {
    const t = setTimeout(() => setSearch(searchInput), 400);
    return () => clearTimeout(t);
  }, [searchInput]);

  return (
    <div className="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 shadow-sm">
      <div className="flex flex-wrap items-center justify-between gap-3 p-4 border-b border-gray-100 dark:border-gray-700">
        <div className="flex items-center gap-3">
          <div className="relative">
            <span className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 dark:text-gray-500 text-xs">🔍</span>
            <input
              value={searchInput}
              onChange={e => setSearchInput(e.target.value)}
              placeholder="Buscar por nombre o email..."
              className="border border-gray-300 dark:border-gray-600 dark:bg-gray-900 dark:text-gray-100 rounded-lg pl-8 pr-3 py-1.5 text-sm w-56"
            />
          </div>
          <span className="text-sm text-gray-400 dark:text-gray-500">{total} clientes</span>
        </div>
        <a href="/clients/new" className="bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium px-4 py-1.5 rounded-lg whitespace-nowrap">
          + Nuevo Cliente
        </a>
      </div>

      {error ? (
        <ErrorState message={error} onRetry={() => load(page)} />
      ) : (
        <>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-gray-100 dark:border-gray-700 text-left text-xs text-gray-500 dark:text-gray-400 uppercase">
                  <th className="px-4 py-3">Nombre</th>
                  <th className="px-4 py-3 hidden sm:table-cell">Email</th>
                  <th className="px-4 py-3 hidden md:table-cell">Telefono</th>
                  <th className="px-4 py-3 hidden lg:table-cell">Ciudad</th>
                  <th className="px-4 py-3"></th>
                </tr>
              </thead>
              {loading ? (
                <TableSkeleton rows={8} cols={5} />
              ) : clients.length === 0 ? (
                <tbody><tr><td colSpan={5}>
                  <EmptyState
                    icon="👥"
                    title={search ? 'Sin resultados' : 'No hay clientes aun'}
                    description={search
                      ? `No se encontraron clientes para "${search}"`
                      : 'Registra tu primer cliente para empezar.'}
                    action={!search ? { label: '+ Nuevo Cliente', href: '/clients/new' } : undefined}
                  />
                </td></tr></tbody>
              ) : (
                <tbody className="divide-y divide-gray-50 dark:divide-gray-700">
                  {clients.map(c => (
                    <tr key={c.id} className="hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors">
                      <td className="px-4 py-3 font-medium text-gray-800 dark:text-gray-200">{c.full_name}</td>
                      <td className="px-4 py-3 text-gray-600 dark:text-gray-400 hidden sm:table-cell">{c.email ?? '—'}</td>
                      <td className="px-4 py-3 text-gray-600 dark:text-gray-400 hidden md:table-cell">{c.phone ?? c.mobile ?? '—'}</td>
                      <td className="px-4 py-3 text-gray-400 dark:text-gray-500 hidden lg:table-cell">{c.city ?? '—'}</td>
                      <td className="px-4 py-3">
                        <a href={`/clients/${c.id}`} className="text-blue-600 dark:text-blue-400 hover:underline text-xs font-medium">Ver →</a>
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
