<script lang="ts">
	import type {
		Component,
		Entry,
		Revision,
		Schema,
		Section,
	} from '../../api';
	import { site } from '../../route.svelte';
	import RevisionList from './RevisionList.svelte';

	/** Published, type, copying from another site, revisions, parent. */
	let {
		schema,
		secDef,
		comp,
		isNew,
		published = $bindable(),
		ty = $bindable(),
		parentId = $bindable(),
		parents,
		revisions,
		oncopy,
		onview,
		onrestore,
	}: {
		schema: Schema;
		secDef: Section;
		comp: Component;
		isNew: boolean;
		published: boolean;
		ty: string;
		parentId: number | null;
		parents: Entry[];
		revisions: Revision[];
		oncopy: (site: string) => void;
		onview: (rev: Revision) => void;
		onrestore: (rev: Revision) => void;
	} = $props();
</script>

<div class="options">
	<span>Published</span>
	<input type="checkbox" bind:checked={published} />
	<span>Section</span>
	<span>{secDef.name}</span>
	<span>Type</span>
	{#if isNew && secDef.types.length > 1}
		<select bind:value={ty}>
			{#each secDef.types as t (t)}
				<option value={t}>
					{schema.components.find(c => c.handle === t)?.name}
				</option>
			{/each}
		</select>
	{:else}
		<span>{comp.name}</span>
	{/if}
	{#if !isNew && schema.sites.length > 1}
		<span>Content</span>
		<select
			value=""
			onchange={e => {
				const other = e.currentTarget.value;
				e.currentTarget.value = '';
				oncopy(other);
			}}
		>
			<option value="" disabled>Copy from another site…</option>
			{#each schema.sites.filter(s => s.handle !== site.current) as s (s.handle)}
				<option value={s.handle}>{s.name}</option>
			{/each}
		</select>
	{/if}
	{#if revisions.length}
		<span>Revisions</span>
		<RevisionList
			{revisions}
			{comp}
			multiSite={schema.sites.length > 1}
			{onview}
			{onrestore}
		/>
	{/if}
	{#if isNew && parents.length}
		<span>Parent</span>
		<select bind:value={parentId}>
			<option value={null}>None, on the top</option>
			{#each parents as p (p.id)}
				<option value={p.id}>{p.title}</option>
			{/each}
		</select>
	{/if}
</div>

<style lang="scss">
	.options {
		display: grid;
		grid-template-columns: 200px 1fr;
		gap: 16px 30px;
		align-items: center;

		select {
			max-width: 24rem;
		}
	}

	@media (max-width: 800px) {
		.options {
			grid-template-columns: 1fr;
		}
	}
</style>
