<script module lang="ts">
	import { listUsers } from './api';

	export async function loadProps() {
		return { users: await listUsers() };
	}
</script>

<script lang="ts">
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Table from 'zipp-ui/Table.svelte';
	import type { Schema } from '../core/schema';
	import type { Account } from './api';
	import { displayName } from './api';
	import { roleNameOf } from '../core/access';
	import { session } from '../core/session.svelte';
	import { go } from '../core/router.svelte';
	import { userPath } from '../core/paths';
	import { time } from '../core/format';
	import { t } from '../i18n/index.svelte';

	/**
	 * The users of the studio, for admins: who has which role and logged
	 * in when; a click opens the account. The roles come from the code, a
	 * user without one is an admin.
	 */
	let { schema, users }: { schema: Schema; users: Account[] } = $props();
</script>

<PageTop title={t('users.title')}>
	{#snippet actions()}
		<button class="primary" onclick={() => go(userPath('new'))}>
			<Icon name="add" />
			{t('users.new')}
		</button>
	{/snippet}
</PageTop>

<div class="card">
	<Table
		rows={users}
		key={u => u.id}
		columns={[t('users.user'), t('users.role'), t('users.lastLogin')]}
		href={u => userPath(u.id)}
	>
		{#snippet row(u)}
			<td>
				<a href={userPath(u.id)}>{displayName(u)}</a>
				{#if u.name}<span class="note">{u.email}</span>{/if}
				{#if u.id === session.user?.id}
					<span class="muted">{t('users.you')}</span>
				{/if}
				{#if u.invited}<span class="badge">
						{t('users.invited')}
					</span>{/if}
			</td>
			<td>{roleNameOf(schema, u.role)}</td>
			<td class="muted">
				{u.lastLoginAt ? time(u.lastLoginAt) : t('common.never')}
			</td>
		{/snippet}
	</Table>
</div>

<style lang="scss">
	a {
		font-weight: 500;
		text-decoration: none;
	}

	.note {
		margin-left: 0.5rem;
	}
</style>
