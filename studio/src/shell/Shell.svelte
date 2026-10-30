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
	import { current, go } from '../core/route.svelte';
	import { endSession } from '../core/router.svelte';
	import type { Schema } from '../core/schema';
	import FlyoutHost from './FlyoutHost.svelte';
	import Sidebar from './Sidebar.svelte';

	/**
	 * The frame of the studio's pages: the sidebar, the page, the flyout
	 * over it.
	 */
	let { schema, children }: { schema: Schema; children: Snippet } = $props();

	async function onlogout() {
		await session.logout();
		endSession();
	}
</script>

<!-- the AI chat and the website take the whole height -->
<div class="layout" class:wide={current.path === '/ai'}>
	<Sidebar {schema} {onlogout} />
	<main>{@render children()}</main>
</div>
<FlyoutHost {schema} />

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
