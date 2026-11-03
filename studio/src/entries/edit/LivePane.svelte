<script module lang="ts">
	// the live preview on or off, in this browser for every entry
	const KEY = 'zipp-live-preview';
	export const liveSetting = $state({
		on: localStorage.getItem(KEY) === '1',
	});

	export function toggleLive() {
		liveSetting.on = !liveSetting.on;
		localStorage.setItem(KEY, liveSetting.on ? '1' : '');
	}
</script>

<script lang="ts">
	import { untrack, type Snippet } from 'svelte';
	import SplitHandle from 'zipp-ui/SplitHandle.svelte';
	import Preview from '../Preview.svelte';
	import type { SitePreview } from '../preview.svelte';
	import type { EntryEditor } from './entryEditor.svelte';
	import { sideNav } from '../../shell/sideNav.svelte';
	import { t } from '../../i18n/index.svelte';

	/**
	 * The editor, `children`, with the website on the right while the
	 * live preview is on; it shows the draft after every save.
	 */
	let {
		ed,
		preview,
		url,
		width = $bindable(0),
		children,
	}: {
		ed: EntryEditor;
		/** null: the live preview is off */
		preview: SitePreview | null;
		/** the entry's page */
		url: string | null;
		/** of the editor next to the website */
		width?: number;
		children: Snippet;
	} = $props();

	let frameWidth = $state(0);
	let resizing = $state(false);

	// the editor gets the sidebar's room while the website is next to it
	$effect(() => {
		if (!preview) return;
		sideNav.hidden = true;
		return () => (sideNav.hidden = false);
	});

	// the draft is new after every save, publish and discard; a published
	// slug moves the page
	let shownUrl = untrack(() => url);
	$effect(() => {
		void ed.draft;
		const u = url;
		untrack(() => {
			if (u && u !== shownUrl) preview?.open(u);
			else preview?.update();
			shownUrl = u;
		});
	});
</script>

<div class:live={!!preview} style:--frame="{frameWidth}px">
	<div bind:clientWidth={width}>
		{@render children()}
	</div>

	{#if preview}
		<div class="frame">
			<SplitHandle
				bind:width={frameWidth}
				bind:resizing
				key="zipp-live-preview-width"
				initial={560}
				min={320}
				side="end"
				label={t('entries.websiteWidth')}
			/>
			<Preview {preview} {resizing} />
		</div>
	{/if}
</div>

<style lang="scss">
	// the website on the right, as high as the window under the bar
	.live {
		display: grid;
		grid-template-columns: minmax(0, 1fr) var(--frame);
		gap: 1.25rem;
	}

	.frame {
		position: sticky;
		top: 3rem;
		align-self: start;
		display: grid;
		height: calc(100vh - 3rem);
		margin: -1.25rem calc(-1 * var(--page-pad)) -2rem 0;
		border-left: 1px solid var(--line);
	}

	@include max-tablet {
		.live {
			grid-template-columns: 1fr;
		}

		.frame {
			display: none;
		}
	}
</style>
