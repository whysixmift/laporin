<script setup lang="ts">
import type { AiPlaygroundResponse } from '~/types/api'

definePageMeta({
  middleware: 'admin'
})

const { runAiPlayground } = useAdmin()
const toast = useToast()

const activeTab = ref<'llm' | 'crawl'>('llm')

// LLM State
const systemPrompt = ref('Kamu adalah asisten AI profesional untuk penyusunan laporan PKL & Magang akademik.')
const userPrompt = ref('Buatkan pengantar Bab I (Latar Belakang) untuk siswa SMK jurusan Rekayasa Perangkat Lunak yang magang di PT Teknologi Nusantara Abadi sebagai Web Developer.')
const isLlmRunning = ref(false)
const llmResult = ref<AiPlaygroundResponse | null>(null)

// Crawler State
const crawlUrl = ref('https://indonesia.go.id')
const isCrawlRunning = ref(false)
const crawlResult = ref<AiPlaygroundResponse | null>(null)

const handleRunLlm = async () => {
  if (!userPrompt.value.trim()) {
    toast.error('Prompt Kosong', 'Silakan masukkan prompt pengujian.')
    return
  }

  isLlmRunning.value = true
  llmResult.value = null
  try {
    const res = await runAiPlayground({
      type: 'llm',
      prompt: userPrompt.value.trim(),
      system_prompt: systemPrompt.value.trim()
    })
    llmResult.value = res
    if (res.success) {
      toast.success('Generasi Selesai', `Selesai dalam ${res.duration_ms} ms.`)
    } else {
      toast.error('Gagal Generasi', res.error || 'Terjadi kesalahan pada 9Router LLM.')
    }
  } catch (err: any) {
    toast.error('Koneksi Gagal', err.message || 'Tidak dapat terhubung ke AI server.')
  } finally {
    isLlmRunning.value = false
  }
}

const handleRunCrawl = async () => {
  if (!crawlUrl.value.trim()) {
    toast.error('URL Kosong', 'Silakan masukkan URL website.')
    return
  }

  isCrawlRunning.value = true
  crawlResult.value = null
  try {
    const res = await runAiPlayground({
      type: 'crawl',
      url: crawlUrl.value.trim()
    })
    crawlResult.value = res
    if (res.success) {
      toast.success('Crawling Selesai', `Selesai dalam ${res.duration_ms} ms.`)
    } else {
      toast.error('Gagal Crawl', res.error || 'Terjadi kendala saat merayapi website.')
    }
  } catch (err: any) {
    toast.error('Koneksi Gagal', err.message || 'Tidak dapat terhubung ke scraper.')
  } finally {
    isCrawlRunning.value = false
  }
}
</script>

