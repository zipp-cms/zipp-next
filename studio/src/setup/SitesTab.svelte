<script lang="ts">
	import type { Schema } from '../core/schema';
	import { accessText, isAdmin, roleName } from '../core/access';
	import { visibleRoles, visibleSections } from './setup';
	import Table from 'zipp-ui/Table.svelte';
	import MailCard from './MailCard.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * The sites, the image transforms and the roles of the code, for
	 * admins how mails are sent.
	 */
	let { schema }: { schema: Schema } = $props();

	const sections = $derived(visibleSections(schema));
	const roles = $derived(visibleRoles(schema));
</script>

<div class="cards">
	<div class="card">
		<h2>{t('setup.sites')}</h2>
		<Table
			rows={schema.sites}
			key={s => s.handle}
			columns={[
				t('common.name'),
				t('setup.handle'),
				t('setup.language'),
				t('setup.url'),
			]}
		>
			{#snippet row(s, i)}
				<td>
					{s.name}{i === 0
						? t('setup.primary')
						: ''}{schema.offline.includes(s.handle)
						? t('setup.offlineSite')
						: ''}
				</td>
				<td><code>{s.handle}</code></td>
				<td>{s.language}</td>
				<td>{schema.siteUrls[s.handle] ?? '–'}</td>
			{/snippet}
		</Table>
	</div>

	<div class="card">
		<h2>{t('setup.transforms')}</h2>
		<Table rows={schema.transforms} key={tf => tf.handle}>
			{#snippet row(tf)}
				<td><code>{tf.handle}</code></td>
				<td>
					{`${tf.width ?? t('setup.auto')} × ${tf.height ?? t('setup.auto')}${tf.crop ? t('setup.cropped') : ''}${tf.format ? `, ${tf.format}` : ''}`}
				</td>
			{/snippet}
		</Table>
	</div>

	<div class="card" id="roles">
		<h2>{t('setup.roles')}</h2>
		<p class="muted">
			{t('setup.youAre', {
				who: isAdmin(schema) ? t('setup.anAdmin') : roleName(schema),
			})}
		</p>
		<Table
			rows={roles}
			key={r => r.handle}
			columns={[
				t('setup.role'),
				...sections.map(s => s.name),
				t('setup.assets'),
				t('setup.ai'),
				t('setup.setup'),
			]}
		>
			{#snippet row(r)}
				<td>
					{r.name}
					<code>{r.handle}</code>
				</td>
				{#each sections as s (s.handle)}
					<td>{accessText(r.sections[s.handle])}</td>
				{/each}
				<td>{accessText(r.assets)}</td>
				<td>{r.ai ? t('common.yes') : '–'}</td>
				<td>{r.setup ? t('common.yes') : '–'}</td>
			{/snippet}
		</Table>
	</div>

	{#if isAdmin(schema)}
		<MailCard />
	{/if}
</div>
