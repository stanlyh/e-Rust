import { defineMiddleware } from 'astro:middleware';

const PUBLIC_PATHS = ['/login', '/api/auth/login', '/api/auth/refresh'];

export const onRequest = defineMiddleware(async ({ url, cookies, redirect }, next) => {
  const isPublic = PUBLIC_PATHS.some(path => url.pathname.startsWith(path));
  if (isPublic) return next();

  const hasRefreshToken = cookies.has('refresh_token');
  if (!hasRefreshToken) {
    return redirect('/login');
  }

  return next();
});
