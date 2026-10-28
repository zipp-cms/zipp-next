<script lang="ts">
	import { getSchema, logout, token, type Schema } from './api';
	import { go, route, setSite, site } from './route.svelte';
	import Login from './pages/Login.svelte';
	import List from './pages/List.svelte';
	import Edit from './pages/Edit.svelte';
	import Assets from './pages/Assets.svelte';
	import Icon from './lib/Icon.svelte';

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
	{@const assets = route.current.page === 'assets'}
	<div class="layout">
		<div class="logo">zipp</div>
		<header>
			<nav class="top">
				<a href="#/list" class:active={!assets}>Entries</a>
				<a href="#/assets" class:active={assets}>Assets</a>
			</nav>
			<div class="right">
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
		</header>
		<nav class="side">
			{#if assets}
				<a href="#/assets" class="active">
					<Icon name="dashboard" />
					Assets
				</a>
			{:else}
				{#each schema.sections as s (s.handle)}
					<a
						href="#/list/{s.handle}"
						class:active={current === s.handle}
					>
						<Icon
							name={s.kind === 'single' ? 'default' : 'all_pages'}
						/>
						{s.name}
					</a>
				{/each}
			{/if}
		</nav>
		<main>
			{#key site.current}
				{#if assets}
					<Assets />
				{:else if route.current.page === 'list' && current}
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
		display: grid;
		grid-template-columns: 240px 1fr;
		grid-template-rows: 60px 1fr;
		min-height: 100vh;
	}

	.logo {
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bar);
		color: white;
		font-size: 22px;
		font-weight: 900;
		text-transform: uppercase;
	}

	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0 32px;
		background: var(--bar);
		color: white;
	}

	.top {
		display: flex;
		gap: 32px;

		a {
			color: rgba(255, 255, 255, 0.7);
			text-decoration: none;
			transition: color 0.4s ease;

			&.active,
			&:hover {
				color: white;
			}
		}
	}

	.right {
		display: flex;
		align-items: center;
		gap: 16px;

		select {
			width: auto;
			padding: 6px 10px;
			background: #3a3b44;
			color: white;
		}

		.link {
			color: white;
		}
	}

	.side {
		padding: 16px;
		background: white;

		a {
			display: flex;
			align-items: center;
			gap: 8px;
			margin-bottom: 8px;
			padding: 9px 8px;
			border-radius: 2px;
			color: inherit;
			text-decoration: none;
			transition: background-color 0.4s ease;

			&.active,
			&:hover {
				background: var(--hover);
			}
		}
	}

	main {
		min-width: 0;
		padding: 16px 32px;
	}

	@media (max-width: 800px) {
		.layout {
			grid-template-columns: 1fr;
			grid-template-rows: auto;
			align-content: start;
		}

		.logo {
			justify-content: flex-start;
			padding: 12px 16px 0;
		}

		header {
			flex-wrap: wrap;
			gap: 12px;
			padding: 12px 16px;
		}

		.side {
			display: flex;
			flex-wrap: wrap;
			gap: 4px;
			padding: 8px;

			a {
				margin: 0;
			}
		}

		main {
			padding: 16px;
		}
	}
</style>
