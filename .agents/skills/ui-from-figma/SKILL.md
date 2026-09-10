---
name: ui-from-figma
description: Accurate UI implementation from Figma designs, preserving typography, spacing, component hierarchy, and responsive layouts without generic AI styling.
---

# UI Implementation from Figma Skill

## 1. Scope & Ownership
Claude Code is responsible for translating Figma designs, wireframes, and design tokens into production-ready Vue/Tailwind components.

## 2. Core Translation Principles
1. **Preserve Design Hierarchy**: Faithfully implement spacing tokens (margins, paddings), typography scales, font weights, and color palettes provided in Figma.
2. **Avoid Generic AI UI**: Do NOT substitute specified design tokens with arbitrary colors, excessive gradients, or generic button styles.
3. **Responsive Layouts**: Ensure all pages adapt smoothly across mobile (375px+), tablet (768px+), and desktop (1280px+) viewports.
4. **Accessible Forms**: Ensure form inputs have proper labels, `aria-*` attributes, focus indicators, and clear validation error states.
5. **Indonesian Localization Context**: Ensure typography and layouts accommodate Indonesian phrasing without text overflow or awkward line wraps.

## 3. UI Implementation Checklist
- [ ] Colors match Figma tokens exactly (primary, neutral, surface, error, success).
- [ ] Typography line-heights and font families match design specifications.
- [ ] Hover, active, focus, and disabled states are implemented for all buttons and interactive elements.
- [ ] Modals and drawers are keyboard-accessible (Escape to close, focus trap).
- [ ] No visual regressions during dynamic state transitions (e.g. loading skeletons).
