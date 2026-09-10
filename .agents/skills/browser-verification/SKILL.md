---
name: browser-verification
description: End-to-end browser verification skill using Playwright / headless browser to test rendered UI and critical user journeys.
---

# Browser Verification Skill

## 1. Goal
Verify that the rendered Nuxt frontend correctly interacts with the user, connects to the Axum backend API, handles async states, and renders documents without visual degradation or console errors.

## 2. Verification Capabilities & Protocol
1. **Start Dev Environment**: Ensure Nuxt dev server (`npm run dev`) and Axum backend (`cargo run`) are active.
2. **Launch Playwright / Browser Session**: Navigate to target routes (`http://localhost:3000`).
3. **Inspect Real DOM Elements**: Locate forms, buttons, inputs, and interactive components.
4. **Assert Reactive UX States**:
   - Verify spinners / progress bars during `researching` and `generating`.
   - Verify error toasts / banners on invalid form input or 4xx/5xx responses.
   - Verify that the PDF preview embeds correctly inside the viewer.
5. **Console & Network Inspection**:
   - Check browser console logs for unhandled exceptions or Vue warnings.
   - Inspect network requests to confirm proper headers (`Cookie: session_id=...`, `X-CSRF-Token`).

## 3. Mandatory Verification Flows
- [ ] **Auth Flow**: Registration, login with CAPTCHA token, OTP verification modal, logout.
- [ ] **Report Creation**: Fill student and internship details, submit draft report.
- [ ] **Research Flow**: Trigger research job, poll status, view extracted facts with source links.
- [ ] **Generation Flow**: Trigger generation job, poll status until `preview_ready`.
- [ ] **Preview Flow**: View rendered PDF preview in the browser iframe/PDF viewer.
- [ ] **Payment Flow**: Trigger checkout, simulate webhook receipt, verify report status transitions to `unlocked`.
- [ ] **Download Flow**: Click download button, verify successful `.docx` binary download.
