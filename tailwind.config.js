/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*.rs"],
  theme: {
    extend: {
      colors: {
        feature: "#0ea5e9",
        "feature-light": "#38bdf8",
        quiet: "#bae6fd",
        line: "#cbd5e1",
        black: "#334155",
        "black-light": "#64748b",
        colored: "#f5f5f4",
      }
    },
    
  },
  plugins: [],
}
