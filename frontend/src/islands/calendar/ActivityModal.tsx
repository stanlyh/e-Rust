import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { format, parseISO } from 'date-fns';
import { activitiesApi, type Activity, type ActivityType } from '../../lib/api/activities';

const activitySchema = z.object({
  title: z.string().min(2, 'Minimo 2 caracteres').max(255),
  type: z.enum(['call', 'email', 'visit', 'whatsapp', 'meeting', 'test_drive', 'delivery']),
  scheduled_start: z.string().min(1, 'La fecha de inicio es requerida'),
  scheduled_end: z.string().min(1, 'La fecha de fin es requerida'),
  description: z.string().max(1000).optional(),
});

type FormData = z.infer<typeof activitySchema>;

const TYPE_OPTIONS: { value: ActivityType; label: string }[] = [
  { value: 'call',       label: '📞 Llamada' },
  { value: 'email',      label: '📧 Email' },
  { value: 'visit',      label: '🏢 Visita' },
  { value: 'whatsapp',   label: '💬 WhatsApp' },
  { value: 'meeting',    label: '🤝 Reunion' },
  { value: 'test_drive', label: '🚗 Test Drive' },
  { value: 'delivery',   label: '🎉 Entrega' },
];

const STATUS_LABELS: Record<string, string> = {
  scheduled:   'Programada',
  completed:   'Completada',
  cancelled:   'Cancelada',
  rescheduled: 'Reprogramada',
};

interface Props {
  activity: Activity | null;
  defaultDate: string | null;
  onClose: () => void;
  onSaved: () => void;
}

