import { svelte, vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { pluginConfig } from 'zipp-ui/vite';

// one ES module for the studio, see zipp-ui/vite.js
export default pluginConfig({
	svelte,
	vitePreprocess,
	file: 'button-group.js',
});
