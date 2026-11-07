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
	import { getAllContexts, untrack } from 'svelte';
	import type { FieldProps } from 'zipp-ui/field';
	import type { FieldOf, Schema, Values } from '../core/schema';
	import type { Refs } from '../entries/api';
	import { host } from './host.svelte';
	import { itemBlocks } from './itemBlocks';
	import { getEditing, editedSite } from '../core/editing';
	import { Last } from '../core/synced.svelte';
	import { isReadonly, watchReadonly } from './readonly';

	/**
	 * A field of a type of another crate: zipp serves its module, which
	 * brings its own framework; the studio mounts it with `FieldProps`
	 * (zipp-ui/field) and updates them. A module that throws breaks only
	 * its field.
	 */
	let props: {
		schema: Schema;
		field: FieldOf<'custom'>;
		values: Values;
		errors: Record<string, string>;
		refs: Refs;
		path: string;
		changed?: Set<string>;
	} = $props();
	const { field, values, errors, path } = $derived(props);

	// its items hold blocks: their editor is the studio's, with the
	// context here (the entry, the site)
	const blocks = untrack(() => props.field.blocks?.length)
		? itemBlocks(
				{
					get schema() {
						return props.schema;
					},
					get field() {
						return props.field;
					},
					get values() {
						return props.values;
					},
					get errors() {
						return props.errors;
					},
					get refs() {
						return props.refs;
					},
					get path() {
						return props.path;
					},
					get changed() {
						return props.changed ?? new Set<string>();
					},
				},
				getAllContexts(),
			)
		: null;

	const editing = getEditing();
	const site = editedSite();
	// the value as the plugin sent it or got it, not given back to it
	const last = new Last();

	let target: HTMLElement;
	let mounted = $state.raw<Mounted | null>(null);
	let error = $state('');
	// a revision, or the role may not edit; a fieldset can change after
	// the start (a conflict, publishing)
	let readonly = $state(false);
	$effect(() => {
		readonly = isReadonly(target);
		return watchReadonly(target, r => (readonly = r));
	});

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
			readonly,
			error: errors[path] ?? null,
			host,
			blocks,
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