export default function ActivityModal({ activity, defaultDate, onClose, onSaved }: Props) {
  const [completeMode, setCompleteMode] = useState(false);
  const [outcome, setOutcome] = useState('');
  const [nextAction, setNextAction] = useState('');
  const [saving, setSaving] = useState(false);

  const defaultStart = defaultDate
    ? format(new Date(defaultDate), "yyyy-MM-dd'T'HH:mm")
    : activity
    ? format(parseISO(activity.scheduled_start), "yyyy-MM-dd'T'HH:mm")
    : format(new Date(), "yyyy-MM-dd'T'HH:mm");

  const defaultEnd = activity
    ? format(parseISO(activity.scheduled_end), "yyyy-MM-dd'T'HH:mm")
    : defaultStart;

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<FormData>({
    resolver: zodResolver(activitySchema),
    defaultValues: {
      title: activity?.title ?? '',
      type: activity?.type ?? 'call',
      scheduled_start: defaultStart,
      scheduled_end: defaultEnd,
      description: activity?.description ?? '',
    },
  });

  const onSubmit = async (data: FormData) => {
    try {
      if (activity) {
        await activitiesApi.update(activity.id, data);
      } else {
        await activitiesApi.create(data);
      }
      onSaved();
    } catch (err) {
      console.error(err);
    }
  };

  const handleComplete = async () => {
    if (!activity || !outcome.trim()) return;
    setSaving(true);
    try {
      await activitiesApi.complete(activity.id, {
        outcome,
        next_action: nextAction || undefined,
      });
      onSaved();
    } catch (err) {
      console.error(err);
    } finally {
      setSaving(false);
    }
  };

  const handleDelete = async () => {
    if (!activity || !confirm('¿Eliminar esta actividad?')) return;
    try {
      await activitiesApi.delete(activity.id);
      onSaved();
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/40 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-xl shadow-xl w-full max-w-md">
        {/* Header */}
        <div className="flex items-center justify-between p-5 border-b border-gray-100">
          <h2 className="text-base font-semibold text-gray-900">
            {activity ? 'Detalle de actividad' : 'Nueva actividad'}
          </h2>
          <div className="flex items-center gap-2">
            {activity && (
              <span className="text-xs px-2 py-1 bg-gray-100 rounded-full text-gray-600">
                {STATUS_LABELS[activity.status] ?? activity.status}
              </span>
            )}
            <button onClick={onClose} className="text-gray-400 hover:text-gray-600 text-xl leading-none">
              ×
            </button>
          </div>
        </div>

        {/* Body */}
        <div className="p-5">
          {!completeMode ? (
            <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
              <div>
                <label className="block text-xs font-medium text-gray-600 mb-1">Titulo *</label>
                <input
                  {...register('title')}
                  className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="Llamar a cliente sobre propuesta"
                />
                {errors.title && <p className="text-red-500 text-xs mt-1">{errors.title.message}</p>}
              </div>

              <div>
                <label className="block text-xs font-medium text-gray-600 mb-1">Tipo *</label>
                <select {...register('type')} className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm">
                  {TYPE_OPTIONS.map(opt => (
                    <option key={opt.value} value={opt.value}>{opt.label}</option>
                  ))}
                </select>
              </div>

              <div className="grid grid-cols-2 gap-3">
                <div>
                  <label className="block text-xs font-medium text-gray-600 mb-1">Inicio *</label>
                  <input
                    {...register('scheduled_start')}
                    type="datetime-local"
                    className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                  {errors.scheduled_start && <p className="text-red-500 text-xs mt-1">{errors.scheduled_start.message}</p>}
                </div>
                <div>
                  <label className="block text-xs font-medium text-gray-600 mb-1">Fin *</label>
                  <input
                    {...register('scheduled_end')}
                    type="datetime-local"
                    className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                  {errors.scheduled_end && <p className="text-red-500 text-xs mt-1">{errors.scheduled_end.message}</p>}
                </div>
              </div>

              <div>
                <label className="block text-xs font-medium text-gray-600 mb-1">Descripcion</label>
                <textarea
                  {...register('description')}
                  rows={2}
                  className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                  placeholder="Detalles adicionales..."
                />
              </div>

              {/* Footer del form */}
              <div className="flex items-center justify-between pt-2">
                <div className="flex gap-2">
                  {activity && activity.status === 'scheduled' && (
                    <button
                      type="button"
                      onClick={() => setCompleteMode(true)}
                      className="text-sm text-green-600 hover:text-green-700 font-medium"
                    >
                      ✓ Completar
                    </button>
                  )}
                  {activity && (
                    <button
                      type="button"
                      onClick={handleDelete}
                      className="text-sm text-red-500 hover:text-red-600"
                    >
                      Eliminar
                    </button>
                  )}
                </div>
                <div className="flex gap-2">
                  <button type="button" onClick={onClose} className="text-sm text-gray-500 hover:text-gray-700 px-3 py-1.5">
                    Cancelar
                  </button>
                  <button
                    type="submit"
                    disabled={isSubmitting}
                    className="bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium px-4 py-1.5 rounded-lg disabled:opacity-50"
                  >
                    {isSubmitting ? 'Guardando...' : activity ? 'Actualizar' : 'Crear'}
                  </button>
                </div>
              </div>
            </form>
          ) : (
            /* Modo completar actividad */
            <div className="space-y-4">
              <p className="text-sm text-gray-600">Registra el resultado de esta actividad.</p>
              <div>
                <label className="block text-xs font-medium text-gray-600 mb-1">Resultado *</label>
                <textarea
                  value={outcome}
                  onChange={e => setOutcome(e.target.value)}
                  rows={3}
                  className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                  placeholder="Que paso en esta actividad?"
                />
              </div>
              <div>
                <label className="block text-xs font-medium text-gray-600 mb-1">Proxima accion</label>
                <input
                  value={nextAction}
                  onChange={e => setNextAction(e.target.value)}
                  className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="Enviar propuesta el lunes"
                />
              </div>
              <div className="flex justify-between pt-2">
                <button type="button" onClick={() => setCompleteMode(false)} className="text-sm text-gray-500">
                  Volver
                </button>
                <button
                  onClick={handleComplete}
                  disabled={!outcome.trim() || saving}
                  className="bg-green-600 hover:bg-green-700 text-white text-sm font-medium px-4 py-1.5 rounded-lg disabled:opacity-50"
                >
                  {saving ? 'Guardando...' : 'Marcar completada'}
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
