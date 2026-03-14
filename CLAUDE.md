# Murmur

SaaS de gestion de chat Twitch — alternative à Twitchat, orientée simplicité et zéro config.

## Positionnement

- **Cible** : streamers + viewers
  - Streamers (50-5000 viewers) : dashboard, bot, config, overlays — pas techniques, veulent un outil qui marche en 1 clic
  - Viewers : chat enrichi indépendant (meilleure UX que le chat Twitch natif, redimensionnable, customisable) — utilisable sur n'importe quel stream, même si le streamer n'utilise pas Murmur
- **Avantage vs Twitchat** : zéro setup (pas d'app Twitch à créer, pas de self-host), bot 24/7, UX simple
- **Modèle** : freemium (tiers à définir quand le MVP tourne)

## Features cibles

### MVP
- Chat Twitch temps réel (IRC over TLS via rustls)
- Bot Twitch (commandes custom, rewards, games)
- Overlays OBS (browser source, servis par le backend)

### Post-MVP
- Polls / Raffles / Bingo via chat
- TTS (Web Speech API côté frontend)
- Hate raid protection
- Timers / Countdowns
- **Auto-threading** : regroupement automatique des messages consécutifs d'un même auteur quand ils sont proches temporellement et pas trop entrelacés avec d'autres — feature custom, pas dans Twitchat

## Stack

- **Backend** : Rust natif, async (tokio), IRC over TLS (rustls)
- **Frontend** : Nuxt (Vue 3)
- **BDD** : Postgres (users, config, OAuth tokens, commandes custom)
- **State partagé** : Redis (state live partagé entre pods/process)
- **Infra** : K3s sur VPS, Traefik (inclus), KEDA plus tard pour le scale-to-zero

## Politique de dépendances

**Deps autorisées** (plomberie / infra / crypto) :
- `tokio` — async runtime
- `rustls` — TLS
- Serveur HTTP (hyper ou minimal)
- `tokio-postgres` — driver Postgres
- `redis-rs` — driver Redis

**Écrit à la main** (logique métier / apprentissage) :
- Parsing IRC
- Logique bot / commandes
- Auto-threading
- Hate raid detection
- State management
- Serialization des messages

**Règle** : si c'est de la plomberie → dep. Si c'est de la logique métier → à la main.

## Architecture

- **Hexagonale (ports & adapters)** — le domain et les usecases ne connaissent pas les protocoles (IRC, WebSocket, etc.)
  - `applications/ports/` : traits abstraits (`ChatConnection`, `ChatConnector`) — contrat métier, pas de couplage protocole
  - `applications/usecases/` : orchestration métier, dépend uniquement des ports
  - `adapters/services/` : implémentations concrètes (ex: `TwitchIrcConnector` implémente `ChatConnector`)
  - `domain/` : entités métier (`Message`, `Channel`), erreurs domain
- Les détails protocolaires (IRC commands, parsing, PING/PONG) restent dans l'adapter, jamais dans les ports ou le domain
- Backend Rust = process long-running, maintient les connexions chat persistantes pour chaque user
- Frontend Nuxt = dashboard, config, overlays
- OAuth Twitch : authorization code flow (le serveur gère le callback)
- Overlays OBS : URL à coller dans OBS, servies par le backend
- Scaling : un process Rust multiplexe N connexions chat (async tokio), scale horizontalement en pods K3s
- Multiplexing WebSocket : une seule connexion IRC par channel, broadcast vers N viewers/streamers connectés en WebSocket — jamais de connexion IRC dupliquée pour le même channel

## DevOps

### Philosophie

Le dev a de l'expérience Docker/Compose mais découvre Kubernetes. Approche progressive : d'abord ça tourne en local, puis en container, puis en cluster.

### Phase 1 — Dev local (actuel)

- Backend : `cargo run`
- Frontend : `npm run dev`
- Pas de container, pas d'orchestration
- Postgres et Redis en Docker Compose pour les données

### Phase 2 — Containerisation

- Dockerfile multi-stage pour le backend Rust (build + runtime minimal)
- Dockerfile pour le frontend Nuxt
- Docker Compose qui orchestre tout : backend, frontend, Postgres, Redis
- Variables d'env pour la config (pas de secrets hardcodés)

### Phase 3 — CI/CD

- GitHub Actions : build, test, push images vers un registry (GHCR ou Docker Hub)
- Lint + cargo clippy dans la CI
- Build d'images multi-arch (amd64 + arm64) si RPi utilisé

### Phase 4 — K3s en prod

- **K3s** sur VPS (2-4 Go RAM minimum) — Kubernetes léger, Traefik inclus comme ingress
- Manifests Kubernetes (Deployments, Services, Ingress) ou Helm charts
- **Traefik** gère le TLS (Let's Encrypt auto) et le routing
- **Postgres** : soit en pod K3s (simple), soit managed (plus fiable)
- **Redis** : en pod K3s suffit pour le MVP
- Un seul pod backend au départ, scale horizontal plus tard

### Phase 5 — Scale et monitoring (post-MVP)

- **KEDA** : scale-to-zero quand aucun streamer connecté (économie VPS)
- Monitoring : Prometheus + Grafana (métriques connexions, messages/s, latence)
- Logs structurés (tracing crate côté Rust)
- Alerting sur les déconnexions IRC

### Contraintes

- Le backend Rust est un **process long-running** — pas du serverless. Pas de cold start, connexions IRC persistantes.
- Un pod backend gère N streamers en async (tokio). Scale horizontal = plusieurs pods, avec Redis pour partager le state.
- Le RPi 3B+ (1 Go RAM) est trop juste pour K3s — préférer un VPS pour la prod, le RPi pour expérimenter Docker Compose.

## Objectifs d'apprentissage

Ce projet sert à apprendre trois domaines en parallèle :
1. **Rust** — async, networking, parsing, state management
2. **Nuxt / Vue 3** — frontend moderne, SSR, composables
3. **DevOps / K8s** — K3s, containers, CI/CD, scaling, monitoring

## Conventions git

- Messages de commit **toujours en anglais**
- Format : `type(scope): description` (conventional commits)

## Mode Sensei

Claude Code est en mode **sensei** sur ce projet — mentor socratique adaptatif :

- **JAMAIS** écrire, éditer ou générer du code source
- **JAMAIS** utiliser Write, Edit, NotebookEdit sur du code
- Guider par la question plutôt que par la réponse (design, archi, trade-offs)
- Répondre directement quand c'est factuel (syntaxe, API, nom de fonction)
- Profil dev : développeur TypeScript qui apprend Rust, async et le networking
- Review le code écrit par le dev et donner du feedback
- Lire le codebase proactivement (Read, Grep, Glob) — ne jamais demander au dev de montrer son code
- Vérifier la doc officielle (WebSearch/WebFetch) avant d'affirmer sur une lib ou une API
- Commandes git (commit, push) — OK si demandé explicitement
- Rédiger/modifier le CLAUDE.md et tout fichier lié à la config Claude — c'est la responsabilité de Claude, pas du dev

## Historique

- 2026-03-14 : connexion IRC TLS fonctionnelle, écoute du chat en read-only
- 2026-03-14 : architecture hexagonale en place (ports/adapters), IRC découplé du domain
- 2026-03-14 : refactoring — IrcCommand déplacé dans l'adapter, ports épurés des détails protocole
- 2026-03-15 : parser IRC complet (IrcFrame) — tags IRCv3, prefix, command, params, trailing
- 2026-03-15 : ChatNotification dans le domain, ports event-driven (next_notification)
- 2026-03-15 : PING/PONG + CAP REQ (tags/commands) + PRIVMSG → Message domain — chat temps réel fonctionnel
- 2026-03-15 : ChatConnectionError dédié, AppError::Connection, gestion EOF
