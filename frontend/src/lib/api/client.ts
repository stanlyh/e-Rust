import { $accessToken, initAuth, scheduleRefresh } from '../auth/store';

const BACKEND_URL = import.meta.env.PUBLIC_BACKEND_URL ?? 'http://127.0.0.1:8080';

interface FetchOptions extends RequestInit {
  skipAuth?: boolean;
}

export class APIError extends Error {
  constructor(
    public status: number,
    message: string,
    public data?: unknown
  ) {
    super(message);
    this.name = 'APIError';
  }
}

async function apiFetch<T>(endpoint: string, options: FetchOptions = {}): Promise<T> {
  const { skipAuth = false, ...fetchOptions } = options;

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(fetchOptions.headers as Record<string, string>),
  };

  if (!skipAuth) {
    let token = $accessToken.get();

    if (!token) {
      const refreshed = await initAuth();
      if (!refreshed) {
        window.location.href = '/login';
        throw new APIError(401, 'No autenticado');
      }
      token = $accessToken.get();
      scheduleRefresh();
    }

    headers['Authorization'] = `Bearer ${token}`;
  }

  const res = await fetch(`${BACKEND_URL}${endpoint}`, { ...fetchOptions, headers });

  if (res.status === 401 && !skipAuth) {
    const refreshed = await initAuth();
    if (!refreshed) {
      window.location.href = '/login';
      throw new APIError(401, 'Sesion expirada');
    }
    return apiFetch<T>(endpoint, options);
  }

  if (!res.ok) {
    const error = await res.json().catch(() => ({ error: res.statusText }));
    throw new APIError(res.status, error.error ?? 'Error en la solicitud', error);
  }

  if (res.status === 204) return undefined as unknown as T;
  return res.json() as Promise<T>;
}

export const api = {
  get:    <T>(ep: string, opts?: FetchOptions) => apiFetch<T>(ep, { ...opts, method: 'GET' }),
  post:   <T>(ep: string, body: unknown, opts?: FetchOptions) =>
    apiFetch<T>(ep, { ...opts, method: 'POST', body: JSON.stringify(body) }),
  put:    <T>(ep: string, body: unknown, opts?: FetchOptions) =>
    apiFetch<T>(ep, { ...opts, method: 'PUT', body: JSON.stringify(body) }),
  patch:  <T>(ep: string, body: unknown, opts?: FetchOptions) =>
    apiFetch<T>(ep, { ...opts, method: 'PATCH', body: JSON.stringify(body) }),
  delete: <T>(ep: string, opts?: FetchOptions) => apiFetch<T>(ep, { ...opts, method: 'DELETE' }),
};
