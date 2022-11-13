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
      colors: {
        transparent: "transparent",
        // For impact points (e.g. btns)
        primary: colors.sky,
        // For gradients
        "primary-analogous": colors.purple,
        // For large blocks
        base: colors.slate,
      },
      backgroundImage: {
        "random-img": "url(https://picsum.photos/536/354)",
      },
    },
    daisyui: {
      themes: [
        {
          light: {
            primary: colors.sky[500],
            secondary: colors.purple[500],
            accent: colors.fuchsia[500],
            neutral: colors.slate[500],
            "base-100": colors.slate[50],
          },
        },
      ],
    },
  },
}
