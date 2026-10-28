<script lang="ts">
	import { login } from '../api';
	import { go } from '../route.svelte';

	let email = $state('');
	let password = $state('');
	let error = $state('');

	async function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		try {
			await login(email, password);
			go('#/');
		} catch (e: any) {
			error = e.message;
		}
	}
</script>

<div class="login">
	<div class="image"><span class="logo">zipp</span></div>
	<form {onsubmit}>
		<h2>Log in to Zipp</h2>
		<label>
			Email
			<input
				type="email"
				placeholder="Email"
				bind:value={email}
				required
			/>
		</label>
		<label>
			Password
			<input
				type="password"
				placeholder="Password"
				bind:value={password}
				required
			/>
		</label>
		<button class="dark">Log in</button>
		{#if error}<p class="error">{error}</p>{/if}
	</form>
</div>

<style>
	.login {
		position: fixed;
		inset: 0;
		display: flex;
		background: white;
	}

	.image {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 70%;
		background: linear-gradient(135deg, #2c2d35, #5d5f6f);
	}

	.logo {
		color: white;
		font-size: 64px;
		font-weight: 900;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	form {
		display: flex;
		flex-direction: column;
		justify-content: center;
		gap: 20px;
		width: 30%;
		padding: 32px;
	}

	h2 {
		margin-bottom: 20px;
		font-size: 24px;
	}

	label {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	button {
		align-self: flex-start;
	}

	@media (max-width: 800px) {
		.image {
			display: none;
		}

		form {
			width: 100%;
		}
	}
</style>
