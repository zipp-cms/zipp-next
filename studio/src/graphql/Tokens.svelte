<script module lang="ts">
	import { listTokens } from './api';

	export async function loadProps() {
		return await listTokens();
	}
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Table from 'zipp-ui/Table.svelte';
	import type { Schema } from '../core/schema';
	import { Busy } from '../core/busy.svelte';
	import { go } from '../core/router.svelte';
	import { date, time } from '../core/format';
	import { deleteToken, openGraphiql, type GraphqlToken } from './api';
	import { t } from '../i18n/index.svelte';

	/**
	 * The tokens frontends send to GraphQL, for admins, like Craft's
	 * schemas: what each reads, until when and when it was used; a new
	 * one, revoking one, and GraphiQL to try queries with the session.
	 */
	let {
		schema,
		tokens: loaded,
		envToken,
	}: { schema: Schema; tokens: GraphqlToken[]; envToken: boolean } = $props();

	let tokens = $state(untrack(() => loaded));
	const action = new Busy();
	// the intro around its two code bits
	const intro = $derived(t('graphql.intro').split(/\s*\{\w+\}\s*/));

	/** The names of some handles, or all. */
	function names(
		handles: string[] | null,
		all: { handle: string; name: string }[],
	) {
		if (handles === null) return t('graphql.all');
		if (!handles.length) return t('common.none');
		return handles
			.map(h => all.find(x => x.handle === h)?.name ?? h)
			.join(', ');
	}

	async function revoke(token: GraphqlToken) {
		const what = t('graphql.confirmRevoke', { name: token.name });
		if (!confirm(what)) return;
		const done = await action.run(() => deleteToken(token.id));
		if (done !== undefined) tokens = tokens.filter(x => x.id !== token.id);
	}
</script>

<PageTop title="GraphQL">
	{#snippet actions()}
		<button onclick={openGraphiql} title={t('graphql.graphiqlTitle')}>
			<Icon name="external" />
			{t('graphql.tryIt')}
		</button>
		<button class="primary" onclick={() => go('/admin/graphql/new')}>
			<Icon name="add" />
			{t('graphql.newToken')}
		</button>
	{/snippet}
</PageTop>

{#if action.error}<p class="error">{action.error}</p>{/if}

<div class="card">
	<p class="muted">
		{intro[0]}
		<code>Authorization: Bearer …</code>
		{intro[1]}
		<code>/api/graphql</code>
		{intro[2]}
		{#if envToken}
			{@const [a, b] = t('graphql.envToken').split(/\s*\{env\}\s*/)}
			{a}
			<code>ZIPP_GRAPHQL_TOKEN</code>
			{b}
		{/if}
	</p>
	<Table
		rows={tokens}
		key={token => token.id}
		columns={[
			t('graphql.token'),
			t('graphql.sections'),
			t('graphql.sites'),
			t('graphql.expires'),
			t('graphql.lastUsed'),
			'',
		]}
		empty={t('graphql.noTokens')}
	>
		{#snippet row(token)}
			<td>
				<strong>{token.name}</strong>
				{#if token.previews}<span class="badge">
						{t('graphql.drafts')}
					</span>{/if}
			</td>
			<td>{names(token.sections, schema.sections)}</td>
			<td>{names(token.sites, schema.sites)}</td>
			<td class="muted">
				{#if token.expiresAt === null}
					{t('common.never')}
				{:else if token.expiresAt < Date.now()}
					<span class="badge">{t('graphql.expired')}</span>
				{:else}
					{date(token.expiresAt)}
				{/if}
			</td>
			<td class="muted">
				{token.lastUsedAt ? time(token.lastUsedAt) : t('common.never')}
			</td>
			<td class="actions">
				<button
					class="icon"
					onclick={() => revoke(token)}
					disabled={action.busy}
					title={t('graphql.revoke')}
				>
					<Icon name="delete" />
				</button>
			</td>
		{/snippet}
	</Table>
</div>

<style lang="scss">
	.actions {
		text-align: right;
	}

	.badge {
		margin-left: 0.5rem;
	}
</style>
