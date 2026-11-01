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

	/** The names of some handles, or all. */
	function names(
		handles: string[] | null,
		all: { handle: string; name: string }[],
	) {
		if (handles === null) return 'All';
		if (!handles.length) return 'None';
		return handles
			.map(h => all.find(x => x.handle === h)?.name ?? h)
			.join(', ');
	}

	async function revoke(t: GraphqlToken) {
		const what = `Revoke ${t.name}? Frontends that send it get no content any more.`;
		if (!confirm(what)) return;
		const done = await action.run(() => deleteToken(t.id));
		if (done !== undefined) tokens = tokens.filter(x => x.id !== t.id);
	}
</script>

<PageTop title="GraphQL">
	{#snippet actions()}
		<button onclick={openGraphiql} title="GraphiQL with your session">
			<Icon name="external" />
			Try it
		</button>
		<button class="primary" onclick={() => go('/admin/graphql/new')}>
			<Icon name="add" />
			New token
		</button>
	{/snippet}
</PageTop>

{#if action.error}<p class="error">{action.error}</p>{/if}

<div class="card">
	<p class="muted">
		Frontends send a token as <code>Authorization: Bearer …</code>
		to
		<code>/api/graphql</code>
		. Each reads only its sections and sites.
		{#if envToken}
			The token of <code>ZIPP_GRAPHQL_TOKEN</code>
			reads everything.
		{/if}
	</p>
	<Table
		rows={tokens}
		key={t => t.id}
		columns={['Token', 'Sections', 'Sites', 'Expires', 'Last used', '']}
		empty="No tokens yet"
	>
		{#snippet row(t)}
			<td>
				<strong>{t.name}</strong>
				{#if t.previews}<span class="badge">Drafts</span>{/if}
			</td>
			<td>{names(t.sections, schema.sections)}</td>
			<td>{names(t.sites, schema.sites)}</td>
			<td class="muted">
				{#if t.expiresAt === null}
					Never
				{:else if t.expiresAt < Date.now()}
					<span class="badge">Expired</span>
				{:else}
					{date(t.expiresAt)}
				{/if}
			</td>
			<td class="muted">{t.lastUsedAt ? time(t.lastUsedAt) : 'Never'}</td>
			<td class="actions">
				<button
					class="icon"
					onclick={() => revoke(t)}
					disabled={action.busy}
					title="Revoke the token"
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
