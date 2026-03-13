# Murmur

SaaS de gestion de chat Twitch — alternative simple et sans config à Twitchat.

## Pourquoi

Apprendre **Rust**, **Nuxt/Vue 3** et le **DevOps/K8s** en construisant un vrai produit.

## Stack

- **Backend** : Rust (tokio, rustls) — connexion IRC over TLS, async
- **Frontend** : Nuxt (Vue 3) — dashboard, config, overlays OBS
- **Infra** : K3s, Traefik (à venir)

## Architecture

Clean architecture (ports & adapters) côté backend :

```
backend/src/
├── domain/          # Entités, value objects, erreurs métier
├── applications/    # Use cases, ports (traits)
└── adapters/        # Implémentations concrètes (Twitch IRC, etc.)
```

## Lancer le projet

```bash
# Backend
cd backend
cargo watch -x run

# Frontend
cd frontend
pnpm dev
```

## Statut

En cours de développement — le backend lit le chat Twitch en temps réel via IRC over TLS.
