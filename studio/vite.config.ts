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

// breakpoints and mixins in every style block, see sass/vars.scss
const vars = fileURLToPath(new URL('./src/sass/vars.scss', import.meta.url));

// the build is embedded in the zipp crate, in dev the api runs on
// `example-cms serve`
export default defineConfig({
	plugins: [svelte()],
	define: { __ZIPP_VERSION__: JSON.stringify(version) },
	css: {
		preprocessorOptions: {
			scss: { additionalData: `@use '${vars}' as *;\n` },
		},
	},
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
