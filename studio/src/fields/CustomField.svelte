<script module lang="ts">
	/** The editor of a field type of another crate, loaded once. */
	export type Mounted = { update(props: object): void; destroy(): void };
	type Module = { mount(target: HTMLElement, props: object): Mounted };

	const modules = new Map<string, Promise<Module>>();

	function load(ty: string): Promise<Module> {
		let m = modules.get(ty);
		if (!m) {
			m = import(/* @vite-ignore */ `/_fields/${ty}.js`);
			modules.set(ty, m);
		}
		return m;
	}
</script>

<script lang="ts">
	import type { FieldOf, Values } from '../core/schema';
	import { host } from './host.svelte';
	import { getEditing, editedSite } from '../core/editing';
	import { Last } from '../core/synced.svelte';

	/**
	 * A field of a type of another crate: zipp serves its module, which
	 * brings its own framework; the studio mounts it, passes the value in
	 * and out, the entry it belongs to (`entryId` null for a new one, the
	 * site) and the `host` api.
	 */
	let { field, values }: { field: FieldOf<'custom'>; values: Values } =
		$props();

	const editing = getEditing();
	const site = editedSite();
	// the value as the plugin sent it or got it, not given back to it
	const last = new Last();

	let target: HTMLElement;
	let mounted = $state.raw<Mounted | null>(null);
	let error = $state('');

	function current() {
		return {
			field: $state.snapshot(field),
			value: $state.snapshot(values[field.handle]) ?? null,
			onchange: (v: unknown) => {
				last.changed(v);
				values[field.handle] = v;
			},
			entryId: editing?.entryId ?? null,
			site,
			host,
		};
	}

	$effect(() => {
		let gone = false;
		let m: Mounted | null = null;
		load(field.type)
			.then(mod => {
				if (gone) return;
				const props = current();
				last.changed(props.value);
				m = mod.mount(target, props);
				mounted = m;
			})
			.catch(e => (error = `${field.type}: ${e.message}`));
		return () => {
			gone = true;
			m?.destroy();
		};
	});

	// changes from outside, like a restored revision; what the plugin
	// sent itself doesn't go back, it could send it again and again
	$effect(() => {
		const p = current();
		if (mounted && last.changed(p.value)) mounted.update(p);
	});
</script>

<div bind:this={target}></div>
{#if error}<span class="error">{error}</span>{/if}
