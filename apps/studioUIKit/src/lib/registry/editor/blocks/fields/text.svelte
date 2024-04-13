<script lang="ts">
	import type { TextField, BlockOf } from '../../editor.ts';

	export let block: BlockOf<TextField>;
</script>

<div>
	{block.name}
	<input type="text" class="rounded-lg border p-2 hover:bg-gray-100" />
</div>
