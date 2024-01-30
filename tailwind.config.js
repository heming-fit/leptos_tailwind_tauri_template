/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./csr/src/main.rs", "./csr/src/**/*.{html,js}"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
}