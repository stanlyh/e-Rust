import { api } from './client';

export type LeadSource = 'web' | 'referral' | 'walk_in' | 'phone' | 'social_media' | 'other';
export type LeadStatus = 'new' | 'contacted' | 'qualified' | 'unqualified' | 'converted';

export interface Lead {
  id: string;
  client_id?: string;
  assigned_to?: string;
  source: LeadSource;
  status: LeadStatus;
  interest_make?: string;
  interest_model?: string;
  interest_year?: number;
  budget_min?: string;
  budget_max?: string;
  notes?: string;
  contacted_at?: string;
  qualified_at?: string;
  created_at: string;
}

export interface LeadCreate {
  client_id?: string;
  source: LeadSource;
  interest_make?: string;
  interest_model?: string;
  interest_year?: number;
  budget_min?: number;
  budget_max?: number;
  notes?: string;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

export const SOURCE_LABELS: Record<LeadSource, string> = {
  web: 'Sitio web',
  referral: 'Referido',
  walk_in: 'Visita directa',
  phone: 'Telefono',
  social_media: 'Redes sociales',
  other: 'Otro',
};

export const STATUS_LABELS: Record<LeadStatus, string> = {
  new: 'Nuevo',
  contacted: 'Contactado',
  qualified: 'Calificado',
  unqualified: 'No calificado',
  converted: 'Convertido',
};

export const STATUS_COLORS: Record<LeadStatus, string> = {
  new: 'bg-blue-100 dark:bg-blue-950/60 text-blue-700 dark:text-blue-300',
  contacted: 'bg-yellow-100 dark:bg-yellow-950/60 text-yellow-700 dark:text-yellow-300',
  qualified: 'bg-green-100 dark:bg-green-950/60 text-green-700 dark:text-green-300',
  unqualified: 'bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400',
  converted: 'bg-purple-100 dark:bg-purple-950/60 text-purple-700 dark:text-purple-300',
};

export const leadsApi = {
  list: (params?: Record<string, string>) => {
    const qs = params ? '?' + new URLSearchParams(params).toString() : '';
    return api.get<PaginatedResponse<Lead>>(`/api/leads${qs}`);
  },
  getById: (id: string) => api.get<Lead>(`/api/leads/${id}`),
  create: (data: LeadCreate) => api.post<Lead>('/api/leads', data),
  update: (id: string, data: Partial<LeadCreate> & { status?: LeadStatus; assigned_to?: string }) =>
    api.put<Lead>(`/api/leads/${id}`, data),
  delete: (id: string) => api.delete<void>(`/api/leads/${id}`),
};
