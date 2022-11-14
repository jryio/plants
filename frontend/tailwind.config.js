module.exports = {
  // If we're running one directory higher this shoudl still find our content files
  content: [
    "**/index.html",
    "**/src/**/*.rs"
  ],
  important: true,
  // darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      fontFamily: {
        sans: "system-ui, -apple-system, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji'",
      }
    },
  },
  // Plugins can work without installing Node.js. The standalone
  // tailwindcss-cli binary comes pre-compiled with the latest versions of
  // first-party plugins.
  //
  // Documentation: https://tailwindcss.com/blog/standalone-cli
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ]
}
