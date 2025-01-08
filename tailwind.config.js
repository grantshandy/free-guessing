/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{svelte,ts}"],
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["light"],
  },
}

