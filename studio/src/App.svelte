<script lang="ts">
	import { getSchema, logout, token, type Schema } from './api';
	import { go, route } from './route.svelte';
	import Login from './pages/Login.svelte';
	import List from './pages/List.svelte';
	import Edit from './pages/Edit.svelte';

	let schema = $state<Schema | null>(null);
	let error = $state('');

	const loggedIn = $derived(route.current.page !== 'login' && !!token());
	const entryTypes = $derived(schema?.components.filter(c => c.entry) ?? []);
	const current = $derived.by(() => {
		const r = route.current;
		if (r.page === 'list') return r.type ?? entryTypes[0]?.handle;
		if (r.page === 'new') return r.type;
		return null;
	});

	$effect(() => {
		if (!loggedIn) {
			if (route.current.page !== 'login') go('#/login');
			return;
		}
		if (schema) return;
		getSchema()
			.then(s => (schema = s))
			.catch(e => (error = e.message));
	});

	async function onLogout() {
		await logout();
		schema = null;
		go('#/login');
	}
</script>

{#if route.current.page === 'login'}
	<Login />
{:else if error}
	<p class="error">{error}</p>
{:else if schema}
	<div class="layout">
		<nav>
			<h1>zipp</h1>
			{#each entryTypes as t (t.handle)}
				<a href="#/list/{t.handle}" class:active={current === t.handle}>
					{t.name}
				</a>
			{/each}
			<button class="link" onclick={onLogout}>Log out</button>
		</nav>
		<main>
			{#if route.current.page === 'list' && current}
				{#key current}
					<List {schema} type={current} />
				{/key}
			{:else if route.current.page === 'new'}
				{#key route.current.type}
					<Edit {schema} id={null} type={route.current.type} />
				{/key}
			{:else if route.current.page === 'entry'}
				{#key route.current.id}
					<Edit {schema} id={route.current.id} type={null} />
				{/key}
			{/if}
		</main>
	</div>
{/if}

<style lang="scss">
	.layout {
		display: flex;
		min-height: 100vh;
	}

	nav {
		width: 13rem;
		flex-shrink: 0;
		padding: 1.5rem 1rem;
		background: var(--side);
		display: flex;
		flex-direction: column;
		gap: 0.25rem;

		h1 {
			font-size: 1.3rem;
			margin: 0 0.5rem 1rem;
		}

		a {
			padding: 0.4rem 0.5rem;
			border-radius: 0.3rem;
			color: inherit;
			text-decoration: none;

			&.active {
				background: var(--line);
			}
		}

		.link {
			margin-top: auto;
			text-align: left;
		}
	}

	main {
		flex: 1;
		padding: 1.5rem 2rem;
		max-width: 56rem;
	}

	@media (max-width: 700px) {
		.layout {
			flex-direction: column;
		}

		nav {
			width: auto;
			flex-direction: row;
			flex-wrap: wrap;
			align-items: center;
			padding: 0.6rem;

			h1 {
				margin: 0 0.75rem 0 0.25rem;
			}

			.link {
				margin: 0 0 0 auto;
			}
		}

		main {
			padding: 1rem;
		}
	}
</style>
