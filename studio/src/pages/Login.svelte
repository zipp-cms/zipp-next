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

<form {onsubmit}>
	<h1>zipp studio</h1>
	<input type="email" placeholder="Email" bind:value={email} required />
	<input
		type="password"
		placeholder="Password"
		bind:value={password}
		required
	/>
	<button class="primary">Log in</button>
	{#if error}<p class="error">{error}</p>{/if}
</form>

<style>
	form {
		max-width: 20rem;
		margin: 20vh auto;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}
</style>
