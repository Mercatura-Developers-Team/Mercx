/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
      "./index.html",
      "./src/**/*.{js,jsx,ts,tsx}", // Adjust this line if you have different file extensions
    ],
    theme: {
      extend: {
        backgroundImage: {
          'gradient-to-r-indigo-500-700': 'linear-gradient(to right, #667eea, #4c51bf)', // from indigo-500 to indigo-700
            'gradient-to-r-indigo-700-darker': 'linear-gradient(to right, #4c51bf, #434190)', // from indigo-700 to a darker indigo
          }
      },
    },
    plugins: [],
  };