CREATE TABLE leads (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_id       UUID REFERENCES clients(id) ON DELETE CASCADE,
    assigned_to     UUID REFERENCES users(id) ON DELETE SET NULL,
    source          lead_source NOT NULL DEFAULT 'other',
    status          lead_status NOT NULL DEFAULT 'new',
    interest_make   VARCHAR(100),
    interest_model  VARCHAR(100),
    interest_year   SMALLINT,
    budget_min      NUMERIC(12,2),
    budget_max      NUMERIC(12,2),
    notes           TEXT,
    contacted_at    TIMESTAMPTZ,
    qualified_at    TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_leads_client_id ON leads(client_id);
CREATE INDEX idx_leads_assigned_to ON leads(assigned_to);
CREATE INDEX idx_leads_status ON leads(status);
CREATE INDEX idx_leads_created_at ON leads(created_at DESC);

CREATE TRIGGER leads_updated_at
    BEFORE UPDATE ON leads
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
