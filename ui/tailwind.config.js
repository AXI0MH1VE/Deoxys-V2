/** @type {import('tailwindcss').Config} */
// Visual Theme OS Configuration
// AxiomHive Sovereign Manifold v2.1.0
// Zero Entropy Law (C=0) - Canonical palette enforcement

module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
    "./ui/**/*.{html,css,js}",
    "./index.html"
  ],
  theme: {
    extend: {
      colors: {
        // Canonical AxiomHive palette
        'axiom-black': '#000000',
        'miami-red': '#FF0038',
        // Derived colors for UI consistency
        'axiom-gray': {
          50: '#0a0a0a',
          100: '#1a1a1a',
          200: '#2a2a2a',
          300: '#3a3a3a',
          400: '#4a4a4a',
          500: '#5a5a5a',
          600: '#6a6a6a',
          700: '#7a7a7a',
          800: '#8a8a8a',
          900: '#9a9a9a',
        },
        'miami-red-light': '#FF3366',
        'miami-red-dark': '#CC0029',
      },
      backgroundImage: {
        // Honeycomb Hex Grid pattern
        'honeycomb': `
          linear-gradient(30deg, transparent 25%, rgba(255, 0, 56, 0.05) 25%),
          linear-gradient(-30deg, transparent 25%, rgba(255, 0, 56, 0.05) 25%),
          linear-gradient(30deg, rgba(255, 0, 56, 0.05) 75%, transparent 75%),
          linear-gradient(-30deg, rgba(255, 0, 56, 0.05) 75%, transparent 75%)
        `,
        'honeycomb-hex': `
          repeating-linear-gradient(
            0deg,
            transparent,
            transparent 2px,
            rgba(255, 0, 56, 0.03) 2px,
            rgba(255, 0, 56, 0.03) 4px
          ),
          repeating-linear-gradient(
            60deg,
            transparent,
            transparent 2px,
            rgba(255, 0, 56, 0.03) 2px,
            rgba(255, 0, 56, 0.03) 4px
          ),
          repeating-linear-gradient(
            120deg,
            transparent,
            transparent 2px,
            rgba(255, 0, 56, 0.03) 2px,
            rgba(255, 0, 56, 0.03) 4px
          )
        `,
      },
      backgroundSize: {
        'honeycomb': '60px 60px',
        'honeycomb-hex': '60px 60px',
      },
      fontFamily: {
        'axiom': ['Inter', 'system-ui', 'sans-serif'],
        'mono': ['JetBrains Mono', 'Fira Code', 'monospace'],
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'glow': 'glow 2s ease-in-out infinite alternate',
      },
      keyframes: {
        glow: {
          '0%': { 'box-shadow': '0 0 5px rgba(255, 0, 56, 0.5)' },
          '100%': { 'box-shadow': '0 0 20px rgba(255, 0, 56, 0.8)' },
        },
      },
    },
  },
  plugins: [],
  // Enforce Zero Entropy: disable all probabilistic features
  corePlugins: {
    // Ensure deterministic styling
  },
}

