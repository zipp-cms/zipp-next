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
<div class="layout" class:wide={current.path === '/admin/ai'}>
	<Sidebar {schema} {onlogout} />
	<main>{@render children()}</main>
</div>
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
		height: 100vh;

		main {
			padding: 0;
			height: 100vh;
		}
	}

	@include max-tablet {
		.layout {
			grid-template-columns: 1fr;
		}
	}
</style>
