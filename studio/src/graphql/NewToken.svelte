<script lang="ts">
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Switch from 'zipp-ui/Switch.svelte';
	import CheckGroup from 'zipp-ui/CheckGroup.svelte';
	import DatePicker from 'zipp-ui/DatePicker.svelte';
	import { formatDay } from 'zipp-ui/date';
	import type { Schema } from '../core/schema';
	import { go } from '../core/router.svelte';
	import { TokenForm } from './tokenForm.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * A new token for a frontend: a name, the sections and sites it reads,
	 * whether preview tokens show drafts, until when. The token shows once
	 * after creating, only its hash is kept.
	 */
	let { schema }: { schema: Schema } = $props();

	const form = new TokenForm();
	// the editor's day, not UTC's: after midnight here it is tomorrow
	const today = formatDay(new Date());

	function save(e: SubmitEvent) {
		e.preventDefault();
		form.save();
	}
</script>

<PageTop
	crumbs={[
		{ label: 'GraphQL', href: '/admin/graphql' },
		{ label: t('graphql.newToken') },
	]}
>
	{#snippet actions()}
		{#if form.created}
			<button class="primary" onclick={() => go('/admin/graphql')}>
				{t('common.done')}
			</button>
		{:else}
			<button class="primary" form="token" disabled={form.busy}>
				{t('common.create')}
			</button>
		{/if}
	{/snippet}
</PageTop>

{#if form.error && !Object.keys(form.fields).length}
	<p class="error">{form.error}</p>
{/if}

{#if form.created}
	{@const [a, b, c] = t('graphql.copyNow').split(/\s*\{\w+\}\s*/)}
	<div class="card">
		<h2>{form.name}</h2>
		<p>
			{a}
			<code>Authorization: Bearer …</code>
			{b}
			<code>ENDPOINT_TOKEN</code>
			{c}
		</p>
		<div class="token">
			<input
				readonly
				value={form.created}
				aria-label={t('graphql.theToken')}
			/>
			<button onclick={() => form.copy()}>
				<Icon name={form.copied ? 'check' : 'copy'} />
				{form.copied ? t('common.copied') : t('common.copy')}
			</button>
		</div>
	</div>
{:else}
	<form id="token" class="cards" onsubmit={save}>
		<div class="card">
			<label class="stacked">
				{t('common.name')}
				<input
					bind:value={form.name}
					placeholder={t('graphql.namePlaceholder')}
					required
				/>
				{#if form.fields.name}
					<span class="error">{form.fields.name}</span>
				{/if}
			</label>
			<label class="stacked">
				{t('graphql.expires')}
				<DatePicker
					value={form.expires}
					onchange={v => (form.expires = v)}
					placeholder={t('common.never')}
					min={today}
				/>
			</label>
		</div>

		<div class="card">
			<h2>{t('graphql.sections')}</h2>
			<Switch bind:checked={form.allSections}>
				{t('graphql.allSections')}
			</Switch>
			{#if !form.allSections}
				<CheckGroup
					label={t('graphql.sections')}
					options={schema.sections.map(s => ({
						value: s.handle,
						label: s.name,
					}))}
					picked={form.sections}
					ontoggle={(v, on) => form.toggle('sections', v, on)}
				/>
			{/if}
			{#if form.fields.sections}
				<span class="error">{form.fields.sections}</span>
			{/if}
		</div>

		<div class="card">
			<h2>{t('graphql.sites')}</h2>
			<Switch bind:checked={form.allSites}>
				{t('graphql.allSites')}
			</Switch>
			{#if !form.allSites}
				<CheckGroup
					label={t('graphql.sites')}
					options={schema.sites.map(s => ({
						value: s.handle,
						label: s.name,
					}))}
					picked={form.sites}
					ontoggle={(v, on) => form.toggle('sites', v, on)}
				/>
			{/if}
			{#if form.fields.sites}
				<span class="error">{form.fields.sites}</span>
			{/if}
		</div>

		<div class="card">
			<h2>{t('graphql.drafts')}</h2>
			<Switch bind:checked={form.previews}>
				{t('graphql.previewsShowDrafts')}
			</Switch>
			<p class="note">
				{t('graphql.previewsNote')}
			</p>
		</div>
	</form>
{/if}

<style lang="scss">
	.token {
		display: flex;
		gap: 0.5rem;

		input {
			flex: 1;
			font-family: var(--mono);
		}
	}
</style>
