<script module lang="ts">
	import type { LoadProps } from '../core/router.svelte';

	/** The schema for the frame and every page in it. */
	export async function loadProps(_props: object, lp: LoadProps) {
		return { schema: await lp.schema() };
	}
</script>

<script lang="ts">
	import { tick, type Snippet } from 'svelte';
	import { session } from '../core/session.svelte';
	import { current, go } from '../core/router.svelte';
	import { endSession } from '../core/router.svelte';
	import type { Schema } from '../core/schema';
	import FlyoutHost from './FlyoutHost.svelte';
	import Sidebar from './Sidebar.svelte';
	import UploadProgress from '../assets/UploadProgress.svelte';
	import LoginAsBar from './LoginAsBar.svelte';
	import { sideNav } from './sideNav.svelte';
	import { layer, trapTab } from 'zipp-ui/layers';
	import { providePageLead } from 'zipp-ui/pageLead';
	import Icon from 'zipp-ui/Icon.svelte';
	import SearchBox from '../search/SearchBox.svelte';
	import { searchBox } from '../search/search.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * The frame of the studio's pages: the sidebar, the page, the flyout
	 * over it. The AI page takes the whole width, the live preview hides
	 * the sidebar too (`sideNav`), and a phone has no room for it: then
	 * it slides in over the page from the Menu button before the page's
	 * title and goes again with the next page. The search opens over all
	 * of it.
	 */
	let { schema, children }: { schema: Schema; children: Snippet } = $props();

	const wide = $derived(current.path === '/admin/ai');
	const hidden = $derived(wide || sideNav.hidden);
	providePageLead({ content: menu });

	$effect(() => {
		// a new page closes it
		void current.path;
		sideNav.open = false;
	});

	const close = () => (sideNav.open = false);

	// the drawer is modal: it takes the focus and keeps Tab, `layer` gives
	// the focus back
	let drawer = $state<HTMLElement>();
	$effect(() => drawer?.focus());

	// Cmd+K (Ctrl+K) opens the search on every page, again closes it
	async function onkeydown(e: KeyboardEvent) {
		if (e.key !== 'k' || !(e.metaKey || e.ctrlKey) || e.altKey) return;
		e.preventDefault();
		// the drawer gives the focus back when it goes, before the search
		// takes it
		sideNav.open = false;
		await tick();
		searchBox.open = !searchBox.open;
	}

	async function onlogout() {
		// logged in as someone: back to the own account
		if (session.impersonatedBy) {
			const ok = await session.endLoginAs();
			return location.assign(ok ? '/admin/users' : '/admin/login');
		}
		await session.logout();
		endSession();
	}
</script>

<!-- in every page's top, shown when there is no sidebar -->
{#snippet menu()}
	<button
		type="button"
		class="icon menu"
		class:shown={hidden}
		title={t('shell.menu')}
		aria-label={t('shell.menu')}
		aria-expanded={sideNav.open}
		onclick={() => (sideNav.open = true)}
	>
		<Icon name="sidebar" size={16} />
	</button>
{/snippet}

<svelte:window {onkeydown} />
<LoginAsBar />
<!-- the AI chat and the website take the whole page -->
<div class="layout" class:wide class:hidden>
	{#if !hidden}<Sidebar {schema} {onlogout} />{/if}
	<main>{@render children()}</main>
</div>
{#if sideNav.open}
	<div class="backdrop" role="presentation" onclick={close}></div>
	<div
		class="drawer"
		role="dialog"
		aria-modal="true"
		aria-label={t('shell.menu')}
		tabindex="-1"
		bind:this={drawer}
		use:layer={close}
		onkeydown={e => drawer && trapTab(e, drawer)}
	>
		<Sidebar {schema} {onlogout} />
	</div>
{/if}
<FlyoutHost {schema} />
{#if searchBox.open}<SearchBox {schema} />{/if}
<UploadProgress />

<style lang="scss">
	.layout {
		display: grid;
		grid-template-columns: 14.5rem minmax(0, 1fr);
		min-height: 100vh;
	}

	// a page lays out by its own width, e.g. the entry editor's bar
	main {
		container: page / inline-size;
		min-width: 0;
		padding: 0 var(--page-pad) 2rem;
	}

	.hidden {
		grid-template-columns: minmax(0, 1fr);
	}

	.wide {
		height: 100vh;

		main {
			padding: 0;
			height: 100vh;
		}
	}

	.drawer {
		position: fixed;
		top: 0;
		bottom: 0;
		left: 0;
		z-index: var(--z-overlay);
		width: 14.5rem;
		max-width: 85vw;
		box-shadow: var(--shadow-flyout);
		outline: none;
		animation: slide 0.18s ease-out;
	}

	@keyframes slide {
		from {
			transform: translateX(-100%);
		}
	}

	.menu:not(.shown) {
		display: none;
	}

	// a phone: the sidebar only from the Menu button
	@include max-tablet {
		.layout {
			grid-template-columns: minmax(0, 1fr);

			> :global(.side) {
				display: none;
			}
		}

		.menu:not(.shown) {
			display: inline-flex;
		}
	}
</style>
