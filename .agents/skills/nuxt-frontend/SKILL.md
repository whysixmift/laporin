---
name: nuxt-frontend
description: Nuxt 3 & Vue 3 frontend architecture, Composition API, state management, composables, SSR/client boundaries, and component structure.
---

# Nuxt 3 Frontend Architecture Skill

## 1. Scope & Ownership
Claude Code owns the entire Nuxt 3 application, including Vue components, composables, Pinia/useState stores, Tailwind CSS styling, and client-side routing.

## 2. Directory Structure Conventions
```text
frontend/
├── app.vue
├── nuxt.config.ts
├── pages/                    # File-based routing (e.g. index, login, reports/[id])
├── components/               # UI components (atoms, molecules, organisms)
│   ├── auth/
│   ├── report/
│   ├── preview/
│   └── common/
├── composables/              # Reusable stateful logic (useAuth, useReport, useApi)
├── types/                    # TypeScript interfaces mirroring OpenAPI models
└── assets/css/               # Tailwind / typography stylesheets
```

## 3. Idiomatic Vue 3 / Nuxt Guidelines
1. **Composition API & `<script setup lang="ts">`**: Always use `<script setup lang="ts">` with TypeScript for type safety.
2. **Component Boundaries**: Keep components focused. Decompose large views into reusable sub-components (e.g., `ReportFactList.vue`, `PreviewViewer.vue`).
3. **Composables for Business Logic**: Encapsulate API polling, auth cookies, and state transitions inside composables (`useReportLifecycle.ts`).
4. **SSR / Client Boundaries**: Use `<ClientOnly>` for DOM-heavy widgets like PDF.js viewers or canvas renderers.
5. **No AI Spaghetti Code**: Maintain clean, readable markup, avoid deeply nested template conditionals, and use semantic HTML elements.
