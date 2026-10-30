<script lang="ts">
	import type { LinkKind } from '../../core/schema';
	import AddGroup from 'zipp-ui/AddGroup.svelte';
	import { LINK_NAMES } from './link';

	/** Adds a link of one of the kinds: `+ Entry | URL`. */
	let { to, onadd }: { to: LinkKind[]; onadd: (kind: LinkKind) => void } =
		$props();
</script>

<span class="link-add">
	<AddGroup
		options={to.map(k => ({ value: k, label: LINK_NAMES[k] }))}
		{onadd}
	/>
</span>
