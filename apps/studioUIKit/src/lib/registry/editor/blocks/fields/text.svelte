<script lang="ts">
	import type { TextField, BlockOf } from '../../editor.ts';

	export let block: BlockOf<TextField>;

	function autoExpand(event: InputEvent) {
		const textarea = event.target as HTMLTextAreaElement;
		textarea.style.height = 'auto';
		textarea.style.height = `${textarea.scrollHeight}px`;
	}
</script>

<label class="flex items-baseline">
	<span class="text-foreground/50 font-bold">
		{block.name}
	</span>
	<textarea
		on:input={autoExpand}
		class="flex-grow resize-none bg-transparent p-2 outline-none"
		rows="1"
	></textarea>
	<!-- <input type="text" class="flex-grow p-2" /> -->
</label>
