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
	import {
		deleteToken,
		openGraphiql,
		type GraphqlSchema,
		type GraphqlToken,
	} from './api';
	import { names } from './schemas';
	import Sentence from '../i18n/Sentence.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * The tokens frontends send to GraphQL, for admins, like Craft's: the
	 * schema of the code each reads, until when and when it was used, the
	 * token of `serve` first; a new one, revoking one, and GraphiQL to try
	 * queries with the session. Under them the schemas themselves.
	 */
	let {
		schema,
		tokens: loaded,
		schemas,
		envToken,
	}: {
		schema: Schema;
		tokens: GraphqlToken[];
		schemas: GraphqlSchema[];
		envToken: boolean;
	} = $props();

	let tokens = $state(untrack(() => loaded));
	const action = new Busy();

	/** the token of `serve` is a row too, so the list shows what reads */
	const rows = $derived<(GraphqlToken | 'env')[]>(
		envToken ? ['env', ...tokens] : tokens,
	);

	function schemaName(handle: string) {
		return schemas.find(s => s.handle === handle)?.name;
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
		<button
			class="primary"
			onclick={() => go('/admin/graphql/new')}
			disabled={!schemas.length}
			title={schemas.length ? undefined : t('graphql.noSchemas')}
		>
			<Icon name="add" />
			{t('graphql.newToken')}
		</button>
	{/snippet}
</PageTop>

{#if action.error}<p class="error">{action.error}</p>{/if}

<div class="cards">
	<div class="card">
		<p class="muted">
			<Sentence
				key="graphql.intro"
				params={{
					header: 'Authorization: Bearer …',
					path: '/api/graphql',
				}}
			/>
		</p>
		<Table
			{rows}
			key={row => (row === 'env' ? 'env' : row.id)}
			columns={[
				t('graphql.token'),
				t('graphql.schema'),
				t('graphql.expires'),
				t('graphql.lastUsed'),
				'',
			]}
			empty={t('graphql.noTokens')}
		>
			{#snippet row(token)}
				{#if token === 'env'}
					<td>
						<strong>{t('graphql.serverToken')}</strong>
						<code>ZIPP_GRAPHQL_TOKEN</code>
					</td>
					<td>{t('graphql.everything')}</td>
					<td class="muted">{t('common.never')}</td>
					<td class="muted">–</td>
					<td></td>
				{:else}
					<td><strong>{token.name}</strong></td>
					<td>
						{#if schemaName(token.schema)}
							{schemaName(token.schema)}
						{:else}
							<span
								class="badge"
								title={t('graphql.unknownSchemaNote')}
							>
								{t('graphql.unknownSchema', {
									handle: token.schema || '–',
								})}
							</span>
						{/if}
					</td>
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
						{token.lastUsedAt
							? time(token.lastUsedAt)
							: t('common.never')}
					</td>
					<td class="actions">
						<button
							class="icon"
							onclick={() => revoke(token)}
							disabled={action.busy}
							title={t('graphql.revoke')}
							aria-label={t('graphql.revoke')}
						>
							<Icon name="delete" />
						</button>
					</td>
				{/if}
			{/snippet}
		</Table>
	</div>

	<div class="card">
		<h2>{t('graphql.schemas')}</h2>
		<p class="muted">
			<Sentence
				key="graphql.schemasNote"
				params={{ code: 'Schema::graphql_schema' }}
			/>
		</p>
		<Table
			rows={schemas}
			key={s => s.handle}
			columns={[
				t('graphql.schema'),
				t('graphql.sections'),
				t('graphql.sites'),
				t('graphql.assets'),
				t('graphql.users'),
				t('graphql.drafts'),
			]}
			empty={t('graphql.noSchemas')}
		>
			{#snippet row(s)}
				<td>
					{s.name}
					<code>{s.handle}</code>
				</td>
				<td>{names(s.sections, schema.sections)}</td>
				<td>{names(s.sites, schema.sites)}</td>
				<td>{s.assets ? t('common.yes') : '–'}</td>
				<td>{s.users ? t('common.yes') : '–'}</td>
				<td>{s.previews ? t('common.yes') : '–'}</td>
			{/snippet}
		</Table>
	</div>
</div>

<style lang="scss">
	.actions {
		text-align: right;
	}
</style>
