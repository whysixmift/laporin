import { test, expect } from '@playwright/test'

test.describe('Laporin Full Authenticated Report Lifecycle Flow', () => {
  test('Complete journey: Register -> Login -> Create Report -> Research -> Generate -> Preview & Paywall', async ({ page }) => {
    const timestamp = Date.now()
    const email = `siswa_${timestamp}@example.com`
    const password = 'password123'

    // 1. Register
    await page.goto('/register')
    await page.locator('input[type="email"]').fill(email)
    await page.locator('input[placeholder="Minimal 8 karakter"]').fill(password)
    await page.locator('input[placeholder="Ulangi kata sandi"]').fill(password)

    // Complete captcha
    await page.click('text=Saya bukan robot (Verifikasi)')
    await expect(page.locator('text=Verifikasi Keamanan Berhasil')).toBeVisible()

    // Submit registration
    await page.click('button:has-text("Daftar Akun")')

    // Expect redirect
    await page.waitForURL(url => url.pathname.includes('/verify-otp') || url.pathname.includes('/login'), { timeout: 10000 })

    // 2. Login
    await page.goto('/login')
    await page.locator('input[type="email"]').fill(email)
    await page.locator('input[type="password"]').fill(password)
    await page.click('text=Saya bukan robot (Verifikasi)')
    await expect(page.locator('text=Verifikasi Keamanan Berhasil')).toBeVisible()
    await page.click('button:has-text("Masuk Akun")')

    // Expect redirect to Dashboard
    await page.waitForURL(url => url.pathname === '/dashboard', { timeout: 10000 })
    await expect(page.locator('h1')).toContainText('Laporan PKL Saya')

    // 3. Create Report Wizard
    await page.click('text=Buat Laporan Baru')
    await page.waitForURL(url => url.pathname === '/reports/new', { timeout: 5000 })

    // Fill Step 1
    await page.locator('input[placeholder*="Muhammad Farhan"]').fill('Budi Wicaksono')
    await page.locator('input[placeholder*="202100412"]').fill('202109988')
    await page.locator('input[placeholder*="SMK Negeri 1"]').fill('SMK Negeri 1 Surabaya')
    await page.click('button:has-text("Lanjut ke Data Tempat PKL")')

    // Fill Step 2
    await page.locator('input[placeholder*="PT Telkom Indonesia"]').fill('PT Indo Pratama Digital')
    await page.locator('input[placeholder*="Jl. Japati"]').fill('Jl. Pemuda No. 88, Surabaya')
    await page.click('button:has-text("Lanjut ke Jurnal Kegiatan")')

    // Fill Step 3
    await page.locator('textarea').fill('1. Melakukan instalasi switch Cisco Catalyst.\n2. Melakukan routing VLAN internal divisi administrasi.\n3. Menyusun dokumentasi topologi jaringan.')
    await page.click('button:has-text("Review & Konfirmasi Draf")')

    // Step 4: Submit Draft
    await page.click('button:has-text("Simpan Draf & Mulai Riset")')

    // Expect redirect to Report Workspace
    await page.waitForURL(url => url.pathname.startsWith('/reports/'), { timeout: 10000 })

    // Check that Report Workspace loaded
    await expect(page.locator('h1')).toContainText('Laporan PKL')
    await expect(page.locator('text=PT Indo Pratama Digital').first()).toBeVisible()
    await expect(page.locator('text=Mulai Riset Profil Perusahaan')).toBeVisible()

    // Test Edit Modal
    await page.click('button:has-text("Edit Data Draf")')
    await expect(page.locator('text=Edit Data Laporan PKL')).toBeVisible()
    await page.locator('input[placeholder*="Hendra Wijaya"]').fill('Ir. Bambang Sugiarto')
    await page.getByRole('button', { name: 'Simpan Perubahan' }).click({ force: true })
    await expect(page.locator('text=Perubahan Disimpan')).toBeVisible()

    // 4. Trigger Research Job
    await page.click('button:has-text("Mulai Riset Profil Perusahaan")')

    // Wait for research completion
    const researchSuccess = page.getByRole('heading', { name: 'Riset Perusahaan Berhasil' })
    await expect(researchSuccess).toBeVisible({ timeout: 15000 })

    // Check extracted fact
    await expect(page.locator('text=Fakta Profil yang Ditemukan')).toBeVisible()

    // 5. Trigger Document Generation
    await page.click('button:has-text("Susun Laporan PKL Sekarang")')

    // Wait for Generation / Preview Ready transition
    const paywallTitle = page.locator('text=Buka & Unduh Dokumen Lengkap')
    await expect(paywallTitle).toBeVisible({ timeout: 20000 })

    // Check Paywall details
    await expect(page.locator('text=Rp15.000').first()).toBeVisible()
    await expect(page.locator('text=Format Microsoft Word (.docx)')).toBeVisible()

    // Check Academic Document Viewer headings
    await expect(page.getByRole('heading', { name: 'BAB I', exact: true })).toBeVisible()
    await expect(page.getByRole('heading', { name: 'PENDAHULUAN' })).toBeVisible()
    await expect(page.locator('text=Pratinjau Laporin').first()).toBeVisible()
  })
})
