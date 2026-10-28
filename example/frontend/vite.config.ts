import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// the content comes from `example-cms serve`
const proxy = { '/api': process.env.ZIPP_API ?? 'http://127.0.0.1:8090' };

export default defineConfig({
	plugins: [svelte()],
	server: { proxy },
	// `vite preview` runs the demo behind a proxy
	preview: { proxy, allowedHosts: true },
});
