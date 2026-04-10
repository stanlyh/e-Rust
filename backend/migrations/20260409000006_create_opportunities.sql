CREATE TABLE opportunities (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lead_id         UUID REFERENCES leads(id) ON DELETE SET NULL,
    client_id       UUID NOT NULL REFERENCES clients(id) ON DELETE RESTRICT,
    vehicle_id      UUID REFERENCES vehicles(id) ON DELETE SET NULL,
    assigned_to     UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    status          opportunity_status NOT NULL DEFAULT 'prospecting',
    title           VARCHAR(255) NOT NULL,
    offered_price   NUMERIC(12,2),
    discount        NUMERIC(10,2) DEFAULT 0,
    final_price     NUMERIC(12,2),
    probability     SMALLINT DEFAULT 20
        CHECK (probability >= 0 AND probability <= 100),
    expected_close  DATE,
    closed_at       TIMESTAMPTZ,
    lost_reason     TEXT,
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_opportunities_client_id ON opportunities(client_id);
CREATE INDEX idx_opportunities_vehicle_id ON opportunities(vehicle_id);
CREATE INDEX idx_opportunities_assigned_to ON opportunities(assigned_to);
CREATE INDEX idx_opportunities_status ON opportunities(status);
CREATE INDEX idx_opportunities_expected_close ON opportunities(expected_close);

CREATE TRIGGER opportunities_updated_at
    BEFORE UPDATE ON opportunities
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
