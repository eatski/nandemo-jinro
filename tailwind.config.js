/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        feature: "#38bdf8",
        "feature-light": "#0ea5e9",
      }
    },
    
  },
  plugins: [],
}
