<script module lang="ts">
	import type { LoadProps } from '../core/router.svelte';

	/** The schema for the frame and every page in it. */
	export async function loadProps(_props: object, lp: LoadProps) {
		return { schema: await lp.schema() };
	}
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import { session } from '../core/session.svelte';
	import { current, go } from '../core/router.svelte';
	import { endSession } from '../core/router.svelte';
	import type { Schema } from '../core/schema';
	import FlyoutHost from './FlyoutHost.svelte';
	import Sidebar from './Sidebar.svelte';
	import UploadProgress from '../assets/UploadProgress.svelte';
	import LoginAsBar from './LoginAsBar.svelte';
	import { sideNav } from './sideNav.svelte';
	import { layer } from 'zipp-ui/layers';

	/**
	 * The frame of the studio's pages: the sidebar, the page, the flyout
	 * over it. The AI page takes the whole width, its sidebar slides in
	 * over it (`sideNav`) and goes again with the next page.
	 */
	let { schema, children }: { schema: Schema; children: Snippet } = $props();

	const wide = $derived(current.path === '/admin/ai');

	$effect(() => {
		// a new page closes it
		void current.path;
		sideNav.open = false;
	});

	const close = () => (sideNav.open = false);

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

<LoginAsBar />
<!-- the AI chat and the website take the whole page -->
<div class="layout" class:wide>
	{#if !wide}<Sidebar {schema} {onlogout} />{/if}
	<main>{@render children()}</main>
</div>
{#if wide && sideNav.open}
	<div class="backdrop" role="presentation" onclick={close}></div>
	<div class="drawer" use:layer={close}>
		<Sidebar {schema} {onlogout} />
	</div>
{/if}
<FlyoutHost {schema} />
<UploadProgress />

<style lang="scss">
	.layout {
		display: grid;
		grid-template-columns: 14.5rem minmax(0, 1fr);
		min-height: 100vh;
	}

	main {
		min-width: 0;
		padding: 0 var(--page-pad) 2rem;
	}

	.wide {
		grid-template-columns: minmax(0, 1fr);
		height: 100vh;

		main {
			padding: 0;
			height: 100vh;
		}
	}

	.backdrop {
		position: fixed;
		inset: 0;
		z-index: var(--z-overlay);
		background: var(--backdrop);
	}

	.drawer {
		position: fixed;
		top: 0;
		bottom: 0;
		left: 0;
		z-index: var(--z-overlay);
		width: 14.5rem;
		box-shadow: var(--shadow-flyout);
		animation: slide 0.18s ease-out;
	}

	@keyframes slide {
		from {
			transform: translateX(-100%);
		}
	}

	@include max-tablet {
		.layout {
			grid-template-columns: 1fr;
		}
	}
</style>
