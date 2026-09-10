# ADR-003: Research Fact Provenance and Source Tracking

## Status
Accepted

## Context
AI-assisted report generation must avoid hallucinations and comply with academic integrity expectations by grounding claims in verifiable sources.

## Decision
- Every extracted fact in `report_research_facts` must reference a valid row in `research_sources` (`source_id`).
- Up to 5 external sources are crawled per report, with max 3 hops depth and 2 MiB per page limit.
- All research facts returned by the API include source metadata (`url`, `title`, `fetched_at`, `confidence_score`).

## Consequences
- **Positive**: Complete audit trail for all AI-researched statements; verifiable provenance.
- **Negative**: Adds database relational structure and crawler validation constraints.
