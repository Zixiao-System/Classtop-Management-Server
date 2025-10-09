# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ClassTop Management Server is a centralized management server for [ClassTop](https://github.com/Zixiao-System/classtop) clients. It manages multiple ClassTop client devices, syncs course/schedule data, and provides a web-based admin interface.

**Technology Stack:**
- Backend: Rust (Actix-Web 4.9, SQLx)
- Database: PostgreSQL (SQL Server support planned)
- API Documentation: utoipa (OpenAPI/Swagger)
- Frontend: Vue 3 + Vite + MDUI 2 (Material Design)

**Platform Support:**
- ✅ Windows Server (fully supported)
- ✅ Linux (fully supported)
- ✅️ macOS (fully supported，Apple silicon & Intel)

## Development Commands

### Backend (Rust)

```bash
# Run in development mode
cargo run

# Run in release mode
cargo run --release

# Build release binary
cargo build --release

# Check code without building
cargo check

# Run tests (when added)
cargo test
```

### Frontend (Vue 3)

```bash
# Install dependencies
cd frontend
npm install

# Run development server (with API proxy)
npm run dev

# Build for production (outputs to ../static/)
npm run build

# Preview production build
npm run preview
```

### Environment Setup

1. Copy `.env.example` to `.env`
2. Configure PostgreSQL database:
   ```env
   DATABASE_URL=postgresql://username:password@localhost:5432/classtop
   HOST=0.0.0.0
   PORT=8765
   ```

### Database

**Supported Databases:**
- PostgreSQL 14+ (fully supported)
- Microsoft SQL Server (planned - see `docs/MSSQL_STATUS.md`)

**Migration System:**
- Migrations run automatically on startup via `db::run_migrations(pool)`
- Migration file: `migrations/001_initial_postgresql.sql`
- Uses SQLx with connection pooling (max 10 connections, 30s timeout)

## Architecture

### Backend Structure (src/)

**Main Entry (`main.rs`):**
- Initializes logger, config, database pool
- Runs database migrations
- Sets up Actix-Web server with CORS, compression, logging
- Mounts API routes, Swagger UI, ReDoc, and static files

**Module Organization:**
- `config.rs` - Environment variable configuration via dotenvy
- `db.rs` - Database pool creation, migrations, Repository pattern for all DB operations
- `models.rs` - All data models with utoipa schemas (Client, Course, ScheduleEntry, SyncRequest, etc.)
- `handlers.rs` - Actix-Web route handlers with utoipa path annotations
- `routes.rs` - OpenAPI specification and route configuration
- `error.rs` - Custom error types (AppError) with Actix ResponseError implementation

**Key Design Patterns:**

1. **Repository Pattern (`db::repository::Repository`):**
   - All database operations centralized in Repository struct
   - Takes DbPool, provides methods for CRUD operations
   - Returns `AppResult<T>` (custom Result type with AppError)

2. **Multi-Client Architecture:**
   - Each client has a unique UUID
   - Courses/schedule entries are linked to clients via `client_id`
   - Client data includes `course_id_on_client` and `entry_id_on_client` to map to original IDs
   - Supports client status tracking (online/offline/error)

3. **Data Synchronization:**
   - Clients POST to `/api/sync` with courses and schedule_entries arrays
   - Server uses UPSERT logic (insert new, update existing based on client_id + *_on_client IDs)
   - Sync logs stored in `sync_logs` table
   - Updates client's `last_sync` timestamp and status

4. **API Response Format:**
   - Success: `{"success": true, "data": {...}}`
   - Error: `{"detail": "error message"}` (from AppError ResponseError impl)

### Database Schema

**Tables:**
- `clients` - Registered client devices (uuid, name, api_url, status, last_sync)
- `courses` - Synced courses (client_id, course_id_on_client, name, teacher, color)
- `schedule_entries` - Synced schedules (client_id, entry_id_on_client, course_id, day_of_week, times, weeks as JSON)
- `sync_logs` - Sync history (client_id, sync_type, status, counts, timestamps)
- `settings` - Server configuration (key-value pairs)

