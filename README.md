# Minecarts

A web app for managing Minecraft servers on [Railway](https://railway.com). Authenticate with your Railway account via OAuth, pick a project, and create/manage Docker-based services — all from a single dashboard.

## Dependencies

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Node.js](https://nodejs.org/) (v18+) and [pnpm](https://pnpm.io/)
- [Docker](https://docs.docker.com/get-docker/) (for Redis via docker-compose)

## Running

### Environment variables

Create a `.env` file in the repo root with:

```
RAILWAY_CLIENT_ID=<your Railway OAuth client ID>
RAILWAY_CLIENT_SECRET=<your Railway OAuth client secret>
RAILWAY_REDIRECT_URI=http://localhost:5173/api/auth/callback
REDIS_URL=redis://localhost:6379
SERVER_HOST=0.0.0.0:3001
```

### Redis (docker-compose)

```bash
docker-compose up -d
```

This starts a Redis 7 instance on port 6379.

### Server (Rust/Axum)

```bash
cargo run
```

The server listens on `localhost:3001` by default.

### Frontend (Vue/Vite)

```bash
cd web
pnpm install
pnpm dev
```

The dev server runs on `localhost:5173` and proxies `/api/*` requests to the Rust server.

## Project Status
### Completed

- [x] OAuth login with Railway (authorization code flow with token refresh)
- [x] Session management with Redis, Authorization header-based auth
- [x] OAuth error handling with user-facing error messages on the login page
- [x] Header with project picker, user avatar, and logout button
- [x] Project listing and selection
- [x] Service listing with search, sorting, and auto-refresh
- [x] Service creation via modal
- [x] Service details side panel with tabs (details, variables)
- [x] Service deletion with confirmation modal
- [x] Environment variable management (list, add, edit, delete)
- [x] Sensible defaults for bringing up the minecraft server

### TODO

- [ ] Allowing setting env variables when creating the service
- [ ] Allowing managing the external URL and port of the server