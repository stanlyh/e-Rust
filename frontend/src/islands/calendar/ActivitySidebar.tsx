import { format, parseISO } from 'date-fns';
import { es } from 'date-fns/locale';
import type { Activity } from '../../lib/api/activities';

export interface ExpiringOpportunity {
  id: string;
  title: string;
  expected_close: string;
  days_remaining: number;
  probability: number;
  offered_price?: string;
}

const TYPE_LABELS: Record<string, string> = {
  call:       '📞 Llamada',
  email:      '📧 Email',
  visit:      '🏢 Visita',
  whatsapp:   '💬 WhatsApp',
  meeting:    '🤝 Reunion',
  test_drive: '🚗 Test Drive',
  delivery:   '🎉 Entrega',
};

interface Props {
  upcomingActivities: Activity[];
  overdueCount: number;
  expiringOpportunities: ExpiringOpportunity[];
  onActivityClick: (activity: Activity) => void;
  onNewActivity: () => void;
}

export default function ActivitySidebar({
  upcomingActivities,
  overdueCount,
  expiringOpportunities,
  onActivityClick,
  onNewActivity,
}: Props) {
  return (
    <div className="w-full lg:w-72 flex flex-col gap-3 shrink-0">
      <button
        onClick={onNewActivity}
        className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2.5 px-4 rounded-lg transition-colors text-sm"
      >
        + Nueva Actividad
      </button>

      {/* Alerta de actividades vencidas */}
      {overdueCount > 0 && (
        <div className="bg-red-50 border border-red-200 rounded-lg p-3">
          <p className="text-red-700 text-sm font-medium">
            ⚠️ {overdueCount} actividad{overdueCount !== 1 ? 'es' : ''} vencida{overdueCount !== 1 ? 's' : ''}
          </p>
          <p className="text-red-500 text-xs mt-0.5">Sin completar y fuera de fecha</p>
        </div>
      )}

      {/* Oportunidades proximas a vencer */}
      {expiringOpportunities.length > 0 && (
        <div className="bg-orange-50 border border-orange-200 rounded-lg p-3">
          <p className="text-orange-700 text-sm font-semibold mb-2">
            🔔 Oportunidades por vencer
          </p>
          <ul className="space-y-2">
            {expiringOpportunities.map(opp => (
              <li key={opp.id} className="text-xs">
                <p className="font-medium text-orange-800 truncate">{opp.title}</p>
                <div className="flex items-center justify-between text-orange-600 mt-0.5">
                  <span>
                    {opp.days_remaining === 0
                      ? 'Vence hoy'
                      : `${opp.days_remaining} dia${opp.days_remaining !== 1 ? 's' : ''}`}
                  </span>
                  {opp.offered_price && (
                    <span className="font-semibold">
                      ${Number(opp.offered_price).toLocaleString('es-MX', { maximumFractionDigits: 0 })}
                    </span>
                  )}
                </div>
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Proximas actividades */}
      <div className="bg-white rounded-xl border border-gray-200 shadow-sm lg:flex-1 overflow-hidden flex flex-col max-h-64 lg:max-h-none">
        <div className="p-4 border-b border-gray-100">
          <h2 className="text-sm font-semibold text-gray-700">Proximas actividades</h2>
        </div>

        <div className="overflow-auto flex-1">
          {upcomingActivities.length === 0 ? (
            <p className="text-sm text-gray-400 text-center py-8">Sin actividades proximas</p>
          ) : (
            <ul className="divide-y divide-gray-50">
              {upcomingActivities.map(activity => (
                <li key={activity.id}>
                  <button
                    onClick={() => onActivityClick(activity)}
                    className="w-full text-left px-4 py-3 hover:bg-gray-50 transition-colors"
                  >
                    <p className="text-xs text-gray-500 mb-0.5">
                      {TYPE_LABELS[activity.type] ?? activity.type}
                    </p>
                    <p className="text-sm font-medium text-gray-800 truncate">{activity.title}</p>
                    <p className="text-xs text-gray-400 mt-0.5">
                      {format(parseISO(activity.scheduled_start), "EEE d MMM, HH:mm", { locale: es })}
                    </p>
                  </button>
                </li>
              ))}
            </ul>
          )}
        </div>
      </div>
    </div>
  );
}
