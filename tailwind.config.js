/** @type {import('tailwindcss').Config} */

const plugin = require("tailwindcss/plugin")
const colors = require("tailwindcss/colors")

module.exports = {
  plugins: [
    require("daisyui"),
    plugin(({ addVariant }) => {
      addVariant("focus-or-active", "&:is(:focus, :active)")
      addVariant("focus-active-or-hover", "&:is(:focus, :active, :hover)")
    }),
  ],
  content: ["./src/**/*.{html,rs,css}"],
  theme: {
    extend: {
      spacing: {
        input: "0.75em",
      },
      colors: {
        transparent: "transparent",
        // For impact points (e.g. btns)
        primary: colors.sky,
        // For gradients
        "primary-analogous": colors.purple,
        // For large blocks
        base: colors.slate,
      },
      transitionDuration: {
        input: "100ms",
      },
      backgroundImage: {
        "random-img": "url(https://picsum.photos/536/354)",
      },
    },
    daisyui: {
      themes: [
        {
          light: {
            primary: "#a991f7",
            secondary: "#f6d860",
            accent: "#37cdbe",
            neutral: "#3d4451",
            "base-100": "#ffffff",
          },
        },
        "dark",
      ],
    },
  },
}
