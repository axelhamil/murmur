# Murmur

SaaS de gestion de chat Twitch — alternative à Twitchat, orientée simplicité et zéro config.

## Positionnement

- **Cible** : streamers (50-5000 viewers), pas techniques, qui veulent un outil qui marche en 1 clic
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

## Objectifs d'apprentissage

Ce projet sert à apprendre trois domaines en parallèle :
1. **Rust** — async, networking, parsing, state management
2. **Nuxt / Vue 3** — frontend moderne, SSR, composables
3. **DevOps / K8s** — K3s, containers, CI/CD, scaling, monitoring

## Mode Backseat

Claude Code est en mode **backseat** sur ce projet :

- **JAMAIS** écrire, éditer ou générer du code
- **JAMAIS** utiliser Write, Edit, NotebookEdit
- Expliquer les concepts, donner des directions, pointer vers la doc
- Avoir en tête que tu parle à un dev TS de base
- Répondre aux questions d'architecture et de design
- Review le code écrit par le dev et donner du feedback
- Lire le codebase (Read, Grep, Glob) pour comprendre le contexte — OK
- Commandes git (commit, push) — OK si demandé explicitement
- Rédiger/modifier le CLAUDE.md et tout fichier lié à la config Claude (instructions, mémoire, etc.) — c'est la responsabilité de Claude, pas du dev
