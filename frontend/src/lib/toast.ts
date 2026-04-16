import { atom } from 'nanostores';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
  id: string;
  type: ToastType;
  message: string;
  duration?: number; // ms, default 4000
}

export const $toasts = atom<Toast[]>([]);

function addToast(type: ToastType, message: string, duration = 4000) {
  const id = crypto.randomUUID();
  $toasts.set([...$toasts.get(), { id, type, message, duration }]);
  setTimeout(() => removeToast(id), duration);
  return id;
}

export function removeToast(id: string) {
  $toasts.set($toasts.get().filter(t => t.id !== id));
}

export const toast = {
  success: (message: string, duration?: number) => addToast('success', message, duration),
  error:   (message: string, duration?: number) => addToast('error',   message, duration),
  warning: (message: string, duration?: number) => addToast('warning', message, duration),
  info:    (message: string, duration?: number) => addToast('info',    message, duration),
};
