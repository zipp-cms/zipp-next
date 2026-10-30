<script lang="ts">
	import type { Schema } from '../core/schema';
	import { accessText } from '../setup/setup';

	/** What a role may do, from the code: per section, assets, AI. */
	let { schema, role }: { schema: Schema; role: string | null } = $props();

	const def = $derived(schema.roles.find(r => r.handle === role));
	const sections = $derived(
		schema.sections.filter(s => def?.sections[s.handle]),
	);
</script>

{#if !def}
	<p class="muted">
		An admin may do everything: all sections, assets, AI, users and Setup.
	</p>
{:else}
	<dl>
		{#each sections as s (s.handle)}
			<dt>{s.name}</dt>
			<dd>{accessText(def.sections[s.handle])}</dd>
		{/each}
		<dt>Assets</dt>
		<dd>{accessText(def.assets)}</dd>
		<dt>AI</dt>
		<dd>{def.ai ? 'Yes' : '–'}</dd>
	</dl>
	<p class="note">
		View sees entries, Edit changes them as drafts, Publish makes them live,
		Delete deletes them. The roles are code,
		<a href="/admin/setup/sites#roles">Setup</a>
		shows all of them.
	</p>
{/if}

<style lang="scss">
	dl {
		display: grid;
		grid-template-columns: max-content 1fr;
		gap: 0.375rem 1.5rem;
		margin: 0;
	}

	p {
		margin: 0;
	}

	dt {
		color: var(--muted);
	}

	dd {
		margin: 0;
	}
</style>
