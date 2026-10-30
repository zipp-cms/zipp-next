<script module lang="ts">
	import type { FieldModule, Mounted } from 'zipp-ui/field';

	/** The editor of a field type of another crate, loaded once. */
	const modules = new Map<string, Promise<FieldModule>>();

	function load(ty: string): Promise<FieldModule> {
		let m = modules.get(ty);
		if (!m) {
			m = import(/* @vite-ignore */ `/_fields/${ty}.js`);
			// a failed load is tried again the next time
			m.catch(() => modules.delete(ty));
			modules.set(ty, m);
		}
		return m;
	}
</script>

<script lang="ts">
	import type { FieldProps } from 'zipp-ui/field';
	import type { FieldOf, Values } from '../core/schema';
	import { host } from './host.svelte';
	import { getEditing, editedSite } from '../core/editing';
	import { Last } from '../core/synced.svelte';

	/**
	 * A field of a type of another crate: zipp serves its module, which
	 * brings its own framework; the studio mounts it with `FieldProps`
	 * (zipp-ui/field) and updates them. A module that throws breaks only
	 * its field.
	 */
	let {
		field,
		values,
		errors = {},
		path = '',
	}: {
		field: FieldOf<'custom'>;
		values: Values;
		errors?: Record<string, string>;
		path?: string;
	} = $props();

	const editing = getEditing();
	const site = editedSite();
	// the value as the plugin sent it or got it, not given back to it
	const last = new Last();

	let target: HTMLElement;
	let mounted = $state.raw<Mounted | null>(null);
	let error = $state('');

	function current(): FieldProps {
		return {
			field: $state.snapshot(field),
			value: $state.snapshot(values[field.handle]) ?? null,
			onchange: (v: unknown) => {
				last.changed(v);
				values[field.handle] = v;
			},
			entryId: editing?.entryId ?? null,
			site,
			// a revision, or the role may not edit
			readonly: !!target?.closest('fieldset[disabled]'),
			error: errors[path] ?? null,
			host,
		};
	}

	/** Calls into the module, what it throws shows at the field. */
	function safely(f: () => void) {
		try {
			f();
		} catch (e: any) {
			error = `${field.type}: ${e?.message ?? e}`;
		}
	}

	$effect(() => {
		let gone = false;
		let m: Mounted | null = null;
		load(field.type)
			.then(mod => {
				if (gone) return;
				const props = current();
				last.changed(props.value);
				safely(() => (m = mod.mount(target, props)));
				mounted = m;
			})
			.catch(e => (error = `${field.type}: ${e.message}`));
		return () => {
			gone = true;
			if (m) safely(() => m!.destroy());
			mounted = null;
		};
	});

	// changes from outside: a restored revision, the id of a new entry
	// after its first save, an error; what the plugin sent itself doesn't
	// go back, it could send it again and again
	let others = '';
	$effect(() => {
		const p = current();
		const now = JSON.stringify([p.entryId, p.readonly, p.error]);
		const changed = last.changed(p.value) || now !== others;
		others = now;
		if (mounted && changed) safely(() => mounted!.update(p));
	});
</script>

<div bind:this={target}></div>
{#if error}<span class="error">{error}</span>{/if}
