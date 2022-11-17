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
        black: "#111827",
        "black-light": "#374151",
        "black-quiet": "#cbd5e1",
        colored: "#f5f5f4",
        "colored-light": "#fafaf9",
      }
    },

  },
  plugins: [],
}
