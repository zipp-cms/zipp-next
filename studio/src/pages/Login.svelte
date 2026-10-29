<script lang="ts">
	import { login } from '../api';
	import { back } from '../route.svelte';

	let email = $state('');
	let password = $state('');
	let error = $state('');

	async function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		try {
			await login(email, password);
			back();
		} catch (e: any) {
			error = e.message;
		}
	}
</script>

<div class="login">
	<form class="card" {onsubmit}>
		<div class="brand">
			<span class="mark">z</span>
			<strong>zipp studio</strong>
		</div>
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

	.brand {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.mark {
		display: grid;
		place-items: center;
		width: 28px;
		height: 28px;
		border-radius: 6px;
		background: var(--accent);
		color: white;
		font-weight: 700;
	}

	label {
		display: flex;
		flex-direction: column;
		gap: 5px;
		font-size: 12px;
		font-weight: 600;
	}

	button {
		justify-content: center;
		padding: 8px 12px;
	}
</style>
