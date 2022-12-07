/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*.rs"],
  theme: {
    extend: {
      colors: {
        screen: "var(--nj-screen)",
        action: "var(--nj-action)",
        "action-hover": "var(--nj-action-hover)",
        "action-disable": "var(--nj-action-disable)",
        "action-label": "var(--nj-action-label)",
        separator: "var(--nj-separator)",
        word: "var(--nj-word)",
        "word-2nd": "var(--nj-word-2nd)",
        "word-disable": "var(--nj-word-disable)",
        layer: "var(--nj-layer)",
        "layer-2nd": "var(--nj-layer-2nd)",
      }
    },

  },
  plugins: [],
}
