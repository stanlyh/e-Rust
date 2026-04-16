import { useStore } from '@nanostores/react';
import { $toasts, removeToast, type Toast } from '../../lib/toast';

const ICONS: Record<string, string> = {
  success: '✓',
  error:   '✕',
  warning: '⚠',
  info:    'ℹ',
};

const STYLES: Record<string, string> = {
  success: 'bg-green-50 border-green-300 text-green-800',
  error:   'bg-red-50 border-red-300 text-red-800',
  warning: 'bg-orange-50 border-orange-300 text-orange-800',
  info:    'bg-blue-50 border-blue-300 text-blue-800',
};

const ICON_STYLES: Record<string, string> = {
  success: 'bg-green-500 text-white',
  error:   'bg-red-500 text-white',
  warning: 'bg-orange-500 text-white',
  info:    'bg-blue-500 text-white',
};

function ToastItem({ toast }: { toast: Toast }) {
  return (
    <div
      className={`flex items-start gap-3 px-4 py-3 rounded-lg border shadow-md
        text-sm max-w-sm w-full animate-in fade-in slide-in-from-right-5
        ${STYLES[toast.type]}`}
      role="alert"
    >
      <span className={`w-5 h-5 rounded-full flex items-center justify-center
        text-xs font-bold shrink-0 mt-0.5 ${ICON_STYLES[toast.type]}`}>
        {ICONS[toast.type]}
      </span>
      <p className="flex-1 leading-snug">{toast.message}</p>
      <button
        onClick={() => removeToast(toast.id)}
        className="shrink-0 opacity-50 hover:opacity-100 text-base leading-none mt-0.5"
        aria-label="Cerrar"
      >
        ×
      </button>
    </div>
  );
}

export default function ToastContainer() {
  const toasts = useStore($toasts);

  if (toasts.length === 0) return null;

  return (
    <div className="fixed bottom-5 right-5 z-50 flex flex-col gap-2 items-end pointer-events-none">
      {toasts.map(t => (
        <div key={t.id} className="pointer-events-auto">
          <ToastItem toast={t} />
        </div>
      ))}
    </div>
  );
}
