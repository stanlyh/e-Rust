import type { APIRoute } from 'astro';

export const POST: APIRoute = async ({ request }) => {
  const body = await request.json();

  const backendUrl = import.meta.env.BACKEND_URL ?? 'http://127.0.0.1:8080';
  const res = await fetch(`${backendUrl}/api/auth/register`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });

  const data = await res.json().catch(() => ({ error: 'Error del servidor' }));
  return new Response(JSON.stringify(data), { status: res.status });
};
