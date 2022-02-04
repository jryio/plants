// IMPORTANT - This tailwind config file will be run within a docker container
module.exports = {
  mode: 'jit',
  // If we're running one directory higher this shoudl still find our content files
  content: [
    "**/index.html",
    "**/src/**/*.rs"
  ],
  important: true,
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      fontFamily: {
        sans: "system-ui, -apple-system, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji'",
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
