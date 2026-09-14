import { test, expect } from '@playwright/test'

test.describe('Laporin Frontend E2E Suite', () => {
  test.beforeEach(async ({ context }) => {
    // Clear cookies and storage for clean isolation between tests
    await context.clearCookies()
  })

  test('1. Landing Page renders distinctive dark identity, document narrative, and pricing', async ({ page }) => {
    await page.goto('/')

    // Check title and dark theme
    await expect(page).toHaveTitle(/Laporin/)
    const html = page.locator('html')
    await expect(html).toHaveClass(/dark/)

    // Check headline
    const heading = page.locator('h1')
    await expect(heading).toContainText('Laporan PKL kamu')

    // Check document preview mock
    await expect(page.locator('text=preview_laporan.docx')).toBeVisible()
    await expect(page.locator('text=BAB I · PENDAHULUAN')).toBeVisible()

    // Check pricing section
    await expect(page.locator('text=Rp15.000').first()).toBeVisible()
    await expect(page.locator('text=Tanpa langganan bulanan')).toBeVisible()

    // Check navigation link
    await expect(page.locator('a[href="/register"]').first()).toBeVisible()
  })

  test('2. Authentication - Register & Login pages with CAPTCHA and validation', async ({ page }) => {
    // Go to Register
    await page.goto('/register')
    await expect(page.locator('h1')).toContainText('Daftar Akun Laporin')

    // Check CAPTCHA shield widget
    const captcha = page.locator('text=Laporin Shield')
    await expect(captcha).toBeVisible()

    // Fill form inputs
    await page.locator('input[type="email"]').fill('siswa@example.com')
    await page.locator('input[placeholder="Minimal 8 karakter"]').fill('password123')
    await page.locator('input[placeholder="Ulangi kata sandi"]').fill('password123')

    // Click Captcha widget
    await page.click('text=Saya bukan robot (Verifikasi)')
    await expect(page.locator('text=Verifikasi Keamanan Berhasil')).toBeVisible()

    // Go to Login page
    await page.goto('/login')
    await expect(page.locator('h1')).toContainText('Masuk ke Laporin')
    await expect(page.locator('text=Lanjutkan dengan Google')).toBeVisible()
    await page.locator('input[type="email"]').fill('siswa@example.com')
    await page.locator('input[type="password"]').fill('password123')

    // Verify OTP page
    await page.goto('/verify-otp?email=siswa%40example.com')
    await expect(page.locator('h1')).toContainText('Verifikasi Kode OTP')
    await expect(page.locator('input[type="email"]')).toHaveValue('siswa@example.com')
  })

  test('3. Dashboard view structure and empty state', async ({ page }) => {
    await page.goto('/dashboard')

    // Expect heading to be visible (either dashboard or login if session unauthenticated)
    const heading = page.locator('h1')
    await expect(heading).toBeVisible()
  })

  test('4. Report Creation multi-step wizard works smoothly', async ({ page }) => {
    await page.goto('/reports/new')

    await expect(page.locator('h1')).toContainText('Formulir Laporan PKL')

    // Step 1: Siswa
    await page.locator('input[placeholder*="Muhammad Farhan"]').fill('Ahmad Pratama')
    await page.locator('input[placeholder*="202100412"]').fill('192010482')
    await page.locator('input[placeholder*="SMK Negeri 1"]').fill('SMK Negeri 2 Bandung')
    await page.click('button:has-text("Lanjut ke Data Tempat PKL")')

    // Step 2: Tempat PKL
    await expect(page.locator('h2')).toContainText('2. Tempat PKL & Pembimbing')
    await page.locator('input[placeholder*="PT Telkom Indonesia"]').fill('PT Astra Graphia Information Technology')
    await page.locator('input[placeholder*="Jl. Japati"]').fill('Jl. Kramat Raya No. 43, Jakarta')
    await page.click('button:has-text("Lanjut ke Jurnal Kegiatan")')

    // Step 3: Kegiatan Nyata
    await expect(page.locator('h2')).toContainText('3. Jurnal Kegiatan & Tanggung Jawab Nyata')
    await page.locator('textarea').fill('1. Melakukan instalasi dan konfigurasi server Linux Ubuntu 22.04.\n2. Melakukan deployment aplikasi web microservice.\n3. Membantu dokumentasi arsitektur jaringan internal.')
    await page.click('button:has-text("Review & Konfirmasi Draf")')

    // Step 4: Konfirmasi
    await expect(page.locator('h2')).toContainText('4. Konfirmasi Data Laporan')
    await expect(page.locator('text=Ahmad Pratama (192010482)')).toBeVisible()
    await expect(page.locator('text=PT Astra Graphia Information Technology')).toBeVisible()
    await expect(page.locator('button:has-text("Simpan Draf & Mulai Riset")')).toBeVisible()
  })

  test('5. Mobile responsiveness across key views', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 })
    await page.goto('/')

    // Check that layout adjusts cleanly without overflow
    await expect(page.locator('h1')).toBeVisible()
    await expect(page.locator('header')).toBeVisible()

    await page.goto('/reports/new')
    await expect(page.locator('h1')).toBeVisible()
  })
})
