import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

// the studio is part of the zipp crate, it has its version
const cargo = readFileSync(
	new URL('../crates/zipp/Cargo.toml', import.meta.url),
	'utf8',
);
const version = cargo.match(/^version = "(.+)"/m)![1];

// breakpoints and mixins in every style block of the studio, see
// ui/vars.scss; the components of zipp-ui load them themselves
const vars = fileURLToPath(new URL('./ui/vars.scss', import.meta.url));
const ui = fileURLToPath(new URL('./ui/', import.meta.url));

// the build is embedded in the zipp crate, in dev the api runs on
// `example-cms serve`
export default defineConfig({
	plugins: [svelte()],
	define: { __ZIPP_VERSION__: JSON.stringify(version) },
	css: {
		preprocessorOptions: {
			scss: {
				additionalData: (source: string, file: string) =>
					file.startsWith(ui)
						? source
						: `@use '${vars}' as *;\n${source}`,
			},
		},
	},
	// /assets are the uploaded files
	build: {
		outDir: '../crates/zipp/studio',
		emptyOutDir: true,
		assetsDir: '_studio',
	},
	// what isn't the studio comes from the cms: the api, uploaded files,
	// the editors of field types and plugins
	server: {
		proxy: Object.fromEntries(
			['/api', '/assets', '/_fields', '/_plugins'].map(path => [
				path,
				'http://127.0.0.1:8090',
			]),
		),
	},
});
