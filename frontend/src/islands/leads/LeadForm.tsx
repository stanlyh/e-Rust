import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { leadsApi, SOURCE_LABELS } from '../../lib/api/leads';

const schema = z.object({
  source: z.enum(['web', 'referral', 'walk_in', 'phone', 'social_media', 'other']),
  interest_make: z.string().max(100).optional(),
  interest_model: z.string().max(100).optional(),
  interest_year: z.coerce.number().int().min(1990).max(2030).optional().or(z.literal('')),
  budget_min: z.coerce.number().min(0).optional().or(z.literal('')),
  budget_max: z.coerce.number().min(0).optional().or(z.literal('')),
  notes: z.string().max(2000).optional(),
});

type FormData = z.infer<typeof schema>;

export default function LeadForm() {
  const { register, handleSubmit, formState: { errors, isSubmitting }, setError } =
    useForm<FormData>({ resolver: zodResolver(schema), defaultValues: { source: 'other' } });

  const onSubmit = async (data: FormData) => {
    try {
      const lead = await leadsApi.create({
        source: data.source,
        interest_make: data.interest_make || undefined,
        interest_model: data.interest_model || undefined,
        interest_year: data.interest_year ? Number(data.interest_year) : undefined,
        budget_min: data.budget_min ? Number(data.budget_min) : undefined,
        budget_max: data.budget_max ? Number(data.budget_max) : undefined,
        notes: data.notes || undefined,
      });
      window.location.href = `/leads/${lead.id}`;
    } catch {
      setError('root', { message: 'Error al crear el lead. Intenta de nuevo.' });
    }
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="max-w-lg space-y-5">
      {errors.root && (
        <div className="bg-red-50 border border-red-200 text-red-700 text-sm px-4 py-3 rounded-lg">
          {errors.root.message}
        </div>
      )}

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">Fuente *</label>
        <select {...register('source')} className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm">
          {Object.entries(SOURCE_LABELS).map(([v, l]) => (
            <option key={v} value={v}>{l}</option>
          ))}
        </select>
      </div>

      <fieldset className="border border-gray-200 rounded-lg p-4 space-y-3">
        <legend className="text-xs font-medium text-gray-500 px-1">Vehiculo de interes</legend>
        <div className="grid grid-cols-3 gap-3">
          <div>
            <label className="block text-xs text-gray-600 mb-1">Marca</label>
            <input {...register('interest_make')} className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" placeholder="Toyota" />
          </div>
          <div>
            <label className="block text-xs text-gray-600 mb-1">Modelo</label>
            <input {...register('interest_model')} className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" placeholder="Hilux" />
          </div>
          <div>
            <label className="block text-xs text-gray-600 mb-1">Año</label>
            <input {...register('interest_year')} type="number" className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" placeholder="2025" />
          </div>
        </div>
      </fieldset>

      <fieldset className="border border-gray-200 rounded-lg p-4 space-y-3">
        <legend className="text-xs font-medium text-gray-500 px-1">Presupuesto</legend>
        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="block text-xs text-gray-600 mb-1">Minimo ($)</label>
            <input {...register('budget_min')} type="number" className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" placeholder="20000" />
          </div>
          <div>
            <label className="block text-xs text-gray-600 mb-1">Maximo ($)</label>
            <input {...register('budget_max')} type="number" className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" placeholder="35000" />
          </div>
        </div>
      </fieldset>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">Notas</label>
        <textarea {...register('notes')} rows={3} className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm resize-none" placeholder="Informacion adicional..." />
      </div>

      <div className="flex gap-3">
        <a href="/leads" className="flex-1 text-center border border-gray-300 text-gray-700 text-sm font-medium py-2 rounded-lg hover:bg-gray-50">
          Cancelar
        </a>
        <button type="submit" disabled={isSubmitting} className="flex-1 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium py-2 rounded-lg disabled:opacity-50">
          {isSubmitting ? 'Guardando...' : 'Crear Lead'}
        </button>
      </div>
    </form>
  );
}
