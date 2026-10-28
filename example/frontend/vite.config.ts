import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// the content and the asset files come from `example-cms serve`
const cms = process.env.ZIPP_API ?? 'http://127.0.0.1:8090';
const proxy = { '/api': cms, '/assets': cms };

export default defineConfig({
	plugins: [svelte()],
	server: { proxy },
	// `vite preview` runs the demo behind a proxy
	preview: { proxy, allowedHosts: true },
	// /assets are the files of the cms
	build: { assetsDir: '_app' },
});
