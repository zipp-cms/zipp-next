<script module lang="ts">
	/** Anyone may open the login. */
	export const isPublic = true;
</script>

<script lang="ts">
	import { session } from '../core/session.svelte';
	import { router } from '../core/router.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import AuthLayout from './AuthLayout.svelte';

	/** The login, a link to a new password if it is forgotten. */
	let email = $state('');
	let password = $state('');
	let remember = $state(true);
	let showPassword = $state(false);
	let error = $state('');
	let busy = $state(false);

	async function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		busy = true;
		try {
			await session.login(email, password, remember);
			// back to the page that wanted the login
			const url = new URLSearchParams(location.search).get('url');
			const back =
				url?.startsWith('/admin') && !url.startsWith('/admin/login');
			router.open(back ? url! : '/admin', {}, { history: 'replace' });
		} catch (e: any) {
			error = e.message;
		} finally {
			busy = false;
		}
	}
</script>

<AuthLayout>
	<form {onsubmit}>
		<div class="intro">
			<h1>Log in</h1>
			<p class="muted">Welcome back. Log in with your account.</p>
		</div>

		<label class="stacked">
			Email
			<input
				type="email"
				autocomplete="username"
				placeholder="name@example.com"
				bind:value={email}
				required
			/>
		</label>

		<label class="stacked">
			Password
			<span class="password">
				<input
					type={showPassword ? 'text' : 'password'}
					autocomplete="current-password"
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

		<label class="check">
			<input type="checkbox" bind:checked={remember} />
			Stay logged in
		</label>

		{#if error}<p class="error">{error}</p>{/if}

		<button class="primary submit" disabled={busy}>
			Log in
			<Icon name="arrow-right" />
		</button>

		<p class="note">
			<a href="/admin/forgot">Forgot your password?</a>
		</p>
	</form>
</AuthLayout>
