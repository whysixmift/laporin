---
name: resource-awareness
description: Hardware budget enforcement and resource safety guidelines for 2 GB RAM / 16 GB SSD VPS.
---

# Resource Awareness Skill

## 1. Physical Target Environment
- **CPU**: 1-2 vCPU
- **RAM**: 2 GB Total (PostgreSQL + Axum + LibreOffice + Caddy + OS)
- **Disk**: 16 GB SSD Total (OS, Postgres data, storage directories, logs)

## 2. Strict Architectural Boundaries
- 🚫 **No Heavy External Services**: Never introduce Redis, Memcached, RabbitMQ, Kafka, Elasticsearch, or Kubernetes. All caching and queueing must use in-process Rust data structures or PostgreSQL tables.
- 🚫 **No In-Memory File Buffering for Large Blobs**: Stream generated documents and previews directly using chunked I/O.
- 🚫 **No Unbounded Concurrency**:
  - Research worker threads: Maximum **2 concurrent jobs**.
  - LibreOffice PDF rendering: Maximum **1 concurrent job** (spikes ~300 MB RSS).

## 3. Dynamic Guards & Thresholds
| Metric | Threshold | Action |
|---|---|---|
| Available RAM | `< 500 MiB` | Workers pause job claiming from PostgreSQL queue. |
| Available Disk | `< 2 GB` | Structured warning logged; trigger temporary file sweep. |
| Available Disk | `< 1 GB` | Reject new report creation (`HTTP 503 INSUFFICIENT_STORAGE`); pause active jobs. |
| Temporary Files | `> 30 minutes old` | Background cleaner deletes files in `storage/tmp/`. |
| Unlocked DOCX | `> 30 days old` | Background retention cleaner deletes files and marks report `expired`. |

## 4. Coding Checklist for Resource Safety
- [ ] Are all database connections pooled and bounded (e.g. `max_connections = 10-15` in SQLx)?
- [ ] Does file streaming avoid loading the entire file into a `Vec<u8>` in RAM?
- [ ] Are subprocesses (e.g. LibreOffice) executed with timeouts and killed if they hang?
- [ ] Are log files configured with max size (10 MB) and rotation?
