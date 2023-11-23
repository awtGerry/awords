/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    darkMode: "class",
    theme: {
      extend: {
        fontFamily: {
          pacifico: ['Pacifico', 'cursive'],
        },
        colors: {
          'aw-bg': '#1A1A1A',
          'aw-fg': '#F8FBEA',
          'aw-green': '#C5EE4F',
          'aw-green-light': '#BAE4AC',
          'aw-green-dark': '#56704C',
          'aw-red': '#FF7B72',
        },
      },
    },
    plugins: [],
}
