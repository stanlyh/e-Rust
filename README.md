# e-Rust CRM — Sistema de Gestión para Concesionarias

CRM especializado en venta de vehículos. Gestiona clientes, inventario, pipeline de ventas, calendario de actividades y reportes.

## Stack tecnológico

| Capa | Tecnología |
|------|-----------|
| Backend | Rust + Axum 0.8 + Tokio |
| Base de datos | PostgreSQL 16 + SQLx 0.8 |
| Frontend | Astro 5 (SSR) + React 19 (islands) |
| Estilos | Tailwind CSS 3.4 |
| Auth | JWT (access 15 min) + Refresh token (cookie, 7 días) + Argon2 |
| Validación | Zod (frontend) + validator crate (backend) |
| Drag & Drop | @dnd-kit |
| Calendario | FullCalendar 6 |
| Gráficos | Recharts 2 |

---

## Arquitectura general

```mermaid
graph TB
    subgraph Frontend["Frontend (Astro SSR · puerto 4321)"]
        MW[Middleware de auth]
        Pages[Páginas Astro]
        Islands[React Islands]
        MW --> Pages --> Islands
    end

    subgraph Backend["Backend (Axum · puerto 8080)"]
        Routes[Routes]
        Handlers[Handlers]
        Repos[Repositories]
        Models[Models]
        AuthMW[JWT Middleware]
        Routes --> AuthMW --> Handlers --> Repos --> Models
    end

    subgraph DB["Base de datos"]
        PG[(PostgreSQL 16)]
    end

    Browser([Navegador]) --> Frontend
    Islands -- "HTTP/REST (Bearer token)" --> Backend
    Pages -- "HTTP/REST (server-side)" --> Backend
    Backend --> PG
```

---

## Estructura del proyecto

```
e-Rust/
├── backend/
│   ├── migrations/          # Migraciones SQL (SQLx)
│   └── src/
│       ├── main.rs
│       ├── config.rs        # Variables de entorno
│       ├── state.rs         # Estado compartido (AppState)
│       ├── error.rs         # Manejo de errores centralizado
│       ├── db/pool.rs       # Pool de conexiones
│       ├── models/          # Structs Rust (serde)
│       ├── handlers/        # Lógica de cada endpoint
│       ├── routes/          # Configuración de rutas Axum
│       ├── repositories/    # Queries SQLx
│       ├── middleware/      # Extractor JWT
│       └── services/        # Lógica de negocio
├── frontend/
│   └── src/
│       ├── pages/           # Rutas Astro (file-based routing)
│       ├── components/      # Componentes Astro (layout, UI estática)
│       ├── islands/         # Componentes React interactivos
│       ├── lib/
│       │   ├── api/         # Clientes HTTP por módulo
│       │   └── auth/        # Store de sesión (nanostores)
│       ├── styles/
│       └── middleware.ts    # Protección de rutas
├── docker-compose.yml
├── .env.example
└── package.json             # Scripts raíz (pnpm)
```

---

## Modelo de datos

```mermaid
erDiagram
    users {
        uuid id PK
        varchar email UK
        varchar password_hash
        varchar full_name
        user_role role
        varchar phone
        boolean is_active
        timestamptz created_at
        timestamptz updated_at
    }

    refresh_tokens {
        uuid id PK
        uuid user_id FK
        varchar token_hash UK
        timestamptz expires_at
        timestamptz revoked_at
    }

    clients {
        uuid id PK
        varchar first_name
        varchar last_name
        varchar email
        varchar phone
        text id_document
        text address
        varchar city
        text notes
        uuid assigned_to FK
        timestamptz created_at
        timestamptz updated_at
    }

    vehicles {
        uuid id PK
        varchar vin UK
        varchar stock_number UK
        varchar make
        varchar model
        smallint year
        varchar trim
        fuel_type fuel_type
        transmission_type transmission
        vehicle_condition condition
        integer mileage
        numeric list_price
        numeric cost_price
        boolean is_available
        jsonb images
        jsonb features
        timestamptz created_at
        timestamptz updated_at
    }

    leads {
        uuid id PK
        uuid client_id FK
        uuid assigned_to FK
        lead_source source
        lead_status status
        varchar interest_make
        varchar interest_model
        smallint interest_year
        numeric budget_min
        numeric budget_max
        text notes
        timestamptz contacted_at
        timestamptz qualified_at
        timestamptz created_at
        timestamptz updated_at
    }

    opportunities {
        uuid id PK
        uuid lead_id FK
        uuid client_id FK
        uuid vehicle_id FK
        uuid assigned_to FK
        opportunity_status status
        varchar title
        numeric offered_price
        numeric discount
        numeric final_price
        smallint probability
        date expected_close
        timestamptz closed_at
        text lost_reason
        text notes
        timestamptz created_at
        timestamptz updated_at
    }

    activities {
        uuid id PK
        varchar title
        activity_type type
        activity_status status
        timestamptz scheduled_start
        timestamptz scheduled_end
        timestamptz completed_at
        text outcome
        text next_action
        uuid assigned_to FK
        uuid client_id FK
        uuid lead_id FK
        uuid opportunity_id FK
        uuid vehicle_id FK
        timestamptz created_at
        timestamptz updated_at
    }

    users ||--o{ refresh_tokens : "tiene"
    users ||--o{ clients : "gestiona"
    users ||--o{ leads : "asignado"
    users ||--o{ opportunities : "asignado"
    users ||--o{ activities : "asignado"
    clients ||--o{ leads : "genera"
    clients ||--o{ opportunities : "tiene"
    clients ||--o{ activities : "tiene"
    leads ||--o{ opportunities : "convierte en"
    leads ||--o{ activities : "tiene"
    opportunities ||--o{ activities : "tiene"
    vehicles ||--o{ opportunities : "asociado"
    vehicles ||--o{ activities : "asociado"
```

