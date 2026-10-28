<script module>
	import Blocks, { blockModules, loadBlocksData } from 'crelte/blocks';

	// every block type has its component in content/, named by its handle
	const mods = blockModules(import.meta.glob('./content/*.svelte'));

	/** @type {import('crelte').LoadDataFn} */
	export const loadData = (cr, blocks) => loadBlocksData(cr, blocks, mods);
</script>

<script>
	let { blocks } = $props();
</script>

<Blocks {blocks} />
