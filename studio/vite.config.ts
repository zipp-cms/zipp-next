import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// the build is embedded in the zipp crate, in dev the api runs on
// `example-cms serve`
export default defineConfig({
	plugins: [svelte()],
	build: { outDir: '../crates/zipp/studio', emptyOutDir: true },
	server: {
		proxy: { '/api': 'http://127.0.0.1:8090' },
	},
});
