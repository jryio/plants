module.exports = {
  mode: 'jit',
  purge: [
    "src/**/*.rs"
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
