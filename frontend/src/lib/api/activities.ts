import { api } from './client';

export type ActivityType = 'call' | 'email' | 'visit' | 'whatsapp' | 'meeting' | 'test_drive' | 'delivery';
export type ActivityStatus = 'scheduled' | 'completed' | 'cancelled' | 'rescheduled';

export interface Activity {
  id: string;
  title: string;
  description?: string;
  type: ActivityType;
  status: ActivityStatus;
  scheduled_start: string;
  scheduled_end: string;
  completed_at?: string;
  outcome?: string;
  next_action?: string;
  assigned_to: string;
  client_id?: string;
  lead_id?: string;
  opportunity_id?: string;
  vehicle_id?: string;
  created_at: string;
}

export interface CalendarEvent {
  id: string;
  title: string;
  start: string;
  end: string;
  type: ActivityType;
  status: ActivityStatus;
  extendedProps: Activity;
}

export interface ExpiringOpportunity {
  id: string;
  title: string;
  expected_close: string;
  days_remaining: number;
  probability: number;
  offered_price?: string;
}

export interface CalendarResponse {
  events: CalendarEvent[];
  overdue_count: number;
  expiring_opportunities: ExpiringOpportunity[];
}

export interface ActivityCreate {
  title: string;
  description?: string;
  type: ActivityType;
  scheduled_start: string;
  scheduled_end: string;
  client_id?: string;
  lead_id?: string;
  opportunity_id?: string;
  vehicle_id?: string;
}

export interface ActivityComplete {
  outcome: string;
  next_action?: string;
}

export const activitiesApi = {
  calendar: (from: string, to: string) =>
    api.get<CalendarResponse>(`/api/calendar?from=${from}&to=${to}`),

  upcoming: () =>
    api.get<Activity[]>('/api/activities/upcoming'),

  overdue: () =>
    api.get<Activity[]>('/api/activities/overdue'),

  getById: (id: string) =>
    api.get<Activity>(`/api/activities/${id}`),

  create: (data: ActivityCreate) =>
    api.post<Activity>('/api/activities', data),

  update: (id: string, data: Partial<ActivityCreate>) =>
    api.put<Activity>(`/api/activities/${id}`, data),

  complete: (id: string, data: ActivityComplete) =>
    api.patch<Activity>(`/api/activities/${id}/complete`, data),

  reschedule: (id: string, start: string, end: string) =>
    api.patch<Activity>(`/api/activities/${id}/reschedule`, { scheduled_start: start, scheduled_end: end }),

  delete: (id: string) =>
    api.delete<void>(`/api/activities/${id}`),
};
