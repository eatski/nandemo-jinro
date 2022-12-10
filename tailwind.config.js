/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*.rs"],
  theme: {
    extend: {
      colors: {
        screen: "var(--nj-screen)",
        "screen-2nd": "var(--nj-screen-2nd)",
        action: "var(--nj-action)",
        "action-hover": "var(--nj-action-hover)",
        "action-disable": "var(--nj-action-disable)",
        "action-label": "var(--nj-action-label)",
        "action-label-disable": "var(--nj-action-label-disable)",
        separator: "var(--nj-separator)",
        word: "var(--nj-word)",
        "word-2nd": "var(--nj-word-2nd)",
        "word-disable": "var(--nj-word-disable)",
        layer: "var(--nj-layer)",
        "layer-2nd": "var(--nj-layer-2nd)",
        error: "var(--nj-error)",
      }
    },

  },
  plugins: [],
}
