import { defineConfig } from 'vite';
import { svelte, vitePreprocess } from '@sveltejs/vite-plugin-svelte';

// one minified ES module with its own Svelte and css, zipp serves it to
// the studio (not lib mode, that keeps the whitespace of ES modules)
export default defineConfig({
	// zipp-ui's components are scss
	plugins: [
		svelte({
			preprocess: vitePreprocess(),
			compilerOptions: { css: 'injected' },
		}),
	],
	build: {
		outDir: '../assets',
		emptyOutDir: true,
		rollupOptions: {
			input: 'src/index.svelte.js',
			preserveEntrySignatures: 'strict',
			output: {
				format: 'es',
				entryFileNames: 'seo.js',
				inlineDynamicImports: true,
			},
		},
	},
});
