---
name: postgresql
description: PostgreSQL database implementation, migrations, indexing, relational constraints, and FOR UPDATE SKIP LOCKED job queue patterns.
---

# PostgreSQL Database & Queue Skill

## 1. Scope & Ownership
OpenCode owns all database schemas, migrations, query optimization, and connection pooling. The single source of truth is `docs/database-schema.md`.

## 2. Rules & Constraints
1. **Strict Schema Fidelity**: All table definitions, column types, CHECK constraints, foreign keys, and indexes must match `docs/database-schema.md` exactly.
2. **Versioned Migrations**: Use SQLx migrations (`migrations/YYYYMMDDHHMMSS_name.sql`). Never apply unversioned schema mutations.
3. **Status Enums**: Enforce the canonical 12-state report enum in SQL CHECK constraints:
   ```sql
   CHECK (status IN ('draft','researching','research_completed','generating','generated','preview_ready','payment_pending','paid','unlocked','failed','cancelled','expired'))
   ```
4. **Ownership Isolation**: Queries fetching user resources MUST include `WHERE user_id = $1` or join through parent tables to prevent horizontal privilege escalation.

## 3. PostgreSQL-Backed Job Queue Pattern
Workers poll jobs using non-blocking transactional row locks:
```sql
BEGIN;
SELECT id, report_id, attempts 
FROM research_jobs 
WHERE status = 'pending' 
ORDER BY created_at ASC 
LIMIT 1 
FOR UPDATE SKIP LOCKED;

UPDATE research_jobs 
SET status = 'running', started_at = NOW(), attempts = attempts + 1 
WHERE id = $1;
COMMIT;
```

## 4. Connection Pool Hygiene
- Bound the SQLx connection pool (`max_connections = 10-15`).
- Ensure transactions are kept short to avoid table lock contention.
