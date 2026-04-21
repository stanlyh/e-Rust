import type { APIRoute } from 'astro';

export const POST: APIRoute = async ({ cookies }) => {
  const refreshToken = cookies.get('refresh_token')?.value;

  if (refreshToken) {
    const backendUrl = import.meta.env.BACKEND_URL ?? 'http://127.0.0.1:8080';
    await fetch(`${backendUrl}/api/auth/logout`, {
      method: 'POST',
      headers: { Cookie: `refresh_token=${refreshToken}` },
    }).catch(() => {});
  }

  cookies.delete('refresh_token', { path: '/' });

  return new Response(JSON.stringify({ message: 'Sesion cerrada' }), { status: 200 });
};
