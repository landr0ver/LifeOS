# LifeOS

Self-hosted PWA organizer with Kanban, Habit Tracker, Knowledge Graph Notes, Calendar, AI (Claude API), and GitHub integration.

## Tech Stack
- **Frontend**: SvelteKit (TypeScript) in `frontend/`
- **Backend**: Rust + Axum in `backend/`
- **Database**: PostgreSQL (via docker-compose)
- **AI**: Claude API (Anthropic)

## Development

### Backend
```bash
cd backend
cargo run          # Start dev server on :3000
cargo test         # Run tests
cargo clippy       # Lint
```

### Frontend
```bash
cd frontend
npm install        # Install dependencies
npm run dev        # Start dev server on :5173
npm run build      # Production build
npm run check      # Type check
```

### Database
```bash
docker compose up -d db          # Start PostgreSQL
cd backend && sqlx migrate run   # Run migrations
```

## Project Structure
- `backend/src/routes/` — API route handlers
- `backend/src/services/` — Business logic
- `backend/src/models/` — Database models (SQLx)
- `backend/migrations/` — SQL migration files
- `frontend/src/routes/` — SvelteKit pages
- `frontend/src/lib/components/` — Reusable components
- `frontend/src/lib/stores/` — Svelte stores
- `frontend/src/lib/services/` — API client functions
- `feature.md` — Feature wishlist (auto-syncs to GitHub issues)

## Conventions
- Backend API prefix: `/api/`
- Use UUID for all primary keys
- Timestamps as TIMESTAMPTZ
- CSS variables for theming (70s retro palette)
