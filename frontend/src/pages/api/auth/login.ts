import type { APIRoute } from 'astro';

export const POST: APIRoute = async ({ request, cookies }) => {
  const body = await request.json();

  const backendUrl = import.meta.env.BACKEND_URL ?? 'http://127.0.0.1:8080';
  const res = await fetch(`${backendUrl}/api/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: 'Credenciales invalidas' }));
    return new Response(JSON.stringify(err), { status: res.status });
  }

  const data = await res.json();

  // El refresh token viene en la cookie Set-Cookie del backend
  // Lo propagamos como cookie HttpOnly
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
