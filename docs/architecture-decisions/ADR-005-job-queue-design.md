# ADR-005: PostgreSQL-Backed In-Process Job Queue

## Status
Accepted

## Context
Redis, RabbitMQ, and Celery consume substantial RAM and add deployment complexity on a 2 GB RAM VPS.

## Decision
- Implement the job queue directly in PostgreSQL using `SELECT ... FOR UPDATE SKIP LOCKED` inside Axum background worker tasks.
- Enforce strict concurrency limits (2 research workers, 1 generation worker).
- Monitor memory pressure (< 500 MiB free pauses worker intake).

## Consequences
- **Positive**: Zero additional server daemons; ACID transactions ensure jobs cannot be lost; low memory overhead.
- **Negative**: Polling interval adds small database query load (negligible at 2s interval).
