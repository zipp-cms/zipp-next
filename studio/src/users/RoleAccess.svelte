<script lang="ts">
	import type { Schema } from '../core/schema';
	import { accessText } from '../core/access';
	import { t } from '../i18n/index.svelte';

	/** What a role may do, from the code: per section, assets, AI. */
	let { schema, role }: { schema: Schema; role: string | null } = $props();

	const def = $derived(schema.roles.find(r => r.handle === role));
	const sections = $derived(
		schema.sections.filter(s => def?.sections[s.handle]),
	);
</script>

{#if !def}
	<p class="muted">{t('users.adminMay')}</p>
{:else}
	<dl>
		{#each sections as s (s.handle)}
			<dt>{s.name}</dt>
			<dd>{accessText(def.sections[s.handle])}</dd>
		{/each}
		<dt>{t('users.assets')}</dt>
		<dd>{accessText(def.assets)}</dd>
		<dt>{t('users.ai')}</dt>
		<dd>{def.ai ? t('common.yes') : '–'}</dd>
	</dl>
	{@const [before, after] = t('users.levels').split('{setup}')}
	<p class="note">
		{before}
		<a href="/admin/setup/sites#roles">{t('users.setup')}</a>
		{after}
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
