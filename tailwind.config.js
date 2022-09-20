/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        feature: "#0ea5e9",
        "feature-light": "#38bdf8",
        quiet: "#bae6fd"
      }
    },
    
  },
  plugins: [],
}
