# Murmur

Twitch chat management SaaS — a simple, zero-config alternative to Twitchat.

## Why

Learning **Rust**, **Nuxt/Vue 3** and **DevOps/K8s** by building a real product.

## Stack

- **Backend**: Rust (tokio, rustls) — IRC over TLS, async
- **Frontend**: Nuxt (Vue 3) — dashboard, config, OBS overlays
- **Infra**: K3s, Traefik (coming soon)

## Architecture

Clean architecture (ports & adapters) on the backend:

```
backend/src/
├── domain/          # Entities, value objects, domain errors
├── applications/    # Use cases, ports (traits)
└── adapters/        # Concrete implementations (Twitch IRC, etc.)
```

## Getting started

```bash
# Backend
cd backend
cargo watch -x run

# Frontend
cd frontend
pnpm dev
```

## Status

Work in progress — the backend reads Twitch chat in real-time via IRC over TLS.
