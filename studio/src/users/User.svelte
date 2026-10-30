<script module lang="ts">
	import { getUser } from './api';

	/** A user's account, or a new user (`id` is `new`). */
	export async function loadProps(props: { id: string }) {
		return {
			account:
				props.id === 'new' ? null : await getUser(Number(props.id)),
		};
	}
</script>

<script lang="ts">
	import { untrack } from 'svelte';
	import type { Schema } from '../core/schema';
	import type { Account } from './api';
	import { displayName } from './api';
	import { UserForm } from './userForm.svelte';
	import RoleAccess from './RoleAccess.svelte';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { go } from '../core/router.svelte';
	import { isAdmin, roleNameOf } from '../core/access';
	import { time } from '../core/format';

	/**
	 * A user's account: name and email, the role and what it may do, a new
	 * password, the sessions. Admins see everyone, a user their own (from
	 * the user menu), without changing the role. Admins invite a new user
	 * without a password, send or copy a reset link and log in as a user
	 * who isn't an admin.
	 */
	let { schema, account }: { schema: Schema; account: Account | null } =
		$props();

	// a new user gets the first role of the code, not admin
	const form = new UserForm(
		untrack(() => account),
		untrack(() => schema.roles[0]?.handle ?? null),
	);
	const admin = $derived(isAdmin(schema));
	const isNew = $derived(form.account === null);
	const title = $derived(
		form.account ? displayName(form.account) : 'New user',
	);
	const crumbs = $derived(
		admin
			? [{ label: 'Users', href: '/admin/users' }, { label: title }]
			: [{ label: 'Your account' }],
	);

	async function save(e?: SubmitEvent) {
		e?.preventDefault();
		const created = await form.save();
		if (created !== null) go(`/admin/users/${created}`, true);
	}

	async function remove() {
		if (!confirm(`Delete ${title}? They can't log in any more.`)) return;
		if (await form.remove()) go('/admin/users');
	}

	function endSessions() {
		const what = form.own
			? 'Log out in all other browsers?'
			: `Log ${title} out everywhere?`;
		if (confirm(what)) form.endSessions();
	}

	function onkeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 's') {
			e.preventDefault();
			save();
		}
	}

	// of the name's first two words, else the email's first letter
	const initials = $derived(
		(form.name.trim()
			? form.name.trim().split(/\s+/).slice(0, 2)
			: [form.email || '?']
		)
			.map(w => w[0].toUpperCase())
			.join(''),
	);
</script>

<svelte:window {onkeydown} />

