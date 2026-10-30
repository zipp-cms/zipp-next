<script lang="ts">
	import { getSchema, logout, token, type Schema } from './api';
	import { go, route, setSite, site, toLogin } from './route.svelte';
	import Login from './pages/Login.svelte';
	import List from './pages/List.svelte';
	import Edit from './pages/edit/Edit.svelte';
	import Assets from './pages/Assets.svelte';
	import Ai from './pages/ai/Ai.svelte';
	import Doc from './pages/Doc.svelte';
	import Users from './pages/Users.svelte';
	import Setup from './pages/setup/Setup.svelte';
	import FlyoutHost from './lib/FlyoutHost.svelte';
	import Sidebar from './lib/nav/Sidebar.svelte';
	import { visibleNav } from './lib/access';

	let schema = $state<Schema | null>(null);
	let error = $state('');

	const loggedIn = $derived(route.current.page !== 'login' && !!token());

	// the section of a list, the first one of the nav at the start
	const current = $derived.by(() => {
		const r = route.current;
		if (r.page === 'list')
			return r.section ?? (schema && visibleNav(schema)[0]?.sections[0]);
		return null;
	});
	// a doc of the code: its page, or a doc in the place of a section
	const doc = $derived.by(() => {
		const r = route.current;
		const handle = r.page === 'doc' ? r.handle : current;
		return schema?.docs.find(d => d.handle === handle) ?? null;
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

	async function onlogout() {
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
	{@const page = route.current.page}
	<div class="layout" class:wide={page === 'ai'}>
		<Sidebar {schema} {onlogout} />
		<main>
			{#key site.current}
				{#if page === 'assets'}
					<Assets {schema} />
				{:else if page === 'users'}
					<Users {schema} />
				{:else if route.current.page === 'setup'}
					<Setup
						{schema}
						tab={route.current.tab}
						handle={route.current.handle}
					/>
				{:else if page === 'ai'}
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
	<FlyoutHost {schema} />
{/if}

<style lang="scss">
	.layout {
		display: grid;
		grid-template-columns: 232px minmax(0, 1fr);
		min-height: 100vh;
	}

	main {
		min-width: 0;
		padding: 0 24px 32px;
	}

	// the AI chat and the website take the whole height
	.wide {
		height: 100vh;

		main {
			padding: 0;
			height: 100vh;
		}
	}

	@media (max-width: 800px) {
		.layout {
			grid-template-columns: 1fr;
		}

		main {
			padding: 0 16px 24px;
		}
	}
</style>
