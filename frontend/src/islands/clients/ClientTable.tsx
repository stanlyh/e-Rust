import { useEffect, useState } from 'react';
import { clientsApi, type Client } from '../../lib/api/clients';

export default function ClientTable() {
  const [clients, setClients] = useState<Client[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState('');
  const [loading, setLoading] = useState(true);

  const load = async (p = 1) => {
    setLoading(true);
    try {
      const params: Record<string, string> = { page: String(p), per_page: '20' };
      if (search) params.search = search;
      const res = await clientsApi.list(params);
      setClients(res.data);
      setTotal(res.total);
      setPage(p);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { load(1); }, [search]);

  return (
    <div className="bg-white rounded-xl border border-gray-200 shadow-sm">
      <div className="flex items-center justify-between p-4 border-b border-gray-100">
        <div className="flex items-center gap-3">
          <input
            value={search}
            onChange={e => setSearch(e.target.value)}
            placeholder="Buscar por nombre o email..."
            className="border border-gray-300 rounded-lg px-3 py-1.5 text-sm w-64"
          />
          <span className="text-sm text-gray-500">{total} clientes</span>
        </div>
        <a href="/clients/new" className="bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium px-4 py-1.5 rounded-lg">
          + Nuevo Cliente
        </a>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-gray-100 text-left text-xs text-gray-500 uppercase">
              <th className="px-4 py-3">Nombre</th>
              <th className="px-4 py-3">Email</th>
              <th className="px-4 py-3">Telefono</th>
              <th className="px-4 py-3">Ciudad</th>
              <th className="px-4 py-3"></th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-50">
            {loading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <tr key={i}>{Array.from({ length: 5 }).map((_, j) => (
                  <td key={j} className="px-4 py-3"><div className="h-4 bg-gray-100 rounded animate-pulse" /></td>
                ))}</tr>
              ))
            ) : clients.length === 0 ? (
              <tr><td colSpan={5} className="px-4 py-12 text-center text-gray-400">No hay clientes</td></tr>
            ) : (
              clients.map(c => (
                <tr key={c.id} className="hover:bg-gray-50">
                  <td className="px-4 py-3 font-medium text-gray-800">{c.full_name}</td>
                  <td className="px-4 py-3 text-gray-600">{c.email ?? '—'}</td>
                  <td className="px-4 py-3 text-gray-600">{c.phone ?? c.mobile ?? '—'}</td>
                  <td className="px-4 py-3 text-gray-500">{c.city ?? '—'}</td>
                  <td className="px-4 py-3">
                    <a href={`/clients/${c.id}`} className="text-blue-600 hover:underline text-xs">Ver</a>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {total > 20 && (
        <div className="flex items-center justify-between p-4 border-t border-gray-100">
          <button onClick={() => load(page - 1)} disabled={page === 1} className="text-sm text-gray-600 disabled:opacity-40 hover:text-blue-600">← Anterior</button>
          <span className="text-sm text-gray-500">Pagina {page}</span>
          <button onClick={() => load(page + 1)} disabled={page * 20 >= total} className="text-sm text-gray-600 disabled:opacity-40 hover:text-blue-600">Siguiente →</button>
        </div>
      )}
    </div>
  );
}
