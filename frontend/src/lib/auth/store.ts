import { atom, computed } from 'nanostores';

export interface AuthUser {
  id: string;
  email: string;
  full_name: string;
  role: 'admin' | 'manager' | 'sales_agent';
}

export const $accessToken = atom<string | null>(null);
export const $authUser = atom<AuthUser | null>(null);
export const $authLoading = atom<boolean>(true);

export const $isAuthenticated = computed(
  [$accessToken, $authUser],
  (token, user) => token !== null && user !== null
);

export function setAuth(token: string, user: AuthUser) {
  $accessToken.set(token);
  $authUser.set(user);
  $authLoading.set(false);
}

export function clearAuth() {
  $accessToken.set(null);
  $authUser.set(null);
}

let refreshTimer: ReturnType<typeof setTimeout> | null = null;
let refreshPromise: Promise<boolean> | null = null;

export async function initAuth(): Promise<boolean> {
  if (refreshPromise) return refreshPromise;

  refreshPromise = (async () => {
    try {
      const res = await fetch('/api/auth/refresh', { method: 'POST' });
      if (!res.ok) {
        $authLoading.set(false);
        return false;
      }
      const { access_token, user } = await res.json();
      setAuth(access_token, user);
      scheduleRefresh();
      return true;
    } catch {
      $authLoading.set(false);
      return false;
    } finally {
      refreshPromise = null;
    }
  })();

  return refreshPromise;
}

export function scheduleRefresh() {
  if (refreshTimer) clearTimeout(refreshTimer);
  // Refresca 1 minuto antes de que expire el access token (14 min)
  refreshTimer = setTimeout(async () => {
    await initAuth();
  }, 14 * 60 * 1000);
}