**Relationships:**
- courses.client_id → clients.id (CASCADE DELETE)
- schedule_entries.client_id → clients.id (CASCADE DELETE)
- schedule_entries.course_id → courses.id (CASCADE DELETE)
- UNIQUE constraints on (client_id, course_id_on_client) and (client_id, entry_id_on_client)

### API Endpoints

Access interactive docs at: `http://localhost:8765/api/docs` (Swagger UI)

**Client Management:**
- `GET /api/clients` - List all clients
- `POST /api/clients/register` - Register new client
- `GET /api/clients/{id}` - Get client details
- `PUT /api/clients/{id}` - Update client
- `DELETE /api/clients/{id}` - Delete client
- `GET /api/clients/{id}/courses` - Get client's courses
- `GET /api/clients/{id}/schedule` - Get client's schedule

**Data Sync:**
- `POST /api/sync` - Sync data from client (body: `{client_uuid, courses[], schedule_entries[]}`)

**Statistics:**
- `GET /api/statistics` - Server-wide stats (total clients, online clients, courses, entries)
- `GET /api/statistics/clients` - Per-client statistics

**Settings:**
- `GET /api/settings` - Get all settings
- `GET /api/settings/{key}` - Get specific setting
- `PUT /api/settings/{key}` - Update setting

**Health:**
- `GET /api/health` - Health check endpoint

## Working with the Codebase

### Adding New API Endpoints

1. Define models in `models.rs` with `#[derive(ToSchema)]`
2. Create handler in `handlers.rs` with `#[utoipa::path(...)]` annotation
3. Add route in `routes.rs` `configure_routes()` function
4. Add path and schemas to `ApiDoc` OpenApi derive in `routes.rs`

### Database Operations

Use the Repository pattern:
```rust
let repo = Repository::new(pool.get_ref().clone());
let result = repo.method_name(...).await?;
```

All DB queries use SQLx with positional parameters (`$1`, `$2`, etc.) for PostgreSQL.

### Error Handling

Return `AppResult<T>` from functions. Use `?` operator to propagate errors. AppError automatically converts to HTTP responses:
- AppError::NotFound → 404
- AppError::BadRequest → 400
- AppError::Database/Internal → 500

### Frontend

**Structure (`frontend/`):**
- Built with Vue 3 (Composition API) + Vite + MDUI 2
- Build output goes to `../static/` directory (served by backend)
- Development server runs on port 5173 with API proxy to backend

**Component Architecture:**
- `App.vue` - Main application shell with top bar, tabs, theme toggle
- `DashboardView.vue` - Server statistics and client overview
- `ClientsView.vue` - Client management (list, register, view, delete)
- `DataView.vue` - Data viewing (courses and schedules by client)
- `api.js` - Centralized API request functions

**Key Features:**
- Material Design 3 with MDUI 2 Web Components
- Light/dark/auto theme switching (follows system preference)
- Responsive layout
- MDUI components: cards, buttons, dialogs, tabs, text fields, etc.
- All `mdui-*` tags configured as custom elements in Vite

**Development Workflow:**
1. Run `npm run dev` in `frontend/` for hot-reload development
2. Vite dev server proxies `/api/*` to `http://localhost:8765`
3. Run `npm run build` to generate production build in `../static/`
4. Backend serves static files from `static/` directory

**API Integration:**
- All API calls in `src/api.js` with error handling
- Uses `fetch()` with JSON responses
- Extracts `data` field from `{success: true, data: {...}}` responses
- Displays errors via MDUI snackbar

## Important Notes

- Database migrations run on every startup (uses `ok()` to ignore "already exists" errors)
- Only PostgreSQL is currently supported (SQL Server support planned)
- No authentication/authorization implemented yet (security warning in README)
- CORS is permissive (`Cors::permissive()`) - tighten for production
- Client UUIDs must be unique (enforced by database)
- Schedule weeks stored as JSON array in TEXT column