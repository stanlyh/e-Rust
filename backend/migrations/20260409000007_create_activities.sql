-- Tabla central del calendario
CREATE TABLE activities (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title            VARCHAR(255) NOT NULL,
    description      TEXT,
    type             activity_type NOT NULL,
    status           activity_status NOT NULL DEFAULT 'scheduled',
    scheduled_start  TIMESTAMPTZ NOT NULL,
    scheduled_end    TIMESTAMPTZ NOT NULL,
    completed_at     TIMESTAMPTZ,
    outcome          TEXT,
    next_action      TEXT,
    assigned_to      UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    -- Relaciones opcionales (la actividad puede estar asociada a uno o varios contextos)
    client_id        UUID REFERENCES clients(id) ON DELETE SET NULL,
    lead_id          UUID REFERENCES leads(id) ON DELETE SET NULL,
    opportunity_id   UUID REFERENCES opportunities(id) ON DELETE SET NULL,
    vehicle_id       UUID REFERENCES vehicles(id) ON DELETE SET NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_date_range CHECK (scheduled_end >= scheduled_start)
);

CREATE INDEX idx_activities_assigned_to ON activities(assigned_to);
CREATE INDEX idx_activities_scheduled_start ON activities(scheduled_start);
CREATE INDEX idx_activities_status ON activities(status);
CREATE INDEX idx_activities_opportunity_id ON activities(opportunity_id);
CREATE INDEX idx_activities_client_id ON activities(client_id);
-- Indice para consultas del calendario por rango de fechas
CREATE INDEX idx_activities_calendar ON activities(assigned_to, scheduled_start, scheduled_end);

CREATE TRIGGER activities_updated_at
    BEFORE UPDATE ON activities
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
