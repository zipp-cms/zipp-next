<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { Asset } from './api';
	import { openAsset } from '../core/flyout.svelte';
	import Thumb from './Thumb.svelte';

	/** A chosen file in a row, like an entry: it opens in the flyout. */
	let { asset, actions }: { asset: Asset; actions: Snippet } = $props();
</script>

<div class="chip box">
	<button
		type="button"
		class="open plain"
		title="Details: title, alt text, focal point"
		onclick={() => openAsset(asset.id)}
	>
		<span class="thumb"><Thumb {asset} /></span>
		<span class="truncate">{asset.title || asset.filename}</span>
	</button>
	{@render actions()}
</div>

<style lang="scss">
	.chip {
		padding-left: 4px;
	}

	.thumb {
		width: 30px;
		flex-shrink: 0;
	}
</style>
