<script module lang="ts">
	import { request } from '../core/client';

	/** Anyone with the link, it is the key. */
	export const isPublic = true;

	export type LinkInfo = {
		kind: 'invite' | 'reset' | 'loginAs';
		email: string;
		name: string;
		expiresAt: number;
	};

	/** What the link is for; used or expired, the page says so. */
	export async function loadProps(props: { token: string }) {
		try {
			const link = await request<LinkInfo>(
				'GET',
				`/auth/links/${props.token}`,
			);
			return { link, problem: '' };
		} catch (e: any) {
			return { link: null, problem: e.message as string };
		}
	}
</script>

<script lang="ts">
	import Icon from 'zipp-ui/Icon.svelte';
	import { session, type Authenticated } from '../core/session.svelte';
	import { ApiError } from '../core/client';
	import AuthLayout from './AuthLayout.svelte';

	/**
	 * The link of an invite or a password reset: choose a password (an
	 * invited user their name too), then logged in.
	 */
	let {
		token,
		link,
		problem,
	}: { token: string; link: LinkInfo | null; problem: string } = $props();

	const invite = $derived(link?.kind === 'invite');
	let name = $state('');
	let password = $state('');
	let showPassword = $state(false);
	let error = $state('');
	let busy = $state(false);

	async function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		busy = true;
		try {
			const auth = await request<Authenticated>(
				'POST',
				`/auth/links/${token}/password`,
				{ password, name: invite ? name : null },
			);
			session.use(auth);
			location.assign('/admin');
		} catch (e: any) {
			// "At least 8 characters" rather than "Some fields are invalid"
			const field = e instanceof ApiError ? e.fields[0] : undefined;
			error = field?.message ?? e.message;
		} finally {
			busy = false;
		}
	}
</script>

<AuthLayout>
	{#if link}
		<form {onsubmit}>
			<div class="intro">
				<h1>{invite ? 'Welcome' : 'A new password'}</h1>
				<p class="muted">
					{invite
						? `Your account is ${link.email}. Choose a password to log in.`
						: `Choose a new password for ${link.email}. You are logged out everywhere else.`}
				</p>
			</div>

			{#if invite}
				<label class="stacked">
					Your name
					<input
						autocomplete="name"
						placeholder="Anna Muster"
						bind:value={name}
					/>
				</label>
			{/if}

			<label class="stacked">
				Password
				<span class="password">
					<input
						type={showPassword ? 'text' : 'password'}
						autocomplete="new-password"
						placeholder="At least 8 characters"
						bind:value={password}
						required
					/>
					<button
						type="button"
						class="icon"
						title={showPassword
							? 'Hide the password'
							: 'Show the password'}
						onclick={() => (showPassword = !showPassword)}
					>
						<Icon name={showPassword ? 'hide' : 'view'} />
					</button>
				</span>
			</label>

			{#if error}<p class="error">{error}</p>{/if}

			<button class="primary submit" disabled={busy}>
				{invite ? 'Set the password and log in' : 'Save and log in'}
				<Icon name="arrow-right" />
			</button>
		</form>
	{:else}
		<div>
			<div class="intro">
				<h1>This link doesn't work</h1>
				<p class="muted">
					{problem}. Ask for a new one, or an admin for an invite.
				</p>
			</div>
			<p class="note">
				<a href="/admin/forgot">New password</a>
				·
				<a href="/admin/login">Log in</a>
			</p>
		</div>
	{/if}
</AuthLayout>
