# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Minecarts is a web app for managing Minecraft servers on [Railway](https://railway.com). It uses Railway's OAuth and GraphQL API to let users manage projects and services (create, update, delete) on their Railway account.

## Development Commands

### Server (Rust/Axum)
```bash
cargo build                  # build the server
cargo run                    # run the server (listens on :3001 by default)
cargo check                  # type-check without building
```

### Frontend (Vue/Vite)
```bash
cd web
pnpm install                 # install dependencies
pnpm dev                     # dev server with HMR (port 5173)
pnpm build                   # type-check (vue-tsc) + production build
```

### Environment Variables
Required in `.env.local` at the repo root:
- `RAILWAY_CLIENT_ID` — Railway OAuth client ID
- `RAILWAY_CLIENT_SECRET` — Railway OAuth client secret
- `RAILWAY_REDIRECT_URI` — defaults to `http://localhost:5173/api/auth/callback`
- `SERVER_HOST` — defaults to `0.0.0.0:3001`
- `REDIS_URL` — Redis connection string (e.g. `redis://localhost:6379`), required

## Architecture

### Monorepo layout
- `server/` — Rust backend (Axum). Workspace member defined in root `Cargo.toml`.
- `web/` — Vue 3 + TypeScript frontend with Tailwind CSS 4, Pinia for state, Vue Router.

### API proxy
Vite proxies `/api/*` requests to the Rust server at `localhost:3001`, stripping the `/api` prefix. The frontend always calls `/api/...` paths; the server routes have no `/api` prefix.

### Authentication flow
OAuth with Railway (`backboard.railway.com`). The server handles the full OAuth flow (login redirect, code exchange, token refresh) and stores sessions in Redis (key `session:{uuid}`, JSON-serialized). A `session_id` cookie identifies the user. The `UserSession` extractor (`server/src/extractors.rs`) validates sessions and auto-refreshes expired tokens. CSRF states remain in-memory (short-lived, only needed during OAuth flow).

### Railway API integration
`server/src/railway/` contains typed clients for Railway's GraphQL v2 API. GraphQL queries live in `.gql` files under `server/src/railway/graphql/` and are embedded at compile time via `include_str!`. Each domain entity (projects, services) has methods on its struct for API operations.

### Server route structure
Routes in `server/src/routes/` map directly to Axum handlers. Services are nested under projects: `/projects/{project_id}/services/...`. State is `Arc<AppState>` shared across handlers.

### Frontend structure
- `web/src/api/` — fetch wrappers for each backend endpoint
- `web/src/stores/` — Pinia stores (auth, projects, services) re-exported from `stores/index.ts`
- `web/src/views/` — page-level components (`HomeView`, `LoginView`)
- `web/src/components/` — reusable UI components (modals, cards, pickers)
- Routes with `meta: { skipAuth: true }` bypass the auth guard

### Development Instructions
1. Always `git add` new files you create.