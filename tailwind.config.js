/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*.rs"],
  theme: {
    extend: {
      colors: {
        screen: "var(--nj-color-screen)",
        "screen-2nd": "var(--nj-color-screen-2nd)",
        action: "var(--nj-color-action)",
        "action-hover": "var(--nj-color-action-hover)",
        "action-disable": "var(--nj-color-action-disable)",
        "action-label": "var(--nj-color-action-label)",
        "action-label-disable": "var(--nj-color-action-label-disable)",
        separator: "var(--nj-color-separator)",
        word: "var(--nj-color-word)",
        "word-2nd": "var(--nj-color-word-2nd)",
        "word-disable": "var(--nj-color-word-disable)",
        layer: "var(--nj-color-layer)",
        "layer-2nd": "var(--nj-color-layer-2nd)",
        error: "var(--nj-color-error)",
      }
    },

  },
  plugins: [],
}
