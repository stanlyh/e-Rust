import { api } from './client';
import type { PaginatedResponse } from './leads';

export type FuelType = 'gasoline' | 'diesel' | 'hybrid' | 'electric' | 'other';
export type TransmissionType = 'manual' | 'automatic' | 'cvt';
export type VehicleCondition = 'new' | 'used' | 'certified_used';

export interface Vehicle {
  id: string;
  vin?: string;
  stock_number?: string;
  make: string;
  model: string;
  year: number;
  trim?: string;
  color_exterior?: string;
  color_interior?: string;
  fuel_type: FuelType;
  transmission: TransmissionType;
  mileage: number;
  condition: VehicleCondition;
  list_price: string;
  is_available: boolean;
  description?: string;
  images: string[];
  features: Record<string, unknown>;
  created_at: string;
}

export interface VehicleCreate {
  make: string;
  model: string;
  year: number;
  list_price: number;
  vin?: string;
  stock_number?: string;
  trim?: string;
  color_exterior?: string;
  color_interior?: string;
  fuel_type?: FuelType;
  transmission?: TransmissionType;
  mileage?: number;
  condition?: VehicleCondition;
  description?: string;
}

export const CONDITION_LABELS: Record<VehicleCondition, string> = {
  new: 'Nuevo',
  used: 'Usado',
  certified_used: 'Certificado',
};

export const FUEL_LABELS: Record<FuelType, string> = {
  gasoline: 'Gasolina',
  diesel: 'Diesel',
  hybrid: 'Hibrido',
  electric: 'Electrico',
  other: 'Otro',
};

export const vehiclesApi = {
  list: (params?: Record<string, string>) => {
    const qs = params ? '?' + new URLSearchParams(params).toString() : '';
    return api.get<PaginatedResponse<Vehicle>>(`/api/vehicles${qs}`);
  },
  getById: (id: string) => api.get<Vehicle>(`/api/vehicles/${id}`),
  create: (data: VehicleCreate) => api.post<Vehicle>('/api/vehicles', data),
  update: (id: string, data: Partial<VehicleCreate>) => api.put<Vehicle>(`/api/vehicles/${id}`, data),
  setAvailability: (id: string, available: boolean) =>
    api.patch<Vehicle>(`/api/vehicles/${id}/availability`, { available }),
  delete: (id: string) => api.delete<void>(`/api/vehicles/${id}`),
};
