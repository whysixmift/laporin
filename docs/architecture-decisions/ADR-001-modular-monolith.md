# ADR-001: Modular Monolith Architecture

## Status
Accepted

## Context
Laporin is deployed on a single VPS with 2 GB RAM and 16 GB storage. The team consists of two primary developers/agents (OpenCode on backend, Claude Code on frontend). Running distributed microservices would exceed RAM limits and introduce unnecessary network serialization and operational overhead.

## Decision
Adopt a **modular monolith** in Rust (Axum) with clear domain boundaries (Auth, Report, Research, Generation, Payment, Storage). All domains share a single PostgreSQL database instance and communicate via strongly-typed in-process Rust modules.

## Consequences
- **Positive**: Low memory footprint (~30-50 MB base RSS), zero network overhead between modules, simplified transactional integrity.
- **Negative**: Requires discipline to prevent tight coupling across domain modules.
