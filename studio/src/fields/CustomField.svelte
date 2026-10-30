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
	import type { Field, Values } from '../core/schema';
	import { host } from './host.svelte';
	import { getEditing } from '../core/editing';
	import { site } from '../core/router.svelte';

	/**
	 * A field of a type of another crate: zipp serves its module, which
	 * brings its own framework; the studio mounts it, passes the value in
	 * and out, the entry it belongs to (`entryId` null for a new one, the
	 * site) and the `host` api.
	 */
	let { field, values }: { field: Field; values: Values } = $props();

	const editing = getEditing();

	let target: HTMLElement;
	let mounted = $state.raw<Mounted | null>(null);
	let error = $state('');

	function current() {
		return {
			field: $state.snapshot(field),
			value: $state.snapshot(values[field.handle]) ?? null,
			onchange: (v: unknown) => (values[field.handle] = v),
			entryId: editing?.entryId ?? null,
			site: editing?.site ?? site.current,
			host,
		};
	}

	$effect(() => {
		if (field.kind !== 'custom') return;
		let gone = false;
		let m: Mounted | null = null;
		load(field.type)
			.then(mod => {
				if (gone) return;
				m = mod.mount(target, current());
				mounted = m;
			})
			.catch(e => (error = `${field.type}: ${e.message}`));
		return () => {
			gone = true;
			m?.destroy();
		};
	});

	// changes from outside, like a restored revision
	$effect(() => {
		const p = current();
		mounted?.update(p);
	});
</script>

<div bind:this={target}></div>
{#if error}<span class="error">{error}</span>{/if}