---

## Enums de la base de datos

| Enum | Valores |
|------|---------|
| `user_role` | `admin`, `manager`, `sales_agent` |
| `lead_source` | `web`, `referral`, `walk_in`, `phone`, `social_media`, `other` |
| `lead_status` | `new`, `contacted`, `qualified`, `unqualified`, `converted` |
| `opportunity_status` | `prospecting`, `needs_analysis`, `proposal`, `negotiation`, `closed_won`, `closed_lost` |
| `activity_type` | `call`, `email`, `visit`, `whatsapp`, `meeting`, `test_drive`, `delivery` |
| `activity_status` | `scheduled`, `completed`, `cancelled`, `rescheduled` |
| `fuel_type` | `gasoline`, `diesel`, `hybrid`, `electric`, `other` |
| `transmission_type` | `manual`, `automatic`, `cvt` |
| `vehicle_condition` | `new`, `used`, `certified_used` |

---

## API REST

```
POST   /api/auth/login
POST   /api/auth/logout
POST   /api/auth/refresh
POST   /api/auth/register
GET    /api/auth/me

GET    /api/clients
POST   /api/clients
GET    /api/clients/:id
PUT    /api/clients/:id
DELETE /api/clients/:id

GET    /api/vehicles
POST   /api/vehicles
GET    /api/vehicles/:id
PUT    /api/vehicles/:id
DELETE /api/vehicles/:id
PATCH  /api/vehicles/:id/availability

GET    /api/leads
POST   /api/leads
GET    /api/leads/:id
PUT    /api/leads/:id
DELETE /api/leads/:id

GET    /api/opportunities/pipeline
POST   /api/opportunities
GET    /api/opportunities/:id
PATCH  /api/opportunities/:id/status
POST   /api/opportunities/:id/close-won
POST   /api/opportunities/:id/close-lost

GET    /api/activities/upcoming
GET    /api/activities/overdue
POST   /api/activities
GET    /api/activities/:id
PUT    /api/activities/:id
DELETE /api/activities/:id
PATCH  /api/activities/:id/complete
PATCH  /api/activities/:id/reschedule

GET    /api/calendar
GET    /api/dashboard
```

---

## Pantallas y flujo de navegación

```mermaid
flowchart TD
    Browser([Navegador]) --> Root["/"]
    Root -->|redirect| Calendar

    subgraph Public["Público"]
        Login["/login\nFormulario de acceso"]
    end

    subgraph Protected["Protegido (requiere sesión)"]
        Calendar["/calendar\nCalendario de actividades\n─────────────────\nFullCalendar + Sidebar\nModal crear/editar actividad"]

        Pipeline["/pipeline\nKanban de oportunidades\n─────────────────\nDrag & drop entre etapas\nTarjetas de oportunidad"]

        subgraph LeadsSection["Leads"]
            LeadsList["/leads\nListado con filtros"]
            LeadNew["/leads/new\nFormulario creación"]
            LeadDetail["/leads/:id\nDetalle del lead"]
            LeadEdit["/leads/edit/:id\nEditar lead"]
        end

        subgraph ClientsSection["Clientes"]
            ClientsList["/clients\nListado con filtros"]
            ClientNew["/clients/new\nFormulario creación"]
            ClientDetail["/clients/:id\nDetalle del cliente"]
            ClientEdit["/clients/edit/:id\nEditar cliente"]
        end

        subgraph VehiclesSection["Vehículos"]
            VehicleGrid["/vehicles\nGrid con filtros"]
            VehicleNew["/vehicles/new\nFormulario creación"]
            VehicleDetail["/vehicles/:id\nDetalle del vehículo"]
            VehicleEdit["/vehicles/edit/:id\nEditar vehículo"]
        end

        Reports["/reports\nDashboard con gráficos\n─────────────────\nRecharts: ventas,\nfunnel, actividades"]

        AdminUsers["/admin/users\nGestión de usuarios\n(solo admin)"]
    end

    Login -->|auth OK → cookie + token| Calendar

    Calendar <-->|crear actividad| LeadDetail
    Calendar <-->|crear actividad| ClientDetail

    Pipeline -->|"nueva oportunidad\n(desde lead)"| LeadsList
    Pipeline <-->|ver detalle| LeadDetail

    LeadsList --> LeadNew
    LeadsList --> LeadDetail
    LeadDetail --> LeadEdit
    LeadDetail -->|convertir| Pipeline

    ClientsList --> ClientNew
    ClientsList --> ClientDetail
    ClientDetail --> ClientEdit
    ClientDetail -->|ver leads| LeadsList

    VehicleGrid --> VehicleNew
    VehicleGrid --> VehicleDetail
    VehicleDetail --> VehicleEdit
    VehicleDetail -->|asociar a oportunidad| Pipeline

    Reports -.->|lee datos de| Calendar
    Reports -.->|lee datos de| Pipeline
    Reports -.->|lee datos de| LeadsList

    AdminUsers -.->|solo role=admin| AdminUsers
```

