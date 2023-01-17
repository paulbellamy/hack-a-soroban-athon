/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      backgroundImage: {
        'hero-pattern': "radial-gradient(circle at 10% 200px, rgba(205,194,246,1) 0%, rgba(205,194,246,0.95) 10%, rgba(205,194,246,0) 20%), radial-gradient(circle at 90% 650px, rgba(205,194,246,1) 0%, rgba(205,194,246,0.95) 15%, rgba(255,255,255,1) 30%)",
      }
    },
    colors: {
      "button-primary": "#444BF7",
      "button-secondary": "#F5F5F5",
      "primary": "#000000",
      "secondary": "#555555",
      "tertiary": "#ffffff",
      "disabled": "#c8c8c8",
      "purple": "#444BF7",
      "card-background": "#ffffff",
      "card-secondary": "#f9f9f9",
      "divider": "rgba(0, 0, 0, 0.09)",
      "blue": "#414caa",
      "purple-background": "#6d62f7",
      "purple-background-light": "#8d85f7",
      "success": "#3dd34c",
      "warning": "#ffbe64",
      "error": "#ff5064",
      "info": "#2280ff",
    },
    fontFamily: {
      sans: ['Inter', 'sans-serif'],
      serif: ['Merriweather', 'serif'],
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}