<template>
  <div class="min-h-screen bg-canvas pb-16">
    <AdminNav />

    <div class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="mb-6">
        <h2 class="text-xl font-bold text-ink-primary">AI & Crawler Playground</h2>
        <p class="text-xs sm:text-sm text-ink-secondary mt-0.5">
          Uji coba langsung pipeline inferensi 9Router LLM dan crawler riset profil perusahaan tanpa mempengaruhi data produksi.
        </p>
      </div>

      <!-- Mode Selector Tabs -->
      <div class="flex items-center gap-2 mb-6 border-b border-border pb-3">
        <button
          type="button"
          :class="[
            'px-4 py-2 text-xs sm:text-sm font-semibold rounded-lg transition-all inline-flex items-center gap-2',
            activeTab === 'llm'
              ? 'bg-accent-600 text-white shadow-subtle'
              : 'bg-surface hover:bg-surface-elevated text-ink-secondary border border-border'
          ]"
          @click="activeTab = 'llm'"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
          <span>9Router LLM Generation</span>
        </button>

        <button
          type="button"
          :class="[
            'px-4 py-2 text-xs sm:text-sm font-semibold rounded-lg transition-all inline-flex items-center gap-2',
            activeTab === 'crawl'
              ? 'bg-accent-600 text-white shadow-subtle'
              : 'bg-surface hover:bg-surface-elevated text-ink-secondary border border-border'
          ]"
          @click="activeTab = 'crawl'"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
          </svg>
          <span>Web Crawler & Scraper</span>
        </button>
      </div>

      <!-- TAB 1: LLM TEST -->
      <div v-if="activeTab === 'llm'" class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Input Form -->
        <div class="bg-surface border border-border rounded-xl p-5 space-y-4">
          <h3 class="text-sm font-bold text-ink-primary uppercase font-mono tracking-wider">
            Parameter Permintaan LLM
          </h3>

          <div>
            <label class="block text-xs font-medium text-ink-secondary mb-1">System Prompt</label>
            <textarea
              v-model="systemPrompt"
              rows="3"
              class="w-full p-3 text-xs bg-surface-elevated border border-border rounded-lg text-ink-primary font-mono focus:outline-none focus:border-accent-500"
              placeholder="Instruksi sistem untuk model AI..."
            />
          </div>

          <div>
            <label class="block text-xs font-medium text-ink-secondary mb-1">User Prompt</label>
            <textarea
              v-model="userPrompt"
              rows="6"
              class="w-full p-3 text-xs bg-surface-elevated border border-border rounded-lg text-ink-primary focus:outline-none focus:border-accent-500 leading-relaxed"
              placeholder="Masukkan instruksi atau draft konten laporan..."
            />
          </div>

          <button
            type="button"
            :disabled="isLlmRunning"
            class="w-full py-2.5 px-4 rounded-lg bg-accent-600 hover:bg-accent-500 disabled:opacity-50 text-white text-xs font-semibold transition-colors inline-flex items-center justify-center gap-2 shadow-subtle"
            @click="handleRunLlm"
          >
            <svg
              v-if="isLlmRunning"
              class="w-4 h-4 animate-spin"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            <span>{{ isLlmRunning ? 'Memproses Generasi AI...' : 'Jalankan Pengujian AI' }}</span>
          </button>
        </div>

        <!-- Output Result -->
        <div class="bg-surface border border-border rounded-xl p-5 flex flex-col">
          <div class="flex items-center justify-between pb-3 border-b border-border mb-4">
            <h3 class="text-sm font-bold text-ink-primary uppercase font-mono tracking-wider">
              Hasil Respon
            </h3>
            <span v-if="llmResult" class="text-xs font-mono px-2 py-0.5 rounded bg-surface-elevated border border-border text-accent-400">
              Latency: {{ llmResult.duration_ms }} ms
            </span>
          </div>

          <div v-if="isLlmRunning" class="flex-1 flex flex-col items-center justify-center p-12 text-center text-ink-muted">
            <div class="inline-block animate-spin w-8 h-8 rounded-full border-2 border-accent-500 border-t-transparent mb-3" />
            <p class="text-xs font-mono">Menunggu inferensi 9Router LLM...</p>
          </div>

          <div v-else-if="llmResult" class="flex-1 space-y-3">
            <div v-if="!llmResult.success" class="p-3 bg-rose-500/10 border border-rose-500/30 rounded-lg text-rose-400 text-xs">
              <strong>Error:</strong> {{ llmResult.error }}
            </div>
            <div v-else class="p-4 bg-surface-elevated border border-border rounded-lg text-xs text-ink-primary whitespace-pre-wrap font-sans leading-relaxed max-h-[420px] overflow-y-auto select-text">
              {{ llmResult.output.response }}
            </div>
          </div>

          <div v-else class="flex-1 flex flex-col items-center justify-center p-12 text-center text-ink-muted">
            <svg class="w-10 h-10 opacity-30 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
            <p class="text-xs">Klik "Jalankan Pengujian AI" untuk melihat hasil respon model.</p>
          </div>
        </div>
      </div>

      <!-- TAB 2: CRAWLER TEST -->
      <div v-else class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Input Form -->
        <div class="bg-surface border border-border rounded-xl p-5 space-y-4">
          <h3 class="text-sm font-bold text-ink-primary uppercase font-mono tracking-wider">
            Target URL Crawling
          </h3>

          <div>
            <label class="block text-xs font-medium text-ink-secondary mb-1">Alamat Website Perusahaan / Instansi</label>
            <input
              v-model="crawlUrl"
              type="url"
              class="w-full p-3 text-xs bg-surface-elevated border border-border rounded-lg text-ink-primary font-mono focus:outline-none focus:border-accent-500"
              placeholder="https://contoh-perusahaan.co.id"
            />
            <p class="text-[11px] text-ink-muted mt-1">
              Crawler akan mengambil konten HTML, memfilter teks bersih, dan mengukur waktu tanggap.
            </p>
          </div>

          <button
            type="button"
            :disabled="isCrawlRunning"
            class="w-full py-2.5 px-4 rounded-lg bg-accent-600 hover:bg-accent-500 disabled:opacity-50 text-white text-xs font-semibold transition-colors inline-flex items-center justify-center gap-2 shadow-subtle"
            @click="handleRunCrawl"
          >
            <svg
              v-if="isCrawlRunning"
              class="w-4 h-4 animate-spin"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            <span>{{ isCrawlRunning ? 'Mengambil Data Halaman...' : 'Uji Crawl URL' }}</span>
          </button>
        </div>

        <!-- Output Result -->
        <div class="bg-surface border border-border rounded-xl p-5 flex flex-col">
          <div class="flex items-center justify-between pb-3 border-b border-border mb-4">
            <h3 class="text-sm font-bold text-ink-primary uppercase font-mono tracking-wider">
              Pratinjau Hasil Crawling
            </h3>
            <span v-if="crawlResult" class="text-xs font-mono px-2 py-0.5 rounded bg-surface-elevated border border-border text-accent-400">
              {{ crawlResult.duration_ms }} ms
            </span>
          </div>

          <div v-if="isCrawlRunning" class="flex-1 flex flex-col items-center justify-center p-12 text-center text-ink-muted">
            <div class="inline-block animate-spin w-8 h-8 rounded-full border-2 border-accent-500 border-t-transparent mb-3" />
            <p class="text-xs font-mono">Menghubungi server target & mengekstrak konten...</p>
          </div>

          <div v-else-if="crawlResult" class="flex-1 space-y-3">
            <div v-if="!crawlResult.success" class="p-3 bg-rose-500/10 border border-rose-500/30 rounded-lg text-rose-400 text-xs">
              <strong>Error:</strong> {{ crawlResult.error }}
            </div>
            <div v-else class="space-y-2">
              <div class="text-[11px] font-mono text-ink-muted">
                Karakter Ditemukan: <strong class="text-ink-primary">{{ crawlResult.output.content_length }} bytes</strong>
              </div>
              <pre class="p-3 bg-surface-elevated border border-border rounded-lg text-[11px] text-ink-primary whitespace-pre-wrap font-mono max-h-[380px] overflow-y-auto select-text">{{ crawlResult.output.preview }}</pre>
            </div>
          </div>

          <div v-else class="flex-1 flex flex-col items-center justify-center p-12 text-center text-ink-muted">
            <svg class="w-10 h-10 opacity-30 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
            </svg>
            <p class="text-xs">Masukkan URL dan klik "Uji Crawl URL".</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
