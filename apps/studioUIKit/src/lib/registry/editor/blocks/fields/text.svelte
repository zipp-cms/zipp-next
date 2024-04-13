<script lang="ts">
	import type { TextField, BlockOf } from '../../editor.ts';

	export let block: BlockOf<TextField>;
</script>

<input type="text" class="w-full rounded-lg p-2 hover:bg-gray-100" placeholder={block.name} />
