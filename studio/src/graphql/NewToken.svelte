<script module lang="ts">
	import { listTokens } from './api';

	export async function loadProps() {
		const { schemas } = await listTokens();
		return { schemas };
	}
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import DatePicker from 'zipp-ui/DatePicker.svelte';
	import { formatDay } from 'zipp-ui/date';
	import type { Schema } from '../core/schema';
	import { go } from '../core/router.svelte';
	import type { GraphqlSchema } from './api';
	import { reads } from './schemas';
	import { TokenForm } from './tokenForm.svelte';
	import Sentence from '../i18n/Sentence.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * A new token for a frontend: a name, the GraphQL schema of the code
	 * it reads, until when. The token shows once after creating, only its
	 * hash is kept.
	 */
	let { schema, schemas }: { schema: Schema; schemas: GraphqlSchema[] } =
		$props();

	const form = new TokenForm(untrack(() => schemas[0]?.handle ?? ''));
	const picked = $derived(schemas.find(s => s.handle === form.schema));
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
				{t('ui.done')}
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
	<div class="card">
		<h2>{form.name}</h2>
		<p>
			<Sentence
				key="graphql.copyNow"
				params={{
					header: 'Authorization: Bearer …',
					env: 'ENDPOINT_TOKEN',
				}}
			/>
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
			<label class="stacked">
				{t('graphql.schema')}
				<select bind:value={form.schema} required>
					{#each schemas as s (s.handle)}
						<option value={s.handle}>{s.name}</option>
					{/each}
				</select>
				{#if form.fields.schema}
					<span class="error">{form.fields.schema}</span>
				{/if}
			</label>
			{#if picked}
				<p class="note">{reads(picked, schema)}</p>
			{/if}
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
