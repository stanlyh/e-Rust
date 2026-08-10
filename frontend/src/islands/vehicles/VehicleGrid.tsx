import { useEffect, useState } from 'react';
import { vehiclesApi, CONDITION_LABELS, FUEL_LABELS, type Vehicle, type VehicleCondition } from '../../lib/api/vehicles';
import { CardSkeleton } from '../ui/Skeleton';
import { ErrorState } from '../ui/ErrorState';
import { EmptyState } from '../ui/EmptyState';
import { Pagination } from '../ui/Pagination';
import { toast } from '../../lib/toast';

const PER_PAGE = 12;

interface Filters {
  make: string;
  condition: string;
  available_only: boolean;
  min_price: string;
  max_price: string;
}

const DEFAULT_FILTERS: Filters = {
  make: '',
  condition: '',
  available_only: false,
  min_price: '',
  max_price: '',
};

export default function VehicleGrid() {
  const [vehicles, setVehicles] = useState<Vehicle[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [filters, setFilters] = useState<Filters>(DEFAULT_FILTERS);
  const [showFilters, setShowFilters] = useState(false);

  const hasActiveFilters = Object.entries(filters).some(([k, v]) =>
    k === 'available_only' ? v === true : v !== ''
  );

  const load = async (p = 1) => {
    setLoading(true);
    setError('');
    try {
      const params: Record<string, string> = { page: String(p), per_page: String(PER_PAGE) };
      if (filters.make)          params.make = filters.make;
      if (filters.condition)     params.condition = filters.condition;
      if (filters.available_only) params.available_only = 'true';
      if (filters.min_price)     params.min_price = filters.min_price;
      if (filters.max_price)     params.max_price = filters.max_price;

      const res = await vehiclesApi.list(params);
      setVehicles(res.data);
      setTotal(res.total);
      setPage(p);
    } catch {
      setError('No se pudo cargar el inventario.');
      toast.error('Error cargando vehiculos');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { load(1); }, [filters]);

  const resetFilters = () => setFilters(DEFAULT_FILTERS);

  return (
    <div>
      {/* Toolbar */}
      <div className="flex flex-wrap items-center justify-between gap-3 mb-4">
        <div className="flex items-center gap-3">
          <button
            onClick={() => setShowFilters(!showFilters)}
            className={`flex items-center gap-1.5 border rounded-lg px-3 py-1.5 text-sm transition-colors
              ${showFilters ? 'border-blue-400 dark:border-blue-600 bg-blue-50 dark:bg-blue-950/40 text-blue-700 dark:text-blue-300' : 'border-gray-300 dark:border-gray-600 text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700/50'}`}
          >
            ⚙ Filtros {hasActiveFilters && <span className="bg-blue-500 text-white text-xs rounded-full w-4 h-4 flex items-center justify-center">!</span>}
          </button>
          <span className="text-sm text-gray-400 dark:text-gray-500">{total} vehiculos</span>
          {hasActiveFilters && (
            <button onClick={resetFilters} className="text-xs text-gray-500 dark:text-gray-400 hover:text-blue-600 dark:hover:text-blue-400 underline">
              Limpiar
            </button>
          )}
        </div>
        <a href="/vehicles/new" className="bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium px-4 py-1.5 rounded-lg whitespace-nowrap">
          + Agregar Vehiculo
        </a>
      </div>

      {/* Panel de filtros */}
      {showFilters && (
        <div className="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 shadow-sm p-4 mb-4 grid grid-cols-2 md:grid-cols-4 gap-3">
          <div>
            <label className="block text-xs text-gray-500 dark:text-gray-400 mb-1">Marca</label>
            <input
              value={filters.make}
              onChange={e => setFilters(f => ({ ...f, make: e.target.value }))}
              placeholder="Toyota, Ford..."
              className="w-full border border-gray-300 dark:border-gray-600 dark:bg-gray-900 dark:text-gray-100 rounded-lg px-3 py-1.5 text-sm"
            />
          </div>

          <div>
            <label className="block text-xs text-gray-500 dark:text-gray-400 mb-1">Condicion</label>
            <select
              value={filters.condition}
              onChange={e => setFilters(f => ({ ...f, condition: e.target.value }))}
              className="w-full border border-gray-300 dark:border-gray-600 dark:bg-gray-900 dark:text-gray-100 rounded-lg px-3 py-1.5 text-sm"
            >
              <option value="">Todas</option>
              {Object.entries(CONDITION_LABELS).map(([v, l]) => (
                <option key={v} value={v}>{l}</option>
              ))}
            </select>
          </div>

          <div>
            <label className="block text-xs text-gray-500 dark:text-gray-400 mb-1">Precio minimo ($)</label>
            <input
              type="number"
              value={filters.min_price}
              onChange={e => setFilters(f => ({ ...f, min_price: e.target.value }))}
              placeholder="0"
              className="w-full border border-gray-300 dark:border-gray-600 dark:bg-gray-900 dark:text-gray-100 rounded-lg px-3 py-1.5 text-sm"
            />
          </div>

          <div>
            <label className="block text-xs text-gray-500 dark:text-gray-400 mb-1">Precio maximo ($)</label>
            <input
              type="number"
              value={filters.max_price}
              onChange={e => setFilters(f => ({ ...f, max_price: e.target.value }))}
              placeholder="999999"
              className="w-full border border-gray-300 dark:border-gray-600 dark:bg-gray-900 dark:text-gray-100 rounded-lg px-3 py-1.5 text-sm"
            />
          </div>

          <div className="col-span-2 md:col-span-4">
            <label className="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400 cursor-pointer">
              <input
                type="checkbox"
                checked={filters.available_only}
                onChange={e => setFilters(f => ({ ...f, available_only: e.target.checked }))}
                className="rounded"
              />
              Solo vehiculos disponibles
            </label>
          </div>
        </div>
      )}

      {/* Contenido */}
      {error ? (
        <ErrorState message={error} onRetry={() => load(page)} />
      ) : loading ? (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          <CardSkeleton count={PER_PAGE} />
        </div>
      ) : vehicles.length === 0 ? (
        <EmptyState
          icon="🚗"
          title={hasActiveFilters ? 'Sin resultados' : 'Sin vehiculos en inventario'}
          description={hasActiveFilters
            ? 'Prueba ajustando los filtros de busqueda.'
            : 'Agrega el primer vehiculo al inventario.'}
          action={!hasActiveFilters ? { label: '+ Agregar Vehiculo', href: '/vehicles/new' } : undefined}
        />
      ) : (
        <>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            {vehicles.map(v => (
              <a
                key={v.id}
                href={`/vehicles/${v.id}`}
                className="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 shadow-sm hover:shadow-md transition-shadow p-4 block"
              >
                <div className="flex items-start justify-between mb-2">
                  <div>
                    <p className="font-semibold text-gray-900 dark:text-gray-100">{v.year} {v.make} {v.model}</p>
                    {v.trim && <p className="text-xs text-gray-500 dark:text-gray-400">{v.trim}</p>}
                  </div>
                  <span className={`text-xs px-2 py-0.5 rounded-full font-medium shrink-0 ${
                    v.is_available ? 'bg-green-100 dark:bg-green-950/60 text-green-700 dark:text-green-300' : 'bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400'
                  }`}>
                    {v.is_available ? 'Disponible' : 'No disponible'}
                  </span>
                </div>

                <div className="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400 mb-3 flex-wrap">
                  <span>{FUEL_LABELS[v.fuel_type]}</span>
                  <span>·</span>
                  <span>{CONDITION_LABELS[v.condition]}</span>
                  {v.mileage > 0 && (
                    <><span>·</span><span>{v.mileage.toLocaleString()} km</span></>
                  )}
                </div>

                <p className="text-lg font-bold text-blue-600 dark:text-blue-400">
                  ${Number(v.list_price).toLocaleString('es-MX')}
                </p>
              </a>
            ))}
          </div>

          <div className="mt-4 bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700">
            <Pagination page={page} total={total} perPage={PER_PAGE} onPage={load} />
          </div>
        </>
      )}
    </div>
  );
}
