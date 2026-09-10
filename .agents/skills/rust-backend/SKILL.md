---
name: rust-backend
description: Rust and Axum modular monolith development conventions, application state, error handling, tracing, and shutdown.
---

# Rust Backend Engineering Skill

## 1. Scope & Ownership
OpenCode owns the Rust backend application. The architecture is a modular monolith written in Rust using Axum, SQLx, and Tokio.

## 2. Module Boundaries
```text
src/
├── main.rs                   # Entry point, CLI flags, graceful shutdown
├── config.rs                 # Environment variables configuration
├── state.rs                  # AppState (DB pool, HTTP client, config)
├── error.rs                  # Standard AppError -> IntoResponse mapping
├── middleware/               # Auth, rate limiting, logging, CSRF/CORS
├── modules/
│   ├── auth/                 # Registration, login, Argon2id, OTP, Google OAuth
│   ├── report/               # Report CRUD, validation, domain models
│   ├── research/             # 9Router client, web crawler, fact extractor
│   ├── generation/           # DOCX placeholder substitution, LibreOffice PDF preview
│   ├── payment/              # Mayar integration, webhook verification, unlock
│   └── storage/              # Path resolution, canonicalization, retention cleaner
└── workers/                  # Postgres-backed async queue workers
```

## 3. Idiomatic Rules
1. **Explicit, Simple Rust**: Favor clear, explicit code over overly complex trait macros or deeply nested generics.
2. **Centralized Error Handling**: Use an `AppError` enum implementing `IntoResponse` that produces consistent JSON errors matching `components.schemas.ErrorResponse` from `docs/openapi.yaml`.
3. **Structured Tracing**: Use `tracing::info!`, `tracing::warn!`, `tracing::error!` with structured fields (`user_id`, `report_id`, `job_id`).
4. **Graceful Shutdown**: Listen for `SIGINT` / `SIGTERM` in Tokio to allow running jobs to finish or cleanly return to `pending`.
5. **No `unwrap()` in Request Handlers**: Always handle errors using `?` or explicit matching.
