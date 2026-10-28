import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// in dev the api runs on `example-cms serve`
export default defineConfig({
	plugins: [svelte()],
	server: {
		proxy: { '/api': 'http://127.0.0.1:8090' },
	},
});
