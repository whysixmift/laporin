# Laporin Frontend 🖥️⚡

Nuxt 3 & Vue 3 frontend for **Laporin**, an automated internship report generator (*Laporan PKL / Magang*) engineered for Indonesian students.

---

## 🎨 Visual Identity & Design Principles

- **Dark-First Architectural Palette**: Built on deep charcoal tones (`#0c0e12`, `#13171f`, `#1e2430`) with crisp borders (`#2d3747`), warm off-white reading text (`#f1f4f8`), and emerald status accents (`#10b981`).
- **Academic Document Preview**: Simulates a physical A4 sheet with standard Indonesian PKL formatting:
  - Serif headings (*Newsreader*)
  - Cover page, Lembar Pengesahan, Kata Pengantar, Bab I (Pendahuluan), Bab II (Profil Perusahaan), Bab III (Pelaksanaan Praktik), Bab IV (Penutup).
  - Watermarked overlay before unlock.
- **Anti-AI-Slop Standard**: No generic gradient soup, floating glowing blobs, artificial AI sparkles, or fake statistics. Information hierarchy is practical and academic.

---

## 🚀 Quick Start

### 1. Install Dependencies
```bash
pnpm install
```

### 2. Development Server
```bash
pnpm run dev
```
Starts dev server at `http://localhost:3000` proxying `/api/v1/**` to `http://127.0.0.1:8080/api/v1/**`.

### 3. Production Build
```bash
pnpm run build
node .output/server/index.mjs
```

---

## 🧪 Testing with Playwright

Run the automated test suite across Desktop & Mobile viewports:
```bash
pnpm exec playwright test
```
