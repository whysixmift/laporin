---
name: ai-research
description: AI research crawler and extraction pipeline using 9Router, fact classification (USER_FACT, RESEARCH_FACT, AI_DERIVED_TEXT), provenance linking, and SSRF defenses.
---

# AI Research & Provenance Skill

## 1. Scope & Ownership
OpenCode owns the research worker and external web crawler. The specification is defined in `docs/research-flow.md` and `docs/architecture-decisions/ADR-003-research-provenance.md`.

## 2. Fact Classification Invariants
- **`USER_FACT`**: Student info and internship details supplied directly by the user. Must never be altered or overwritten by AI research.
- **`RESEARCH_FACT`**: Structured claims discovered by web crawling. MUST reference an entry in `research_sources` (`source_id`).
- **`AI_DERIVED_TEXT`**: Free-form prose synthesized by LLM in generation step. Must strictly ground all statements in existing `USER_FACT` and `RESEARCH_FACT` entries.

## 3. Crawler Security & Safety
1. **SSRF Defense**:
   - Scheme MUST be `https`.
   - Resolve DNS and reject all private/loopback/link-local IP addresses (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16, 127.0.0.0/8, ::1, 169.254.0.0/16).
2. **Quota Bounds**:
   - Max **5 sources** per project.
   - Max **3 hops** link depth.
   - Max **2 MiB** response payload per page.
3. **HTML Sanitization**:
   - Strip `<script>`, `<iframe>`, `<style>`, and binary embeds before passing text to extraction prompts.
4. **Prompt Injection Resistance**:
   - Wrap untrusted page content in structured delimiters and JSON escape all input strings.

## 4. 9Router Integration
- Do NOT hard-code LLM model names. Route all requests through the configurable 9Router client (`LLM_ROUTER_URL`, `LLM_DEFAULT_MODEL`).
