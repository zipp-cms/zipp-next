<script lang="ts">
	import { tick } from 'svelte';
	import type { Schema } from '../core/schema';
	import { current } from '../core/router.svelte';
	import {
		canAi,
		canAssets,
		canTrash,
		hasAgent,
		isAdmin,
		visibleNav,
	} from '../core/access';
	import { navHref } from '../core/paths';
	import Icon from 'zipp-ui/Icon.svelte';
	import SiteSwitch from './SiteSwitch.svelte';
	import UserMenu from './UserMenu.svelte';
	import { groupIcon, groupItems } from './nav';
	import { setupTabs } from '../setup/tabs';
	import SchemaIcon from '../core/SchemaIcon.svelte';
	import { searchBox } from '../search/search.svelte';
	import { sideNav } from './sideNav.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * The sidebar: the site, the groups of the schema's navigation with
	 * the sections of the open one, assets, AI, Setup, users, GraphQL, the
	 * trash,
	 * who is logged in; the search above them.
	 */
	let { schema, onlogout }: { schema: Schema; onlogout: () => void } =
		$props();

	// the first part of the path: entries, assets, setup…
	const page = $derived(current.path.split('/')[2] ?? '');
	// the pages that aren't a group of the nav
	const other = $derived(
		['assets', 'ai', 'users', 'graphql', 'setup', 'trash'].includes(page),
	);
	// only the sections the role may see
	const groups = $derived(visibleNav(schema));
	// the group of the section on screen
	const group = $derived(
		groups.find(g => g.sections.includes(current.section)) ?? groups[0],
	);
	const items = $derived(groupItems(schema, group));

	const active = (handle: string) => current.section === handle;

	// the key of the shortcut, see Shell
	const mod = /Mac|iPhone|iPad/.test(navigator.userAgent) ? '⌘' : 'Ctrl ';

	// after the drawer is gone, which gives the focus back to its button
	async function search() {
		sideNav.open = false;
		await tick();
		searchBox.open = true;
	}
</script>

<aside class="side">
	<SiteSwitch {schema} />

	<button type="button" class="open-search" onclick={search}>
		<Icon name="search" size={14} />
		{t('ui.search')}
		<kbd>{mod}K</kbd>
	</button>

	<nav>
		{#each groups as g (g.name)}
			{@const open = !other && g === group}
			<a
				class="item nav-item"
				href={navHref(schema, g.sections[0])}
				class:active={open}
			>
				<SchemaIcon icon={g.icon} fallback={groupIcon(schema, g)} />
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
				class="item nav-item"
				href="/admin/assets"
				class:active={page === 'assets'}
			>
				<Icon name="dashboard" />
				{t('shell.assets')}
			</a>
		{/if}
		{#if (schema.aiModels.length || hasAgent(schema)) && canAi(schema)}
			<a
				class="item nav-item"
				href="/admin/ai"
				class:active={page === 'ai'}
			>
				<Icon name="sparkles" />
				{t('shell.ai')}
			</a>
		{/if}
		<hr />
		<!-- read only for everyone, a role sees its part -->
		<a
			class="item nav-item"
			href="/admin/setup"
			class:active={page === 'setup'}
		>
			<Icon name="settings" />
			{t('shell.setup')}
		</a>
		{#if page === 'setup'}
			{@const tab = current.path.split('/')[3] ?? 'overview'}
			<div class="sub">
				{#each setupTabs(schema) as [handle, key] (handle)}
					<a
						href="/admin/setup/{handle}"
						class:active={tab === handle}
					>
						{t(key)}
					</a>
				{/each}
			</div>
		{/if}
		{#if isAdmin(schema)}
			<a
				class="item nav-item"
				href="/admin/users"
				class:active={page === 'users'}
			>
				<Icon name="users" />
				{t('shell.users')}
			</a>
			<a
				class="item nav-item"
				href="/admin/graphql"
				class:active={page === 'graphql'}
			>
				<Icon name="graphql" />
				GraphQL
			</a>
		{/if}
		{#if canTrash(schema)}
			<a
				class="item nav-item"
				href="/admin/trash"
				class:active={page === 'trash'}
			>
				<Icon name="delete" />
				{t('shell.trash')}
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
		gap: 0.75rem;
		height: 100vh;
		padding: 0.75rem 0.625rem;
		overflow-y: auto;
		background: var(--surface);
		border-right: 1px solid var(--line);
	}

	.open-search {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.4375rem 0.625rem;
		border: 1px solid var(--border);
		border-radius: var(--radius);
		background: var(--surface);
		color: var(--subtle);
		font-size: var(--fs-md);
		text-align: left;

		@include hover {
			&:hover {
				color: var(--text-2);
				background: var(--hover);
			}
		}

		kbd {
			margin-left: auto;
			font-family: inherit;
			font-size: var(--fs-xs);
		}

		// a phone has no keys for it
		@include max-tablet {
			kbd {
				display: none;
			}
		}
	}

	nav {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	hr {
		width: 100%;
		margin: 0.5rem 0;
		border: none;
		border-top: 1px solid var(--line);
	}

	.item {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		padding: 0.4375rem 0.625rem;
		font-weight: 500;
		text-decoration: none;
	}

	.sub {
		display: flex;
		flex-direction: column;
		gap: 1px;
		margin: 2px 0 0.375rem 1.125rem;
		padding-left: 0.5rem;

		a {
			padding: 0.3125rem 0.625rem;
			border-radius: var(--radius);
			color: var(--muted);
			font-size: var(--fs-md);
			text-decoration: none;

			@include hover {
				&:hover {
					color: var(--text);
					background: var(--hover);
				}
			}

			&.active {
				color: var(--text);
				font-weight: 600;
				background: var(--hover);
			}
		}
	}
</style>
