-- Roles de usuario
CREATE TYPE user_role AS ENUM ('admin', 'manager', 'sales_agent');

-- Leads
CREATE TYPE lead_source AS ENUM ('web', 'referral', 'walk_in', 'phone', 'social_media', 'other');
CREATE TYPE lead_status AS ENUM ('new', 'contacted', 'qualified', 'unqualified', 'converted');

-- Oportunidades
CREATE TYPE opportunity_status AS ENUM (
    'prospecting',
    'needs_analysis',
    'proposal',
    'negotiation',
    'closed_won',
    'closed_lost'
);

-- Actividades del calendario
CREATE TYPE activity_type AS ENUM (
    'call',
    'email',
    'visit',
    'whatsapp',
    'meeting',
    'test_drive',
    'delivery'
);
CREATE TYPE activity_status AS ENUM ('scheduled', 'completed', 'cancelled', 'rescheduled');

-- Vehiculos
CREATE TYPE fuel_type AS ENUM ('gasoline', 'diesel', 'hybrid', 'electric', 'other');
CREATE TYPE transmission_type AS ENUM ('manual', 'automatic', 'cvt');
CREATE TYPE vehicle_condition AS ENUM ('new', 'used', 'certified_used');
