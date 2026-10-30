<script lang="ts">
	import type { Schema } from '../core/schema';
	import { route, shown } from '../core/route.svelte';
	import {
		canAi,
		canAssets,
		hasAgent,
		isAdmin,
		visibleNav,
	} from '../core/access';
	import Icon from '../ui/Icon.svelte';
	import SiteSwitch from './SiteSwitch.svelte';
	import UserMenu from './UserMenu.svelte';
	import { SETUP_TABS, groupIcon, groupItems, navHref } from './nav';

	/**
	 * The sidebar: the site, the groups of the schema's navigation with
	 * the sections of the open one, assets, AI, Setup, users, who is
	 * logged in.
	 */
	let { schema, onlogout }: { schema: Schema; onlogout: () => void } =
		$props();

	const page = $derived(route.current.page);
	// the pages that aren't a group of the nav
	const other = $derived(['assets', 'ai', 'users', 'setup'].includes(page));
	// only the sections the role may see
	const groups = $derived(visibleNav(schema));
	// the group of the section on screen
	const group = $derived(
		groups.find(g => g.sections.includes(shown.section)) ?? groups[0],
	);
	const items = $derived(groupItems(schema, group));

	function active(handle: string) {
		const r = route.current;
		return (
			shown.section === handle ||
			(r.page === 'doc' && r.handle === handle)
		);
	}
</script>

<aside class="side">
	<SiteSwitch {schema} />

	<nav>
		{#each groups as g (g.name)}
			{@const open = !other && g === group}
			<a
				class="item"
				href={navHref(schema, g.sections[0])}
				class:active={open}
			>
				<Icon name={groupIcon(schema, g)} />
				{g.name}
			</a>
			{#if open}
				<div class="sub">
					{#each items as item (item.handle)}
						<a
							href={navHref(schema, item.handle)}
							class:active={active(item.handle)}
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
			<a class="item" href="/admin/ai" class:active={page === 'ai'}>
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
		{#if route.current.page === 'setup'}
			{@const tab = route.current.tab}
			<div class="sub">
				{#each SETUP_TABS as [handle, name] (handle)}
					<a
						href="/admin/setup/{handle}"
						class:active={tab === handle}
					>
						{name}
					</a>
				{/each}
			</div>
		{/if}
		{#if isAdmin(schema)}
			<a class="item" href="/admin/users" class:active={page === 'users'}>
				<Icon name="users" />
				Users
			</a>
		{/if}
	</nav>

	<UserMenu {schema} {onlogout} />
</aside>

<style lang="scss">
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

	nav {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	hr {
		width: 100%;
		margin: 8px 0;
		border: none;
		border-top: 1px solid var(--line);
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

	@media (max-width: 800px) {
		.side {
			position: static;
			height: auto;
		}

		nav {
			flex-direction: row;
			flex-wrap: wrap;
		}

		.sub,
		hr {
			display: none;
		}
	}
</style>
