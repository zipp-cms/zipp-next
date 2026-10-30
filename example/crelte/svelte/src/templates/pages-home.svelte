<script module>
	import Content, {
		loadData as loadContentData,
	} from '@/components/Content.svelte';

	/** @type {import('crelte').LoadData} */
	export const loadData = {
		blocks: (cr, entry) => loadContentData(cr, entry.content),
	};
</script>

<script>
	import Intro from '@/components/Intro.svelte';

	let { entry, blocks } = $props();
</script>

<Intro {entry} />
<Content {blocks} />
