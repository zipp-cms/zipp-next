<script lang="ts">
	import {
		deleteUser,
		listUsers,
		saveUser,
		setRole,
		type Schema,
		type User,
	} from '../api';
	import { shown } from '../route.svelte';

	/**
	 * The users of the studio, for admins: who has which role. The roles
	 * come from the code, a user without one is an admin.
	 */
	let { schema }: { schema: Schema } = $props();

	let users = $state<User[]>([]);
	let error = $state('');
	let email = $state('');
	let password = $state('');
	let role = $state<string | null>(null);

	$effect(() => {
		shown.section = '';
	});

	async function load() {
		users = await listUsers();
	}
	load().catch(e => (error = e.message));

	async function run(f: () => Promise<unknown>) {
		error = '';
		try {
			await f();
			await load();
		} catch (e: any) {
			error = e.message;
		}
	}

	function add(e: SubmitEvent) {
		e.preventDefault();
		return run(async () => {
			await saveUser(email, password, role);
			email = '';
			password = '';
		});
	}

	const roleName = (h: string | null) =>
		h === null
			? 'Admin'
			: (schema.roles.find(r => r.handle === h)?.name ?? h);
</script>

<div class="page-top">
	<h1>Users</h1>
</div>

{#if error}<p class="error">{error}</p>{/if}

<div class="card">
	<table class="table">
		<tbody>
			{#each users as u (u.id)}
				<tr>
					<td>{u.email}</td>
					<td>
						{#if u.id === schema.me.id}
							{roleName(u.role)}
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
						{#if u.id !== schema.me.id}
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
	<button class="primary">Save</button>
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
