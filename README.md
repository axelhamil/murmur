# Murmur

Twitch chat management SaaS — a simple, zero-config alternative to Twitchat.

## Why Murmur?

Setting up Twitchat means creating a Twitch app, managing OAuth credentials, and often self-hosting. That's fine for tech-savvy streamers, but most just want a tool that works.

Murmur is built for streamers (50–5,000 viewers) who don't want to deal with setup. One click, your chat is connected, your bot is running, your overlays are live. No Twitch app to create, no server to manage, no config to tweak.

## Features

### MVP
- **Real-time Twitch chat** — IRC over TLS, low latency
- **Twitch bot** — custom commands, channel point rewards, chat games
- **OBS overlays** — browser sources served by the backend, ready to drop into OBS

### Planned
- Polls, raffles, bingo via chat
- TTS (Web Speech API)
- Hate raid protection
- Timers & countdowns
- **Auto-threading** — automatically groups consecutive messages from the same author when they're close in time and not interleaved with others

## Stack

- **Backend**: Rust (tokio, rustls) — async IRC over TLS
- **Frontend**: Nuxt (Vue 3) — dashboard, config, OBS overlays
- **Database**: Postgres — users, config, OAuth tokens, custom commands
- **Shared state**: Redis — live state shared across pods
- **Infra**: K3s on VPS, Traefik, KEDA (later, for scale-to-zero)

## Architecture

Hexagonal architecture (ports & adapters) on the backend. The domain and use cases never depend on protocols — only on abstract traits.

```
backend/src/
├── domain/          # Entities (Message, Channel), value objects, domain errors
├── applications/    # Use cases, ports (ChatConnection, ChatConnector)
└── adapters/        # Concrete implementations (Twitch IRC, etc.)
```

Protocol-specific details (IRC commands, parsing, PING/PONG) stay in the adapter layer.

## Getting started

```bash
# Backend
cd backend
cargo watch -x run

# Frontend (coming soon)
cd frontend
pnpm dev
```

## Status

Work in progress. The backend connects to Twitch IRC over TLS and reads chat in real-time. Next up: IRC message parsing, PING/PONG handling, and requesting IRCv3 tags for full message metadata.

## Business model

Freemium — tiers to be defined once the MVP is running.
