# Getting Started with LifeOS

A self-hosted PWA organizer with Kanban, Habit Tracker, Knowledge Graph Notes, Calendar, AI (Claude API), and GitHub integration.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- [Rust](https://rustup.rs/) (1.75+ recommended)
- [Node.js](https://nodejs.org/) 22+
- [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli): `cargo install sqlx-cli --no-default-features --features postgres`

## Quick Start (Docker)

Run the entire stack with one command:

```bash
docker compose up
```

This starts PostgreSQL, the Rust backend (port 3000), and the SvelteKit frontend (port 5173). Open **http://localhost:5173** in your browser.

Default login password: `changeme` (set via `AUTH_PASSWORD` env var).

## Development Setup

### 1. Configure Environment

```bash
cd backend
cp .env.example .env
```

Edit `backend/.env` with your values:

```env
DATABASE_URL=postgres://lifeos:lifeos_dev@localhost/lifeos
AUTH_PASSWORD=changeme
CLAUDE_API_KEY=sk-ant-xxxxx       # Optional — enables AI features
GITHUB_TOKEN=ghp_xxxxx            # Optional — enables GitHub sync
RUST_LOG=info
HOST=0.0.0.0
PORT=3000
```

### 2. Start the Database

```bash
docker compose up -d db
```

### 3. Run Migrations

```bash
cd backend
sqlx migrate run
```

### 4. Start the Backend

```bash
cd backend
cargo run
```

The API server starts on **http://localhost:3000**.

### 5. Start the Frontend

```bash
cd frontend
npm install
npm run dev
```

The dev server starts on **http://localhost:5173**. API requests are proxied to the backend automatically.

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://lifeos:lifeos_dev@localhost/lifeos` |
| `AUTH_PASSWORD` | Login password | `changeme` |
| `CLAUDE_API_KEY` | Anthropic API key (optional) | — |
| `GITHUB_TOKEN` | GitHub personal access token (optional) | — |
| `RUST_LOG` | Log level | `info` |
| `HOST` | Backend bind address | `0.0.0.0` |
| `PORT` | Backend port | `3000` |
| `DB_PASSWORD` | PostgreSQL password (docker-compose) | `lifeos_dev` |

## Accessing the App

1. Open **http://localhost:5173** in your browser
2. Log in with the password set in `AUTH_PASSWORD`
3. You'll land on the Dashboard with links to all modules

## Installing as PWA on iPhone

1. Open **https://your-server-ip:5173** in Safari (HTTPS required — use a reverse proxy like [Caddy](https://caddyserver.com/) or a self-signed certificate)
2. Tap the **Share** button (square with arrow)
3. Select **Add to Home Screen**
4. The app will run in standalone mode like a native app

## Useful Commands

| Command | What it does |
|---------|-------------|
| `cd backend && cargo run` | Start backend dev server |
| `cd backend && cargo test` | Run backend tests |
| `cd backend && cargo clippy` | Lint Rust code |
| `cd frontend && npm run dev` | Start frontend dev server |
| `cd frontend && npm run build` | Production build |
| `cd frontend && npm run check` | TypeScript type check |
| `docker compose up -d db` | Start only PostgreSQL |
| `docker compose up` | Start full stack |
| `cd backend && sqlx migrate run` | Run database migrations |
