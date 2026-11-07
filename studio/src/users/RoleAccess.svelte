<script lang="ts">
	import type { Schema } from '../core/schema';
	import { accessText } from '../core/access';
	import { sitesPath } from '../core/paths';
	import Sentence from '../i18n/Sentence.svelte';
	import { t } from '../i18n/index.svelte';

	/** What a role may do, from the code: per section, assets, AI, Setup. */
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
		<dt>{t('users.studio')}</dt>
		<dd>{def.studio ? t('common.yes') : t('users.websiteOnly')}</dd>
		{#each sections as s (s.handle)}
			<dt>{s.name}</dt>
			<dd>{accessText(def.sections[s.handle])}</dd>
		{/each}
		<dt>{t('users.assets')}</dt>
		<dd>{accessText(def.assets)}</dd>
		<dt>{t('users.ai')}</dt>
		<dd>{def.ai ? t('common.yes') : '–'}</dd>
		<dt>{t('users.setup')}</dt>
		<dd>{def.setup ? t('common.yes') : '–'}</dd>
	</dl>
	<p class="note">
		<Sentence key="users.levels" params={{ setup: t('users.setup') }}>
			{#snippet param(p)}
				<a href={sitesPath('roles')}>{p.text}</a>
			{/snippet}
		</Sentence>
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
