/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				primary: 'rgb(var(--primary) / <alpha-value>)',
				foreground: 'rgb(var(--foreground) / <alpha-value>)',
				background: 'rgb(var(--background) / <alpha-value>)'
			}
		}
	},
	plugins: []
};
