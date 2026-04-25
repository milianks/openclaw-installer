/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        claw: {
          50: '#fbf1eb',
          100: '#f6e1d2',
          200: '#edc2a5',
          300: '#e39b76',
          400: '#d7794f',
          500: '#c95c34',
          600: '#af4726',
          700: '#8e381f',
          800: '#742f1d',
          900: '#5f281b',
          950: '#34120b',
        },
        surface: {
          app: 'var(--bg-app)',
          sidebar: 'var(--bg-sidebar)',
          card: 'var(--bg-card)',
          'card-hover': 'var(--bg-card-hover)',
          input: 'var(--bg-input)',
          elevated: 'var(--bg-elevated)',
          overlay: 'var(--bg-overlay)',
          code: 'var(--bg-code)',
        },
        content: {
          primary: 'var(--text-primary)',
          secondary: 'var(--text-secondary)',
          tertiary: 'var(--text-tertiary)',
          inverse: 'var(--text-inverse)',
        },
        edge: {
          DEFAULT: 'var(--border-primary)',
          secondary: 'var(--border-secondary)',
        },
        dark: {
          900: '#13110f',
          800: '#1a1613',
          700: '#231e1a',
          600: '#312922',
          500: '#43372d',
          400: '#5a493c',
        },
        accent: {
          cyan: '#2e8f83',
          purple: '#6f7dd9',
          green: '#739a52',
          amber: '#d19539',
        }
      },
      fontFamily: {
        sans: [
          'Avenir Next',
          'Segoe UI Variable Display',
          'SF Pro Display',
          'PingFang SC',
          'Hiragino Sans GB',
          'Microsoft YaHei',
          'sans-serif',
        ],
        mono: [
          'JetBrains Mono',
          'SF Mono',
          'Fira Code',
          'Menlo',
          'monospace',
        ],
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'glow': 'glow 2s ease-in-out infinite alternate',
        'slide-up': 'slideUp 0.3s ease-out',
        'slide-down': 'slideDown 0.3s ease-out',
        'fade-in': 'fadeIn 0.2s ease-out',
      },
      keyframes: {
        glow: {
          '0%': { boxShadow: '0 0 5px rgba(201, 92, 52, 0.32)' },
          '100%': { boxShadow: '0 0 20px rgba(201, 92, 52, 0.58)' },
        },
        slideUp: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        slideDown: {
          '0%': { transform: 'translateY(-10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
      },
      boxShadow: {
        'glow-claw': '0 0 30px rgba(201, 92, 52, 0.28)',
        'glow-cyan': '0 0 30px rgba(46, 143, 131, 0.24)',
        'glow-green': '0 0 30px rgba(115, 154, 82, 0.24)',
        'inner-light': 'inset 0 1px 0 0 rgba(255, 255, 255, 0.05)',
        'card': 'var(--shadow-card)',
      },
      backdropBlur: {
        xs: '2px',
      },
    },
  },
  plugins: [],
}
