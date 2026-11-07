import { relative } from 'node:path';
import { fileURLToPath } from 'node:url';

const ui = fileURLToPath(new URL('.', import.meta.url));

/** @param {{ hash: (s: string) => string, css: string, filename?: string }} o */
function cssHash({ hash, css, filename }) {
	if (!filename) return `svelte-${hash(css)}`;
	const name = filename.startsWith(ui)
		? `zipp-ui/${filename.slice(ui.length)}`
		: relative(process.cwd(), filename);
	return `svelte-${hash(name)}`;
}

/**
 * The vite config of a plugin's studio module, its `vite.config.js` is
 * `export default pluginConfig({ svelte, vitePreprocess, file: 'seo.js' })`.
 * It builds `src/index.svelte.js` into one minified ES module
 * `../assets/<file>` with its own Svelte and css, zipp serves it to the
 * studio (not lib mode, that keeps the whitespace of ES modules). The
 * plugin passes its own vite-plugin-svelte; zipp-ui's components are
 * scss, so it needs `sass` too. Svelte hashes a component's file name
 * into its css class, which vite leaves absolute for zipp-ui (outside the
 * plugin's root): `cssHash` hashes it from zipp-ui's folder instead, so a
 * build is the same in every checkout.
 *
 * @param {{ svelte: Function, vitePreprocess: Function, file: string }} o
 */
export function pluginConfig({ svelte, vitePreprocess, file }) {
	return {
		plugins: [
			svelte({
				preprocess: vitePreprocess(),
				compilerOptions: { css: 'injected', cssHash },
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
