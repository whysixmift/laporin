# ADR-008: 9Router Abstraction and Configurable LLM Routing

## Status
Accepted

## Context
Specific LLM models change rapidly in pricing, availability, and capability. Hard-coding specific model strings (e.g. `gpt-4-turbo` or specific versions) into backend application logic creates tight coupling and vendor lock-in.

## Decision
- Route all LLM requests through **9Router** deployed locally/as a reverse-proxy routing layer.
- Model names, API keys, fallback routes, and temperature parameters are configured purely through environment variables (`LLM_ROUTER_URL`, `LLM_DEFAULT_MODEL`, `LLM_FALLBACK_MODEL`).
- Rust backend uses a provider-agnostic client trait (`LlmClient`) sending standardized prompt payloads.

## Consequences
- **Positive**: Zero backend code changes needed to swap models or providers; resilient routing.
- **Negative**: Adds an environment configuration dependency.
