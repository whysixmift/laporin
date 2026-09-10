# ADR-006: Local Filesystem Storage Layout & Security

## Status
Accepted

## Context
Files must be stored safely with strict path-traversal prevention, deterministic naming, and disk-quota protections.

## Decision
- Use dedicated subdirectories: `storage/templates`, `storage/reports/{id}/docx`, `storage/reports/{id}/preview`, `storage/tmp`.
- Name all dynamic files with UUID v4 strings.
- Enforce path validation (`canonicalize` and prefix match against root).
- Enforce automatic retention (30 min for temp files, 30 days for DOCX).

## Consequences
- **Positive**: Strict isolation, immune to path traversal, predictable disk consumption.
- **Negative**: Requires disk cleanup tasks and monitoring.
