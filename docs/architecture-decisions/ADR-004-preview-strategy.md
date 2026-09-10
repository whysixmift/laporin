# ADR-004: Direct Backend PDF Preview Delivery Strategy

## Status
Accepted

## Context
Deploying an S3/MinIO bucket on a 2 GB VPS or paying for cloud object storage introduces unnecessary cost and operational complexity for an MVP.

## Decision
- Previews are rendered to PDF using headless LibreOffice and stored in the local filesystem.
- Previews are served via `GET /reports/{id}/preview` through an authorized Axum endpoint that streams the PDF.
- Internal filesystem paths are never returned to the client.

## Consequences
- **Positive**: Zero extra infrastructure cost; simple authorization model; no presigned URL expiration bugs.
- **Negative**: Axum handles file streaming I/O; bounded by local disk space.
