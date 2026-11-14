import { host } from '../fields/host.svelte';

type Mounted = { destroy(): void };

/**
 * Mounts what the export `fn` of a plugin's module `/_plugins/<name>.js`
 * renders into `target`: it gets `props` with the plugin's `data` and the
 * `host`, and returns `{ destroy() }`. A module without the export shows
 * an error, unless it is `optional` (a setup card).
 */
export function mountPlugin(
	target: HTMLElement,
	plugin: { name: string; data: unknown },
	fn: string,
	props: Record<string, unknown>,
	optional = false,
) {
	let mounted: Mounted | null = null;
	let gone = false;
	import(/* @vite-ignore */ `/_plugins/${plugin.name}.js`)
		.then(m => {
			if (gone) return;
			if (!m[fn]) {
				if (!optional) throw new Error(`no export ${fn}`);
				return;
			}
			mounted = m[fn](target, { ...props, data: plugin.data, host });
		})
		.catch(e => (target.textContent = `${plugin.name}: ${e.message}`));

	return {
		destroy() {
			gone = true;
			mounted?.destroy();
		},
	};
}