<PageTop {crumbs}>
	{#snippet state()}
		{#if form.account}
			<span class="badge">{roleNameOf(schema, form.account.role)}</span>
			{#if form.own}<span class="muted">That's you</span>{/if}
		{/if}
	{/snippet}
	{#snippet actions()}
		{#if admin && form.account && !form.own}
			<button class="icon" onclick={remove} title="Delete the user">
				<Icon name="delete" />
			</button>
		{/if}
		<button
			class="primary"
			form="account"
			disabled={form.busy || (!isNew && !form.changed)}
		>
			{isNew ? 'Create' : 'Save'}
		</button>
	{/snippet}
</PageTop>

{#if form.error && !Object.keys(form.fields).length}
	<p class="error">{form.error}</p>
{/if}

<div class="with-aside">
	<form id="account" class="cards" onsubmit={save}>
		<div class="card">
			<h2>Account</h2>
			<label class="stacked">
				Name
				<input bind:value={form.name} placeholder="Anna Muster" />
			</label>
			<label class="stacked">
				Email
				<input
					type="email"
					autocomplete="off"
					bind:value={form.email}
					required
				/>
				{#if form.fields.email}
					<span class="error">{form.fields.email}</span>
				{/if}
			</label>
		</div>

		<div class="card">
			<h2>Role</h2>
			{#if admin && !form.own}
				<select bind:value={form.role}>
					<option value={null}>Admin</option>
					{#each schema.roles as r (r.handle)}
						<option value={r.handle}>{r.name}</option>
					{/each}
				</select>
			{:else}
				<p><strong>{roleNameOf(schema, form.role)}</strong></p>
			{/if}
			<RoleAccess {schema} role={form.role} />
		</div>

		<div class="card">
			<h2>Password</h2>
			{#if isNew}
				<p class="muted">
					Without one the user gets an invite link to choose it.
				</p>
			{:else}
				<p class="muted">
					Leave it empty to keep the current one. A new password logs {form.own
						? 'you'
						: 'them'} out in all
					{form.own ? 'other ' : ''}browsers.
				</p>
			{/if}
			<label class="stacked">
				{isNew ? 'Password' : 'New password'}
				<input
					type="password"
					autocomplete="new-password"
					placeholder="At least 8 characters"
					bind:value={form.password}
				/>
				{#if form.fields.password}
					<span class="error">{form.fields.password}</span>
				{/if}
			</label>
			{#if isNew && !form.password}
				<label class="check">
					<input type="checkbox" bind:checked={form.sendInvite} />
					Send the invite by mail
				</label>
			{/if}
			{#if form.own && form.password}
				<label class="stacked">
					Your current password
					<input
						type="password"
						autocomplete="current-password"
						bind:value={form.currentPassword}
						required
					/>
					{#if form.fields.currentPassword}
						<span class="error">{form.fields.currentPassword}</span>
					{/if}
				</label>
			{/if}
		</div>
	</form>

	<aside class="card">
		<div class="who">
			<span class="avatar">{initials}</span>
			<div class="truncate">
				<strong class="truncate">
					{form.name || form.email || 'New user'}
				</strong>
				{#if form.name}<span class="note truncate">
						{form.email}
					</span>{/if}
			</div>
		</div>
		{#if form.account}
			<dl>
				<dt>Created</dt>
				<dd>{time(form.account.createdAt)}</dd>
				<dt>Last login</dt>
				<dd>
					{form.account.lastLoginAt
						? time(form.account.lastLoginAt)
						: 'Never'}
				</dd>
				<dt>Logged in</dt>
				<dd>
					{form.account.sessions === 0
						? 'Not logged in'
						: form.account.sessions === 1
							? 'in 1 browser'
							: `in ${form.account.sessions} browsers`}
				</dd>
			</dl>
			{#if form.account.sessions > (form.own ? 1 : 0)}
				<button onclick={endSessions} disabled={form.busy}>
					<Icon name="logout" />
					{form.own
						? 'Log out everywhere else'
						: 'Log out everywhere'}
				</button>
			{/if}
			{#if admin && !form.own}
				<div class="access">
					<h3>Access</h3>
					{#if form.account.invited}
						<p class="note">Invited, no password yet.</p>
					{/if}
					<div class="buttons">
						<button
							onclick={() => form.link(true)}
							disabled={form.busy}
						>
							<Icon name="mail" />
							{form.account.invited
								? 'Send the invite again'
								: 'Send a password reset'}
						</button>
						<button
							class="light"
							onclick={() => form.link(false)}
							disabled={form.busy}
						>
							<Icon name="copy-url" />
							{form.account.invited
								? 'Copy the invite link'
								: 'Copy a reset link'}
						</button>
						{#if form.account.role !== null}
							<button
								onclick={() => form.loginAs(false)}
								disabled={form.busy}
							>
								<Icon name="users" />
								Log in as {title}
							</button>
							<button
								class="light"
								onclick={() => form.loginAs(true)}
								disabled={form.busy}
								title="To open in a private window, your login stays"
							>
								<Icon name="copy-url" />
								Copy a login link
							</button>
						{/if}
					</div>
					{#if form.linkNote}
						<p class="note" role="status">{form.linkNote}</p>
					{/if}
				</div>
			{/if}
		{/if}
	</aside>
</div>

<style lang="scss">
	.card {
		display: flex;
		flex-direction: column;
		gap: 0.875rem;

		h2 {
			margin: 0;
		}

		p {
			margin: 0;
		}
	}

	select {
		width: auto;
		align-self: flex-start;
	}

	aside {
		position: sticky;
		top: 4rem;
	}

	.who {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		min-width: 0;

		div {
			display: flex;
			flex-direction: column;
		}
	}

	.avatar {
		display: grid;
		place-items: center;
		width: 2.75rem;
		height: 2.75rem;
		flex-shrink: 0;
		border-radius: 50%;
		background: var(--accent-soft);
		color: var(--accent);
		font-weight: 600;
	}

	dl {
		display: grid;
		grid-template-columns: max-content 1fr;
		gap: 0.375rem 1rem;
		margin: 0;
		font-size: var(--fs-md);
	}

	dt {
		color: var(--muted);
	}

	dd {
		margin: 0;
	}

	aside button {
		align-self: flex-start;
	}

	.access {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding-top: 0.875rem;
		border-top: 1px solid var(--line);

		h3 {
			margin: 0;
			font-size: var(--fs-sm);
			font-weight: 600;
		}

		p {
			margin: 0;
		}
	}

	.buttons {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 0.375rem;
	}
</style>
