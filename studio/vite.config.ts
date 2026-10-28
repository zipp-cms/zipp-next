import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// the build is embedded in the zipp crate, in dev the api runs on
// `example-cms serve`
export default defineConfig({
	plugins: [svelte()],
	// /assets are the uploaded files
	build: {
		outDir: '../crates/zipp/studio',
		emptyOutDir: true,
		assetsDir: '_studio',
	},
	server: {
		proxy: {
			'/api': 'http://127.0.0.1:8090',
			'/assets': 'http://127.0.0.1:8090',
		},
	},
});
