import { api } from './client';
import type { PaginatedResponse } from './leads';

export interface Client {
  id: string;
  first_name: string;
  last_name: string;
  full_name: string;
  email?: string;
  phone?: string;
  mobile?: string;
  id_document?: string;
  address?: string;
  city?: string;
  notes?: string;
  assigned_to?: string;
  created_at: string;
}

export interface ClientCreate {
  first_name: string;
  last_name: string;
  email?: string;
  phone?: string;
  mobile?: string;
  id_document?: string;
  address?: string;
  city?: string;
  notes?: string;
}

export const clientsApi = {
  list: (params?: Record<string, string>) => {
    const qs = params ? '?' + new URLSearchParams(params).toString() : '';
    return api.get<PaginatedResponse<Client>>(`/api/clients${qs}`);
  },
  getById: (id: string) => api.get<Client>(`/api/clients/${id}`),
  create: (data: ClientCreate) => api.post<Client>('/api/clients', data),
  update: (id: string, data: Partial<ClientCreate>) => api.put<Client>(`/api/clients/${id}`, data),
  delete: (id: string) => api.delete<void>(`/api/clients/${id}`),
};
