/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [],
  mode: 'jit', // Just-In-Time Mode
  purge: [
    './src/**/*.{rs,html}'
  ],
};
