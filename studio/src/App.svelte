<script lang="ts">
	import { getSchema, logout, token, type Schema } from './api';
	import { go, route, setSite, site } from './route.svelte';
	import Login from './pages/Login.svelte';
	import List from './pages/List.svelte';
	import Edit from './pages/Edit.svelte';

	let schema = $state<Schema | null>(null);
	let error = $state('');

	const loggedIn = $derived(route.current.page !== 'login' && !!token());
	const current = $derived.by(() => {
		const r = route.current;
		if (r.page === 'list') return r.section ?? schema?.sections[0]?.handle;
		if (r.page === 'new') return r.section;
		return null;
	});

	$effect(() => {
		if (!loggedIn) {
			if (route.current.page !== 'login') go('#/login');
			return;
		}
		if (schema) return;
		getSchema()
			.then(s => {
				if (!s.sites.some(x => x.handle === site.current)) {
					setSite(s.sites[0].handle);
				}
				schema = s;
			})
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
			{#each schema.sections as s (s.handle)}
				<a href="#/list/{s.handle}" class:active={current === s.handle}>
					{s.name}
				</a>
			{/each}
			<div class="bottom">
				{#if schema.sites.length > 1}
					<select
						value={site.current}
						onchange={e => setSite(e.currentTarget.value)}
						title="Site"
					>
						{#each schema.sites as s (s.handle)}
							<option value={s.handle}>{s.name}</option>
						{/each}
					</select>
				{/if}
				<button class="link" onclick={onLogout}>Log out</button>
			</div>
		</nav>
		<main>
			{#key site.current}
				{#if route.current.page === 'list' && current}
					{#key current}
						<List {schema} section={current} />
					{/key}
				{:else if route.current.page === 'new'}
					{#key route.current.section}
						<Edit
							{schema}
							id={null}
							section={route.current.section}
						/>
					{/key}
				{:else if route.current.page === 'entry'}
					{#key route.current.id}
						<Edit {schema} id={route.current.id} section={null} />
					{/key}
				{/if}
			{/key}
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

		.bottom {
			margin-top: auto;
			display: flex;
			flex-direction: column;
			gap: 0.5rem;
		}

		.link {
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

			.bottom {
				margin: 0 0 0 auto;
				flex-direction: row;
				align-items: center;
			}
		}

		main {
			padding: 1rem;
		}
	}
</style>
