<script lang="ts">
	import { getSchema, logout, token, type Schema } from './api';
	import {
		canAi,
		hasAgent,
		canAssets,
		isAdmin,
		roleName,
		visibleNav,
	} from './lib/access';
	import { go, route, setSite, shown, site, toLogin } from './route.svelte';
	import Login from './pages/Login.svelte';
	import List from './pages/List.svelte';
	import Edit from './pages/edit/Edit.svelte';
	import Assets from './pages/Assets.svelte';
	import Ai from './pages/Ai.svelte';
	import Doc from './pages/Doc.svelte';
	import Users from './pages/Users.svelte';
	import Setup from './pages/setup/Setup.svelte';
	import Icon from './lib/Icon.svelte';
	import FlyoutHost from './lib/FlyoutHost.svelte';

	let schema = $state<Schema | null>(null);
	let error = $state('');

	const loggedIn = $derived(route.current.page !== 'login' && !!token());
	// only the sections the role may see
	const groups = $derived(schema ? visibleNav(schema) : []);
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

	// a group of only globals is settings, others are content
	function groupIcon(sections: string[]) {
		const all = sections.map(h =>
			schema?.sections.find(s => s.handle === h),
		);
		return all.every(s => s?.kind === 'single') ? 'globe' : 'entries';
	}

	const SETUP_TABS = [
		['sections', 'Sections'],
		['types', 'Entry types'],
		['blocks', 'Blocks'],
		['sites', 'Sites and roles'],
	];

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
	{@const page = route.current.page}
	{@const other = ['assets', 'ai', 'users', 'setup'].includes(page)}
	{@const siteDef = schema.sites.find(s => s.handle === site.current)}
	<div class="layout" class:wide={page === 'ai'}>
		<aside class="side">
			<!-- the site the studio edits, and where it runs -->
			<label class="site-switch" title="Site">
				<span class="mark">{(siteDef?.name ?? '?')[0]}</span>
				<span class="site-name">
					<strong>{siteDef?.name}</strong>
					<small>
						{schema.dev
							? 'Development'
							: 'Production'}{schema.offline.includes(
							site.current,
						)
							? ' · offline'
							: ''}
					</small>
				</span>
				{#if schema.sites.length > 1}
					<Icon name="chevrons-up-down" />
					<select
						value={site.current}
						onchange={e => setSite(e.currentTarget.value)}
					>
						{#each schema.sites as s (s.handle)}
							<option value={s.handle}>
								{s.name}{schema.offline.includes(s.handle)
									? ' (offline)'
									: ''}
							</option>
						{/each}
					</select>
				{/if}
			</label>

			<nav>
				{#each groups as g (g.name)}
					{@const open = !other && g === group}
					<a
						class="item"
						href={navHref(g.sections[0])}
						class:active={open}
					>
						<Icon name={groupIcon(g.sections)} />
						{g.name}
					</a>
					{#if open}
						<div class="sub">
							{#each groupItems as item (item.handle)}
								<a
									href={navHref(item.handle)}
									class:active={shown.section ===
										item.handle ||
										(page === 'doc' &&
											route.current.page === 'doc' &&
											route.current.handle ===
												item.handle)}
								>
									{item.name}
								</a>
							{/each}
						</div>
					{/if}
				{/each}
				{#if canAssets(schema, 'view')}
					<a
						class="item"
						href="/admin/assets"
						class:active={page === 'assets'}
					>
						<Icon name="dashboard" />
						Assets
					</a>
				{/if}
				{#if (schema.aiModels.length || hasAgent(schema)) && canAi(schema)}
					<a
						class="item"
						href="/admin/ai"
						class:active={page === 'ai'}
					>
						<Icon name="sparkles" />
						AI
					</a>
				{/if}
				<hr />
				<!-- read only for everyone, a role sees its part -->
				<a
					class="item"
					href="/admin/setup/sections"
					class:active={page === 'setup'}
				>
					<Icon name="settings" />
					Setup
				</a>
				{#if page === 'setup'}
					<div class="sub">
						{#each SETUP_TABS as [tab, name] (tab)}
							<a
								href="/admin/setup/{tab}"
								class:active={route.current.page === 'setup' &&
									route.current.tab === tab}
							>
								{name}
							</a>
						{/each}
					</div>
				{/if}
				{#if isAdmin(schema)}
					<a
						class="item"
						href="/admin/users"
						class:active={page === 'users'}
					>
						<Icon name="users" />
						Users
					</a>
				{/if}
			</nav>

			<div class="me">
				<a href="/admin/setup/sites#roles" title="What you may do">
					<span class="avatar">
						{schema.me.email[0].toUpperCase()}
					</span>
					<span class="who">
						<strong>{schema.me.email}</strong>
						<small>{roleName(schema)}</small>
					</span>
				</a>
				<button class="icon" onclick={onLogout} title="Log out">
					<Icon name="logout" />
				</button>
			</div>
		</aside>
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

	.side {
		position: sticky;
		top: 0;
		display: flex;
		flex-direction: column;
		gap: 12px;
		height: 100vh;
		padding: 12px 10px;
		overflow-y: auto;
		background: var(--surface);
		border-right: 1px solid var(--line);
	}

	.site-switch {
		position: relative;
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 8px 10px;
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		color: var(--muted);

		.mark {
			display: grid;
			place-items: center;
			width: 28px;
			height: 28px;
			flex-shrink: 0;
			border-radius: var(--radius);
			background: var(--accent);
			color: white;
			font-weight: 700;
			font-size: var(--fs-md);
		}

		.site-name {
			display: flex;
			flex-direction: column;
			margin-right: auto;
			min-width: 0;

			strong {
				color: var(--text);
				font-size: var(--fs-md);
			}

			small {
				font-size: var(--fs-xs);
			}
		}

		// the whole box opens the choice
		select {
			position: absolute;
			inset: 0;
			opacity: 0;
			cursor: pointer;
		}
	}

	nav {
		display: flex;
		flex-direction: column;
		gap: 2px;

		hr {
			width: 100%;
			margin: 8px 0;
			border: none;
			border-top: 1px solid var(--line);
		}
	}

	.item {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 7px 10px;
		border-radius: var(--radius);
		color: var(--text-2);
		font-weight: 500;
		text-decoration: none;

		&:hover {
			color: var(--text);
			background: var(--hover);
		}

		&.active {
			color: var(--accent);
			background: var(--accent-soft);
		}
	}

	.sub {
		display: flex;
		flex-direction: column;
		gap: 1px;
		margin: 2px 0 6px 18px;
		padding-left: 8px;

		a {
			padding: 5px 10px;
			border-radius: var(--radius);
			color: var(--muted);
			font-size: var(--fs-md);
			text-decoration: none;

			&:hover {
				color: var(--text);
				background: var(--hover);
			}

			&.active {
				color: var(--text);
				font-weight: 600;
				background: var(--hover);
			}
		}
	}

	.me {
		display: flex;
		align-items: center;
		gap: 4px;
		margin-top: auto;
		padding-top: 12px;
		border-top: 1px solid var(--line);

		a {
			display: flex;
			align-items: center;
			gap: 10px;
			flex: 1;
			min-width: 0;
			padding: 4px 6px;
			border-radius: var(--radius);
			color: inherit;
			text-decoration: none;

			&:hover {
				background: var(--hover);
			}
		}

		.avatar {
			display: grid;
			place-items: center;
			width: 28px;
			height: 28px;
			flex-shrink: 0;
			border-radius: 50%;
			background: var(--accent-soft);
			color: var(--accent);
			font-size: var(--fs-sm);
			font-weight: 700;
		}

		.who {
			display: flex;
			flex-direction: column;
			min-width: 0;

			strong {
				overflow: hidden;
				text-overflow: ellipsis;
				white-space: nowrap;
				font-size: var(--fs-sm);
				font-weight: 600;
			}

			small {
				color: var(--muted);
				font-size: var(--fs-xs);
			}
		}
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

		.side {
			position: static;
			height: auto;
		}

		nav {
			flex-direction: row;
			flex-wrap: wrap;

			.sub,
			hr {
				display: none;
			}
		}

		.me {
			margin-top: 0;
		}

		main {
			padding: 0 16px 24px;
		}
	}
</style>
