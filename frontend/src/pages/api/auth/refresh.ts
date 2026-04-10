import type { APIRoute } from 'astro';

export const POST: APIRoute = async ({ cookies }) => {
  const refreshToken = cookies.get('refresh_token')?.value;

  if (!refreshToken) {
    return new Response(JSON.stringify({ error: 'No hay sesion activa' }), { status: 401 });
  }

  const res = await fetch(`${import.meta.env.BACKEND_URL}/api/auth/refresh`, {
    method: 'POST',
    headers: { Cookie: `refresh_token=${refreshToken}` },
  });

  if (!res.ok) {
    cookies.delete('refresh_token', { path: '/' });
    return new Response(JSON.stringify({ error: 'Sesion expirada' }), { status: 401 });
  }

  const data = await res.json();

  // Rotar el refresh token
  const setCookie = res.headers.get('set-cookie');
  if (setCookie) {
    const match = setCookie.match(/refresh_token=([^;]+)/);
    if (match) {
      cookies.set('refresh_token', match[1], {
        httpOnly: true,
        secure: import.meta.env.PROD,
        sameSite: 'strict',
        path: '/',
        maxAge: 7 * 24 * 60 * 60,
      });
    }
  }

  return new Response(JSON.stringify({
    access_token: data.access_token,
    expires_in: data.expires_in,
    user: data.user,
  }), { status: 200 });
};