---

## Flujo de autenticación

```mermaid
sequenceDiagram
    participant B as Navegador
    participant FE as Frontend (Astro SSR)
    participant BE as Backend (Axum)
    participant DB as PostgreSQL

    B->>FE: GET /ruta-protegida
    FE->>FE: Middleware: ¿refresh_token en cookie?
    alt Sin cookie
        FE-->>B: Redirect /login
    end

    B->>FE: POST /api/auth/login {email, password}
    FE->>BE: POST /api/auth/login
    BE->>DB: SELECT user WHERE email
    DB-->>BE: user row
    BE->>BE: Argon2 verify(password, hash)
    BE->>DB: INSERT refresh_token
    BE-->>FE: {access_token, ...}
    FE-->>B: Set-Cookie: refresh_token (httpOnly)\nBody: {access_token}
    B->>B: sessionStorage.setItem(access_token)

    B->>FE: GET /ruta (Authorization: Bearer <token>)
    FE->>BE: Proxy con Bearer token
    BE->>BE: JWT verify + extraer claims
    BE-->>FE: 200 + datos
    FE-->>B: Página renderizada

    Note over B,DB: Cuando access_token expira (15 min)
    B->>FE: POST /api/auth/refresh (cookie auto)
    FE->>BE: POST /api/auth/refresh
    BE->>DB: SELECT refresh_token WHERE hash AND NOT revoked
    BE-->>FE: {access_token nuevo}
    FE-->>B: Nuevo access_token
```

---

## Flujo de ventas (ciclo completo)

```mermaid
flowchart LR
    Prospecto([Prospecto]) -->|"lead_source:\nweb/referral/etc."| Lead

    subgraph Lead["Lead (Calificación)"]
        L1[new]-->L2[contacted]-->L3[qualified]
        L3 -->|no aplica| L4[unqualified]
    end

    L3 -->|convertir| Oportunidad

    subgraph Oportunidad["Oportunidad (Pipeline)"]
        O1[prospecting]-->O2[needs_analysis]-->O3[proposal]-->O4[negotiation]
        O4-->O5[closed_won]
        O4-->O6[closed_lost]
    end

    Oportunidad -->|asignar| Vehiculo[(Vehículo\ndel inventario)]

    subgraph Actividades["Actividades de seguimiento"]
        A1[call]
        A2[test_drive]
        A3[meeting]
        A4[delivery]
    end

    Lead --- Actividades
    Oportunidad --- Actividades
    O5 -->|marcar is_available=false| Vehiculo
```

---

## Relación entre pantallas y módulos backend

```mermaid
graph LR
    subgraph Pantallas
        CAL["/calendar"]
        PIP["/pipeline"]
        LEA["/leads"]
        CLI["/clients"]
        VEH["/vehicles"]
        REP["/reports"]
        ADM["/admin/users"]
    end

    subgraph API
        ACAL["/api/calendar\n/api/activities"]
        AOPP["/api/opportunities"]
        ALEA["/api/leads"]
        ACLI["/api/clients"]
        AVEH["/api/vehicles"]
        ADASH["/api/dashboard"]
        AUSE["/api/auth"]
    end

    CAL --> ACAL
    PIP --> AOPP
    PIP --> ALEA
    LEA --> ALEA
    LEA --> ACLI
    CLI --> ACLI
    VEH --> AVEH
    REP --> ADASH
    ADM --> AUSE
```

---

## Arranque rápido

```bash
# 1. Base de datos
docker compose up -d postgres

# 2. Backend (desde raíz)
pnpm dev:backend        # cargo watch — recarga en cambios

# 3. Frontend (desde raíz)
pnpm dev                # astro dev

# O ambos juntos
pnpm dev:all
```

Migraciones se aplican automáticamente al arrancar el backend (`sqlx migrate run`).

---

## Roles de usuario

| Rol | Acceso |
|-----|--------|
| `admin` | Todo + gestión de usuarios (`/admin/users`) |
| `manager` | Todo excepto gestión de usuarios |
| `sales_agent` | Solo sus propios registros asignados |
