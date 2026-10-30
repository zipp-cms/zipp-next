<script lang="ts" module>
	type Device = 'desktop' | 'tablet' | 'mobile';

	/** The width of the website in each mode, desktop takes all there is. */
	const WIDTHS: Record<Device, number | null> = {
		desktop: null,
		tablet: 768,
		mobile: 390,
	};

	const DEVICE_KEY = 'zipp-preview-device';
</script>

<script lang="ts">
	import type { SitePreview } from './preview.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Segmented from 'zipp-ui/Segmented.svelte';

	/**
	 * The website with every draft, next to the chat: reload, where it is,
	 * as wide as a desktop, a tablet or a phone (kept in this browser),
	 * open in a new tab. `resizing`: the chat's width is dragged, the frame
	 * may not take the pointer.
	 */
	let {
		preview,
		resizing = false,
	}: { preview: SitePreview; resizing?: boolean } = $props();

	let frame: HTMLIFrameElement;
	$effect(() => preview.start(frame));

	const saved = localStorage.getItem(DEVICE_KEY) as Device | null;
	let device = $state<Device>(saved && saved in WIDTHS ? saved : 'desktop');

	function setDevice(d: Device) {
		device = d;
		localStorage.setItem(DEVICE_KEY, d);
	}

	const devices: { value: Device; label: string }[] = [
		{ value: 'desktop', label: 'Desktop' },
		{ value: 'tablet', label: 'Tablet' },
		{ value: 'mobile', label: 'Mobile' },
	];
</script>

<section class="site">
	<div class="bar">
		<button
			type="button"
			class="icon"
			title="Reload"
			aria-label="Reload"
			onclick={() => preview.refresh()}
		>
			<Icon name="reload" size={15} />
		</button>
		<span class="url" title={decodeURI(preview.view)}>
			{decodeURI(preview.view)}
		</span>
		<Segmented
			options={devices}
			value={device}
			onchange={setDevice}
			label="Width of the website"
		>
			{#snippet item(o)}
				<span class="device" title={o.label}>
					<Icon name={o.value} size={15} />
					<span class="sr-only">{o.label}</span>
				</span>
			{/snippet}
		</Segmented>
		{#if preview.token}
			<a
				class="icon open"
				href={preview.withToken(preview.view)}
				target="_blank"
				rel="noreferrer"
				title="Open in a new tab"
				aria-label="Open in a new tab"
			>
				<Icon name="external" size={15} />
			</a>
		{/if}
	</div>
	<div class="stage" class:framed={WIDTHS[device]}>
		<!-- its src is set by the preview, see `load` -->
		<iframe
			bind:this={frame}
			title="Website"
			style:width={WIDTHS[device] ? `${WIDTHS[device]}px` : null}
			style:pointer-events={resizing ? 'none' : null}
		></iframe>
	</div>
</section>

<style lang="scss">
	.site {
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.bar {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		min-height: 3rem;
		padding: 0.375rem 0.75rem;
		border-bottom: 1px solid var(--line);
		background: var(--surface);
	}

	// like an address field, read only
	.url {
		flex: 1;
		min-width: 0;
		overflow: hidden;
		padding: 0.3125rem 0.75rem;
		border-radius: var(--radius-lg);
		background: var(--hover);
		color: var(--text-2);
		font-size: var(--fs-md);
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.device {
		display: grid;
		place-items: center;
	}

	.bar :global(.segmented button) {
		padding: 0.25rem 0.5rem;
	}

	// a link that looks like the icon buttons
	.open {
		display: grid;
		place-items: center;
		padding: 0.25rem;
		border-radius: var(--radius);
		color: var(--muted);

		@include hover {
			&:hover {
				color: var(--text);
				background: var(--hover);
			}
		}
	}

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		overflow: hidden;
		clip-path: inset(50%);
		white-space: nowrap;
	}

	.stage {
		display: flex;
		flex: 1;
		justify-content: center;
		min-height: 0;
		overflow: auto;
		background: var(--surface);
	}

	// a tablet or phone on the grey of the page
	.framed {
		padding: 1rem;
		background: var(--bg);

		iframe {
			flex: none;
			max-width: 100%;
			border: 1px solid var(--border);
			border-radius: var(--radius-lg);
			box-shadow: var(--shadow);
		}
	}

	iframe {
		flex: 1;
		width: 100%;
		height: 100%;
		border: none;
		background: var(--surface);
	}
</style>
