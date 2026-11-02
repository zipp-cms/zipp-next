/**
 * The vite config of a plugin's studio module, its `vite.config.js` is
 * `export default pluginConfig({ svelte, vitePreprocess, file: 'seo.js' })`.
 * It builds `src/index.svelte.js` into one minified ES module
 * `../assets/<file>` with its own Svelte and css, zipp serves it to the
 * studio (not lib mode, that keeps the whitespace of ES modules). The
 * plugin passes its own vite-plugin-svelte; zipp-ui's components are
 * scss, so it needs `sass` too.
 *
 * @param {{ svelte: Function, vitePreprocess: Function, file: string }} o
 */
export function pluginConfig({ svelte, vitePreprocess, file }) {
	return {
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
					entryFileNames: file,
					inlineDynamicImports: true,
				},
			},
		},
	};
}
