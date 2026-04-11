import { useEffect, useState } from 'react';
import { vehiclesApi, CONDITION_LABELS, FUEL_LABELS, type Vehicle } from '../../lib/api/vehicles';

export default function VehicleGrid() {
  const [vehicles, setVehicles] = useState<Vehicle[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [availableOnly, setAvailableOnly] = useState(false);
  const [loading, setLoading] = useState(true);

  const load = async (p = 1) => {
    setLoading(true);
    try {
      const params: Record<string, string> = { page: String(p), per_page: '12' };
      if (availableOnly) params.available_only = 'true';
      const res = await vehiclesApi.list(params);
      setVehicles(res.data);
      setTotal(res.total);
      setPage(p);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { load(1); }, [availableOnly]);

  return (
    <div>
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <label className="flex items-center gap-2 text-sm text-gray-600 cursor-pointer">
            <input type="checkbox" checked={availableOnly} onChange={e => setAvailableOnly(e.target.checked)} className="rounded" />
            Solo disponibles
          </label>
          <span className="text-sm text-gray-500">{total} vehiculos</span>
        </div>
        <a href="/vehicles/new" className="bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium px-4 py-1.5 rounded-lg">
          + Agregar Vehiculo
        </a>
      </div>

      {loading ? (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {Array.from({ length: 6 }).map((_, i) => (
            <div key={i} className="bg-white rounded-xl border border-gray-200 p-4 animate-pulse">
              <div className="h-32 bg-gray-100 rounded-lg mb-3" />
              <div className="h-4 bg-gray-100 rounded mb-2" />
              <div className="h-3 bg-gray-100 rounded w-2/3" />
            </div>
          ))}
        </div>
      ) : vehicles.length === 0 ? (
        <div className="bg-white rounded-xl border border-gray-200 p-12 text-center text-gray-400">
          No hay vehiculos en inventario
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {vehicles.map(v => (
            <a key={v.id} href={`/vehicles/${v.id}`} className="bg-white rounded-xl border border-gray-200 shadow-sm hover:shadow-md transition-shadow p-4 block">
              <div className="flex items-start justify-between mb-2">
                <div>
                  <p className="font-semibold text-gray-900">{v.year} {v.make} {v.model}</p>
                  {v.trim && <p className="text-xs text-gray-500">{v.trim}</p>}
                </div>
                <span className={`text-xs px-2 py-0.5 rounded-full font-medium ${
                  v.is_available ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-500'
                }`}>
                  {v.is_available ? 'Disponible' : 'No disponible'}
                </span>
              </div>

              <div className="flex items-center gap-3 text-xs text-gray-500 mb-3">
                <span>{FUEL_LABELS[v.fuel_type]}</span>
                <span>•</span>
                <span>{CONDITION_LABELS[v.condition]}</span>
                {v.mileage > 0 && <><span>•</span><span>{v.mileage.toLocaleString()} km</span></>}
              </div>

              <p className="text-lg font-bold text-blue-600">
                ${Number(v.list_price).toLocaleString('es-MX')}
              </p>
            </a>
          ))}
        </div>
      )}

      {total > 12 && (
        <div className="flex items-center justify-between mt-4">
          <button onClick={() => load(page - 1)} disabled={page === 1} className="text-sm text-gray-600 disabled:opacity-40 hover:text-blue-600">← Anterior</button>
          <span className="text-sm text-gray-500">Pagina {page}</span>
          <button onClick={() => load(page + 1)} disabled={page * 12 >= total} className="text-sm text-gray-600 disabled:opacity-40 hover:text-blue-600">Siguiente →</button>
        </div>
      )}
    </div>
  );
}
