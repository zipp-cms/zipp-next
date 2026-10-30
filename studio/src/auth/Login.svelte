<script module lang="ts">
	/** Anyone may open the login. */
	export const isPublic = true;
</script>

<script lang="ts">
	import { session } from '../core/session.svelte';
	import { router } from '../core/router.svelte';
	import logo from '../logo/full.svg';

	let email = $state('');
	let password = $state('');
	let error = $state('');

	async function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		try {
			await session.login(email, password);
			// back to the page that wanted the login
			const url = new URLSearchParams(location.search).get('url');
			const back =
				url?.startsWith('/admin') && !url.startsWith('/admin/login');
			router.open(back ? url! : '/admin', {}, { history: 'replace' });
		} catch (e: any) {
			error = e.message;
		}
	}
</script>

<div class="login">
	<form class="card" {onsubmit}>
		<img class="logo" src={logo} alt="zipp" />
		<h1>Log in</h1>
		<label>
			Email
			<input
				type="email"
				placeholder="you@example.com"
				bind:value={email}
				required
			/>
		</label>
		<label>
			Password
			<input type="password" bind:value={password} required />
		</label>
		<button class="primary">Log in</button>
		{#if error}<p class="error">{error}</p>{/if}
	</form>
</div>

<style>
	.login {
		position: fixed;
		inset: 0;
		display: grid;
		place-items: center;
		padding: 16px;
		background: var(--bg);
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 16px;
		width: min(360px, 100%);
		padding: 28px;
	}

	.logo {
		width: 120px;
		margin: -8px 0 -4px -6px;
	}

	label {
		display: flex;
		flex-direction: column;
		gap: 5px;
		font-size: var(--fs-sm);
		font-weight: 600;
	}

	button {
		justify-content: center;
		padding: 8px 12px;
	}
</style>
