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
	import { onDestroy, untrack } from 'svelte';
	import type { Schema } from '../core/schema';
	import type { Account } from './api';
	import { displayName } from './api';
	import { UserForm } from './userForm.svelte';
	import RoleAccess from './RoleAccess.svelte';
	import UserAccess from './UserAccess.svelte';
	import Avatar from './Avatar.svelte';
	import PasswordInput from '../auth/PasswordInput.svelte';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Checkbox from 'zipp-ui/Checkbox.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Segmented from 'zipp-ui/Segmented.svelte';
	import { go } from '../core/router.svelte';
	import { userPath } from '../core/paths';
	import { guardLeave } from '../core/leave';
	import { isAdmin, roleNameOf } from '../core/access';
	import { browserLang, LANGUAGES, t, type Lang } from '../i18n/index.svelte';

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
	// leaving the page and closing the tab ask while it isn't saved
	onDestroy(guardLeave(() => form.changed));
	const admin = $derived(isAdmin(schema));
	const isNew = $derived(form.account === null);
	const title = $derived(
		form.account ? displayName(form.account) : t('users.new'),
	);
	const crumbs = $derived(
		admin
			? [
					{ label: t('users.title'), href: '/admin/users' },
					{ label: title },
				]
			: [{ label: t('common.yourAccount') }],
	);

	// empty follows the browser, it says which that is
	const languages = $derived([
		{
			value: '',
			label: t('users.browserLanguage', {
				language: LANGUAGES[browserLang()],
			}),
		},
		...(Object.keys(LANGUAGES) as Lang[]).map(l => ({
			value: l,
			label: LANGUAGES[l],
		})),
	]);

	async function save(e?: SubmitEvent) {
		e?.preventDefault();
		const created = await form.save();
		if (created !== null) go(userPath(created), true);
	}

	async function remove() {
		if (!confirm(t('users.confirmDelete', { name: title }))) return;
		if (await form.remove()) go('/admin/users');
	}

	function onkeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 's') {
			e.preventDefault();
			save();
		}
	}
</script>

<svelte:window {onkeydown} />

{#snippet currentPassword()}
	<label class="stacked">
		{t('users.currentPassword')}
		<PasswordInput
			autocomplete="current-password"
			bind:value={form.currentPassword}
		/>
		{#if form.fields.currentPassword}
			<span class="error">{form.fields.currentPassword}</span>
		{/if}
	</label>
{/snippet}

<PageTop {crumbs}>
	{#snippet state()}
		{#if form.account}
			<span class="badge">{roleNameOf(schema, form.account.role)}</span>
			{#if form.own}<span class="muted">{t('users.thatsYou')}</span>{/if}
		{/if}
	{/snippet}
	{#snippet actions()}
		{#if admin && form.account && !form.own}
			<button
				type="button"
				class="icon"
				onclick={remove}
				title={t('users.delete')}
				aria-label={t('users.delete')}
			>
				<Icon name="delete" />
			</button>
		{/if}
		<button
			class="primary"
			form="account"
			disabled={form.busy || (!isNew && !form.changed)}
		>
			{isNew ? t('common.create') : t('common.save')}
		</button>
	{/snippet}
</PageTop>

{#if form.error && !Object.keys(form.fields).length}
	<p class="error">{form.error}</p>
{/if}

<div class="with-aside">
	<form id="account" class="cards" onsubmit={save}>
		<div class="card">
			<h2>{t('users.account')}</h2>
			<label class="stacked">
				{t('common.name')}
				<input
					bind:value={form.name}
					placeholder={t('common.namePlaceholder')}
				/>
			</label>
			<label class="stacked">
				{t('common.email')}
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
			<!-- a new email alone asks for it here, with a password below -->
			{#if form.needsCurrent && !form.password}
				{@render currentPassword()}
			{/if}
			<!-- not a select: the role's is the only one in the form -->
			<div class="language">
				<span>{t('users.language')}</span>
				<Segmented
					options={languages}
					value={form.language}
					onchange={l => (form.language = l)}
					label={t('users.language')}
				/>
			</div>
		</div>

		<div class="card">
			<h2>{t('users.role')}</h2>
			{#if admin && !form.own}
				<select bind:value={form.role} aria-label={t('users.role')}>
					<option value={null}>{t('common.admin')}</option>
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
			<h2>{t('common.password')}</h2>
			{#if isNew}
				<p class="muted">{t('users.passwordNew')}</p>
			{:else}
				<p class="muted">
					{form.own
						? t('users.passwordKeepOwn')
						: t('users.passwordKeep')}
				</p>
			{/if}
			<label class="stacked">
				{isNew ? t('common.password') : t('common.newPassword')}
				<PasswordInput
					autocomplete="new-password"
					placeholder={t('common.passwordPlaceholder')}
					bind:value={form.password}
					required={false}
				/>
				{#if form.fields.password}
					<span class="error">{form.fields.password}</span>
				{/if}
			</label>
			{#if isNew && !form.password}
				<Checkbox bind:checked={form.sendInvite}>
					{t('users.sendInvite')}
				</Checkbox>
			{/if}
			{#if form.needsCurrent && form.password}
				{@render currentPassword()}
			{/if}
		</div>
	</form>

	<aside class="card">
		<div class="who">
			<Avatar user={form} />
			<div class="truncate">
				<strong class="truncate">
					{form.name || form.email || t('users.new')}
				</strong>
				{#if form.name}<span class="note truncate">
						{form.email}
					</span>{/if}
			</div>
		</div>
		{#if form.account}
			<UserAccess {form} account={form.account} {admin} {title} />
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

	.language {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 0.3125rem;

		// small and bold like the labels above
		> span {
			font-size: var(--fs-sm);
			font-weight: 600;
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
</style>
