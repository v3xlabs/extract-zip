/** @type {import('tailwindcss').Config} */
module.exports = {
    purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
        ],
    },
    theme: {
        extend: {},
    },
    plugins: [],
};
