<script lang="ts">
	import { getSchema, logout, navGroups, token, type Schema } from './api';
	import { go, route, setSite, shown, site, toLogin } from './route.svelte';
	import Login from './pages/Login.svelte';
	import List from './pages/List.svelte';
	import Edit from './pages/edit/Edit.svelte';
	import Assets from './pages/Assets.svelte';
	import Ai from './pages/Ai.svelte';
	import Doc from './pages/Doc.svelte';
	import Icon from './lib/Icon.svelte';

	let schema = $state<Schema | null>(null);
	let error = $state('');

	const loggedIn = $derived(route.current.page !== 'login' && !!token());
	const groups = $derived(schema ? navGroups(schema) : []);
	// the group of the section on screen
	const group = $derived(
		groups.find(g => g.sections.includes(shown.section)) ?? groups[0],
	);
	// the sections, docs and pages of the group, in the order of the nav
	const groupItems = $derived(
		(group?.sections ?? []).map(h => {
			const s = schema?.sections.find(s => s.handle === h);
			// the texts of the frontend are a single of their own
			const icon =
				h === 'translations' ? 'translate' : (s?.kind ?? 'doc');
			if (s) return { handle: h, name: s.name, icon };
			const d = schema?.docs.find(d => d.handle === h);
			return { handle: h, name: d?.name ?? h, icon: 'doc' };
		}),
	);
	const doc = $derived.by(() => {
		const r = route.current;
		return r.page === 'doc'
			? schema?.docs.find(d => d.handle === r.handle)
			: (schema?.docs.find(d => d.handle === current) ?? null);
	});

	/** The page of a section or a doc in the nav. */
	function navHref(handle: string) {
		if (schema?.docs.some(d => d.handle === handle))
			return '/admin/docs/' + handle;
		return '/admin/entries/' + handle;
	}
	const current = $derived.by(() => {
		const r = route.current;
		if (r.page === 'list') return r.section ?? groups[0]?.sections[0];
		if (r.page === 'new') return r.section;
		return null;
	});

	$effect(() => {
		if (!loggedIn) {
			// an old session failed, the next login starts over
			error = '';
			toLogin();
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
		go('/login');
	}
</script>

{#if route.current.page === 'login'}
	<Login />
{:else if error}
	<p class="error">{error}</p>
{:else if schema}
	{@const assets = route.current.page === 'assets'}
	{@const ai = route.current.page === 'ai'}
	<div class="layout" class:wide={ai}>
		<div class="logo">zipp</div>
		<header>
			<nav class="top">
				{#each groups as g (g.name)}
					<a
						href={navHref(g.sections[0])}
						class:active={!assets && !ai && g === group}
					>
						{g.name}
					</a>
				{/each}
				<a href="/admin/assets" class:active={assets}>Assets</a>
				{#if schema.aiModels.length}
					<a href="/admin/ai" class:active={ai}>AI</a>
				{/if}
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
				<a href="/admin/assets" class="active">
					<Icon name="dashboard" />
					Assets
				</a>
			{:else}
				{#each groupItems as item (item.handle)}
					<a
						href={navHref(item.handle)}
						class:active={shown.section === item.handle}
					>
						<Icon name={item.icon} />
						{item.name}
					</a>
				{/each}
			{/if}
		</nav>
		<main>
			{#key site.current}
				{#if assets}
					<Assets />
				{:else if ai}
					<Ai {schema} />
				{:else if doc}
					<Doc {doc} />
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

	// the AI chat and the website take the whole width
	.wide {
		height: 100vh;
		grid-template-rows: 60px minmax(0, 1fr);

		.side {
			display: none;
		}

		main {
			grid-column: 1 / -1;
			padding: 0;
		}
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
