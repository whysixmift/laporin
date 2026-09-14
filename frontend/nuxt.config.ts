export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  devtools: { enabled: false },

  modules: [
    '@nuxtjs/tailwindcss'
  ],

  components: [
    {
      path: '~/components',
      pathPrefix: false
    }
  ],

  app: {
    head: {
      title: 'Laporin — Laporan PKL, Otomatis Jadi & Rapi',
      htmlAttrs: {
        lang: 'id',
        class: 'dark'
      },
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { name: 'description', content: 'Ubah informasi dan kegiatan magang/PKL menjadi laporan PKL akademik yang rapi, terstruktur, dan siap cetak.' },
        { name: 'theme-color', content: '#0c0e12' }
      ],
      link: [
        { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
        { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' },
        {
          rel: 'stylesheet',
          href: 'https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500&family=Newsreader:ital,opsz,wght@0,6..72,400..700;1,6..72,400..700&family=Plus+Jakarta+Sans:wght@400;500;600;700&display=swap'
        }
      ]
    }
  },

  runtimeConfig: {
    public: {
      apiBase: process.env.NUXT_PUBLIC_API_BASE || '/api/v1',
      hcaptchaSiteKey: process.env.NUXT_PUBLIC_HCAPTCHA_SITEKEY || '10000000-ffff-ffff-ffff-000000000001'
    }
  },

  nitro: {
    routeRules: {
      '/api/v1/**': {
        proxy: 'http://127.0.0.1:8080/api/v1/**'
      }
    },
    devProxy: {
      '/api/v1': {
        target: 'http://127.0.0.1:8080/api/v1',
        changeOrigin: true
      }
    }
  }
})
