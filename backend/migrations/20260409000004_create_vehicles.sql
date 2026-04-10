CREATE TABLE vehicles (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vin             VARCHAR(17) UNIQUE,
    stock_number    VARCHAR(50) UNIQUE,
    make            VARCHAR(100) NOT NULL,
    model           VARCHAR(100) NOT NULL,
    year            SMALLINT NOT NULL,
    trim            VARCHAR(100),
    color_exterior  VARCHAR(50),
    color_interior  VARCHAR(50),
    fuel_type       fuel_type NOT NULL DEFAULT 'gasoline',
    transmission    transmission_type NOT NULL DEFAULT 'automatic',
    mileage         INTEGER NOT NULL DEFAULT 0,
    condition       vehicle_condition NOT NULL DEFAULT 'new',
    list_price      NUMERIC(12,2) NOT NULL,
    cost_price      NUMERIC(12,2),
    is_available    BOOLEAN NOT NULL DEFAULT TRUE,
    description     TEXT,
    images          JSONB NOT NULL DEFAULT '[]',
    features        JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_vehicles_make_model ON vehicles(make, model);
CREATE INDEX idx_vehicles_available ON vehicles(is_available);
CREATE INDEX idx_vehicles_condition ON vehicles(condition);
CREATE INDEX idx_vehicles_year ON vehicles(year);

CREATE TRIGGER vehicles_updated_at
    BEFORE UPDATE ON vehicles
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
