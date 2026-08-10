import { api } from './client';
import type { PaginatedResponse } from './leads';

export type OpportunityStatus =
  | 'prospecting'
  | 'needs_analysis'
  | 'proposal'
  | 'negotiation'
  | 'closed_won'
  | 'closed_lost';

export interface Opportunity {
  id: string;
  lead_id?: string;
  client_id: string;
  vehicle_id?: string;
  assigned_to: string;
  status: OpportunityStatus;
  title: string;
  offered_price?: string;
  discount?: string;
  final_price?: string;
  probability: number;
  expected_close?: string;
  closed_at?: string;
  lost_reason?: string;
  notes?: string;
  created_at: string;
}

export interface PipelineColumn {
  status: OpportunityStatus;
  opportunities: Opportunity[];
  total_value: number;
  count: number;
}

export interface PipelineResponse {
  columns: PipelineColumn[];
}

export const STATUS_LABELS: Record<OpportunityStatus, string> = {
  prospecting:    'Prospeccion',
  needs_analysis: 'Analisis',
  proposal:       'Propuesta',
  negotiation:    'Negociacion',
  closed_won:     'Ganado',
  closed_lost:    'Perdido',
};

export const STATUS_COLORS: Record<OpportunityStatus, string> = {
  prospecting:    'border-blue-300 dark:border-blue-800 bg-blue-50 dark:bg-blue-950/30',
  needs_analysis: 'border-yellow-300 dark:border-yellow-800 bg-yellow-50 dark:bg-yellow-950/30',
  proposal:       'border-orange-300 dark:border-orange-800 bg-orange-50 dark:bg-orange-950/30',
  negotiation:    'border-purple-300 dark:border-purple-800 bg-purple-50 dark:bg-purple-950/30',
  closed_won:     'border-green-300 dark:border-green-800 bg-green-50 dark:bg-green-950/30',
  closed_lost:    'border-gray-300 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/60',
};

export const STAGE_ORDER: OpportunityStatus[] = [
  'prospecting', 'needs_analysis', 'proposal',
  'negotiation', 'closed_won', 'closed_lost',
];

export const opportunitiesApi = {
  pipeline: () => api.get<PipelineResponse>('/api/opportunities/pipeline'),
  list: (params?: Record<string, string>) => {
    const qs = params ? '?' + new URLSearchParams(params).toString() : '';
    return api.get<PaginatedResponse<Opportunity>>(`/api/opportunities${qs}`);
  },
  getById: (id: string) => api.get<Opportunity>(`/api/opportunities/${id}`),
  updateStatus: (id: string, status: OpportunityStatus) =>
    api.patch<Opportunity>(`/api/opportunities/${id}/status`, { status }),
};
