<script lang="ts">
	import type { Schema } from '../core/schema';
	import { setSite, site } from '../core/router.svelte';
	import Icon from '../ui/Icon.svelte';

	/** The site the studio edits, and where it runs; the box opens a choice. */
	let { schema }: { schema: Schema } = $props();

	const current = $derived(schema.sites.find(s => s.handle === site.current));
	const offline = (handle: string) => schema.offline.includes(handle);
</script>

<label class="site-switch" title="Site">
	<span class="mark">{(current?.name ?? '?')[0]}</span>
	<span class="name">
		<strong class="truncate">{current?.name}</strong>
		<small>
			{schema.dev ? 'Development' : 'Production'}{offline(site.current)
				? ' · offline'
				: ''}
		</small>
	</span>
	{#if schema.sites.length > 1}
		<Icon name="chevrons-up-down" />
		<select
			value={site.current}
			onchange={e => setSite(e.currentTarget.value)}
		>
			{#each schema.sites as s (s.handle)}
				<option value={s.handle}>
					{s.name}{offline(s.handle) ? ' (offline)' : ''}
				</option>
			{/each}
		</select>
	{/if}
</label>

<style lang="scss">
	.site-switch {
		position: relative;
		display: flex;
		align-items: center;
		gap: 0.625rem;
		padding: 0.5rem 0.625rem;
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		color: var(--muted);
	}

	.mark {
		display: grid;
		place-items: center;
		width: 1.75rem;
		height: 1.75rem;
		flex-shrink: 0;
		border-radius: var(--radius);
		background: var(--accent);
		color: var(--on-accent);
		font-weight: 700;
		font-size: var(--fs-md);
	}

	.name {
		display: flex;
		flex-direction: column;
		margin-right: auto;
		min-width: 0;

		strong {
			color: var(--text);
			font-size: var(--fs-md);
		}

		small {
			font-size: var(--fs-xs);
		}
	}

	// the whole box opens the choice
	select {
		position: absolute;
		inset: 0;
		opacity: 0;
		cursor: pointer;
	}
</style>
