<script module lang="ts">
	import { listUsers as list } from './api';

	export async function loadProps() {
		return { first: await list() };
	}
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import PageTop from '../ui/PageTop.svelte';
	import { type Schema } from '../core/schema';
	import { deleteUser, listUsers, saveUser, setRole, type User } from './api';
	import { roleNameOf } from '../core/access';
	import { session } from '../core/session.svelte';
	import { Busy } from '../core/busy.svelte';

	/**
	 * The users of the studio, for admins: who has which role. The roles
	 * come from the code, a user without one is an admin.
	 */
	let { schema, first }: { schema: Schema; first: User[] } = $props();

	let users = $state<User[]>(untrack(() => first));
	const action = new Busy();
	let email = $state('');
	let password = $state('');
	let role = $state<string | null>(null);

	async function load() {
		users = await listUsers();
	}

	/** Changes the users, then shows them again. */
	const run = (f: () => Promise<unknown>) =>
		action.run(async () => {
			await f();
			await load();
		});

	function add(e: SubmitEvent) {
		e.preventDefault();
		return run(async () => {
			await saveUser(email, password, role);
			email = '';
			password = '';
		});
	}
</script>

<PageTop title="Users" />

{#if action.error}<p class="error">{action.error}</p>{/if}

<div class="card">
	<table class="table">
		<tbody>
			{#each users as u (u.id)}
				<tr>
					<td>{u.email}</td>
					<td>
						{#if u.id === session.user?.id}
							{roleNameOf(schema, u.role)}
							<span class="muted">(you)</span>
						{:else}
							<select
								value={u.role ?? ''}
								onchange={e =>
									run(() =>
										setRole(
											u.id,
											e.currentTarget.value || null,
										),
									)}
							>
								<option value="">Admin</option>
								{#each schema.roles as r (r.handle)}
									<option value={r.handle}>{r.name}</option>
								{/each}
							</select>
						{/if}
					</td>
					<td class="actions">
						{#if u.id !== session.user?.id}
							<button
								class="link"
								onclick={() =>
									confirm(`Delete ${u.email}?`) &&
									run(() => deleteUser(u.id))}
							>
								Delete
							</button>
						{/if}
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<form class="card add" onsubmit={add}>
	<h2>Add a user</h2>
	<p class="muted">
		With an email that exists, this sets a new password and role.
	</p>
	<input type="email" placeholder="Email" bind:value={email} required />
	<input
		type="password"
		placeholder="Password, at least 8 characters"
		bind:value={password}
		minlength="8"
		required
	/>
	<select bind:value={role}>
		<option value={null}>Admin</option>
		{#each schema.roles as r (r.handle)}
			<option value={r.handle}>{r.name}</option>
		{/each}
	</select>
	<button class="primary" disabled={action.busy}>Save</button>
</form>

<style>
	select {
		width: auto;
	}

	.actions {
		text-align: right;
	}

	.add {
		display: grid;
		gap: 10px;
		max-width: 420px;
		margin-top: 20px;
	}
</style>
