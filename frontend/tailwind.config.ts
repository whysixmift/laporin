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
        canvas: '#090a0c',
        surface: {
          DEFAULT: '#101215',
          subtle: '#14171b',
          elevated: '#1a1d23',
          hover: '#20242b',
          active: '#272c35',
        },
        border: {
          faint: '#181b20',
          subtle: '#20242b',
          DEFAULT: '#2a2f38',
          strong: '#3b424e',
        },
        ink: {
          primary: '#f4f3ef',
          secondary: '#a3a6ad',
          muted: '#686c75',
          faint: '#3d4047',
        },
        accent: {
          50: '#f0fdf4',
          100: '#dcfce7',
          200: '#bbf7d0',
          300: '#86efac',
          400: '#4ade80',
          500: '#22c55e',
          600: '#16a34a',
          700: '#15803d',
          800: '#166534',
          900: '#14532d',
          DEFAULT: '#16a34a',
        },
        ochre: {
          400: '#eab308',
          500: '#ca8a04',
          600: '#a16207',
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
        'subtle': '0 1px 2px 0 rgba(0, 0, 0, 0.4)',
        'elevated': '0 4px 20px -2px rgba(0, 0, 0, 0.5)',
        'doc': '0 12px 40px -4px rgba(0, 0, 0, 0.75)',
      },
    },
  },
  plugins: [],
}

