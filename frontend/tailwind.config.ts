import type { Config } from 'tailwindcss'

export default <Config>{
  darkMode: 'class',
  content: [
    './components/**/*.{vue,js,ts}',
    './layouts/**/*.vue',
    './pages/**/*.vue',
    './composables/**/*.{js,ts}',
    './plugins/**/*.{js,ts}',
    './app.vue',
  ],
  theme: {
    extend: {
      colors: {
        canvas: '#0c0e12',
        surface: {
          DEFAULT: '#13171f',
          subtle: '#181d26',
          elevated: '#1e2430',
          hover: '#252d3a',
          active: '#2c3545',
        },
        border: {
          faint: '#1c222c',
          subtle: '#242c39',
          DEFAULT: '#2d3747',
          strong: '#3f4c60',
        },
        ink: {
          primary: '#f1f4f8',
          secondary: '#9da8b9',
          muted: '#637083',
          faint: '#3e4757',
        },
        accent: {
          50: '#ecfdf5',
          100: '#d1fae5',
          200: '#a7f3d0',
          300: '#6ee7b7',
          400: '#34d399',
          500: '#10b981',
          600: '#059669',
          700: '#047857',
          800: '#065f46',
          900: '#064e3b',
          DEFAULT: '#10b981',
        },
        ochre: {
          400: '#f59e0b',
          500: '#d97706',
          600: '#b45309',
        },
        danger: {
          400: '#f87171',
          500: '#ef4444',
          600: '#dc2626',
        }
      },
      fontFamily: {
        sans: ['Plus Jakarta Sans', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'sans-serif'],
        serif: ['Newsreader', 'Lora', 'Georgia', 'serif'],
        mono: ['JetBrains Mono', 'SFMono-Regular', 'Menlo', 'Monaco', 'Consolas', 'monospace'],
      },
      boxShadow: {
        'subtle': '0 1px 2px 0 rgba(0, 0, 0, 0.35)',
        'elevated': '0 4px 16px -2px rgba(0, 0, 0, 0.45)',
        'doc': '0 8px 32px -4px rgba(0, 0, 0, 0.65), 0 2px 6px -1px rgba(0, 0, 0, 0.4)',
      },
    },
  },
  plugins: [],
}
