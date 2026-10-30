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
		<label class="stacked">
			Email
			<input
				type="email"
				placeholder="you@example.com"
				bind:value={email}
				required
			/>
		</label>
		<label class="stacked">
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
		padding: 1rem;
		background: var(--bg);
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		width: min(22.5rem, 100%);
		padding: 1.75rem;
	}

	.logo {
		width: 7.5rem;
		margin: -0.5rem 0 -0.25rem -0.375rem;
	}

	button {
		justify-content: center;
		padding: 0.5rem 0.75rem;
	}
</style>
