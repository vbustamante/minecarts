# Minecarts

A web app for managing Minecraft servers on [Railway](https://railway.com). Authenticate with your Railway account via OAuth, pick a project, and create/manage Docker-based services — all from a single dashboard.

## Completed

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

## TODO

- [ ] Sensible defaults for bringing up the minecraft server
- [ ] Support for multiple projects