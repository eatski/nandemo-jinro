/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*.rs"],
  theme: {
    extend: {
      colors: {
        feature: "#0ea5e9",
        "feature-light": "#38bdf8",
        quiet: "#bae6fd",
        line: "#e2e8f0",
        black: "#334155",
        "black-light": "#64748b",
      }
    },
    
  },
  plugins: [],
}
