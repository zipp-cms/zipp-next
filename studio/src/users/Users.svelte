<script module lang="ts">
	import { listUsers } from './api';

	export async function loadProps() {
		return { users: await listUsers() };
	}
</script>

<script lang="ts">
	import PageTop from '../ui/PageTop.svelte';
	import Icon from '../ui/Icon.svelte';
	import type { Schema } from '../core/schema';
	import type { Account } from './api';
	import { displayName } from './api';
	import { roleNameOf } from '../core/access';
	import { session } from '../core/session.svelte';
	import { go } from '../core/router.svelte';
	import { time } from '../core/format';

	/**
	 * The users of the studio, for admins: who has which role and logged
	 * in when; a click opens the account. The roles come from the code, a
	 * user without one is an admin.
	 */
	let { schema, users }: { schema: Schema; users: Account[] } = $props();
</script>

<PageTop title="Users">
	{#snippet actions()}
		<button class="primary" onclick={() => go('/admin/users/new')}>
			<Icon name="add" />
			New user
		</button>
	{/snippet}
</PageTop>

<div class="card">
	<table class="table">
		<thead>
			<tr>
				<th>User</th>
				<th>Role</th>
				<th>Last login</th>
			</tr>
		</thead>
		<tbody>
			{#each users as u (u.id)}
				<tr>
					<td>
						<a href="/admin/users/{u.id}">{displayName(u)}</a>
						{#if u.name}<span class="note">{u.email}</span>{/if}
						{#if u.id === session.user?.id}
							<span class="muted">(you)</span>
						{/if}
					</td>
					<td>{roleNameOf(schema, u.role)}</td>
					<td class="muted">
						{u.lastLoginAt ? time(u.lastLoginAt) : 'Never'}
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
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
