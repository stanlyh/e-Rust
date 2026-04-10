import type { APIRoute } from 'astro';

export const POST: APIRoute = async ({ cookies }) => {
  const refreshToken = cookies.get('refresh_token')?.value;

  if (refreshToken) {
    await fetch(`${import.meta.env.BACKEND_URL}/api/auth/logout`, {
      method: 'POST',
      headers: { Cookie: `refresh_token=${refreshToken}` },
    }).catch(() => {});
  }

  cookies.delete('refresh_token', { path: '/' });

  return new Response(JSON.stringify({ message: 'Sesion cerrada' }), { status: 200 });
};
