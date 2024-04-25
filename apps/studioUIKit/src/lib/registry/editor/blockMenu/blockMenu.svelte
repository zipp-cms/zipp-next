<script lang="ts">
	import { onMount, tick } from 'svelte';
	import type { BlockOf, ComponentContext } from '../editor.ts';
	import Fuse from 'fuse.js';
	import { Box } from 'lucide-svelte';

	export let context: ComponentContext;
	export let block: BlockOf<ComponentField>;

	const activeBlock = context.ui.selection.activeBlock;

	let open = false;
	let focused = false;

	// if the activeBlock changes, close the block menu
	$: {
		focused = $activeBlock === block.id;
		if (!focused) {
			open = false;
		}
	}

	function choose(component: string) {
		context.choose(block, component);
		open = false;
		textarea.value = '';
		componentIndex = 0;
	}

	// autofocus the textarea when focused
	let textarea: HTMLElement;
	$: if (focused) {
		tick().then(() => {
			textarea.focus();
		});
	}

	let index: Fuse<string>;
	let filteredComponents = block.settings.component;
	let componentIndex = 0;
	onMount(() => {
		// initialize flexsearch
		index = new Fuse(block.settings.component);
	});

	function handleKeydown(e) {
		if (e.key === 'ArrowDown') {
			e.preventDefault();
			componentIndex = (componentIndex + 1) % filteredComponents.length;
			return;
		}

		if (e.key === 'ArrowUp') {
			e.preventDefault();
			componentIndex = (componentIndex - 1 + filteredComponents.length) % filteredComponents.length;
			return;
		}

		if (e.key === 'Enter') {
			e.preventDefault();
			choose(filteredComponents[componentIndex]);
			return;
		}
	}

	function handleKeyup() {
		const results = index.search(textarea.value.replace('/', ''));

		open = textarea.value[0] === '/';

		if (results.length === 0) {
			filteredComponents = block.settings.component;
			return;
		}

		filteredComponents = results.map((result) => result.item);
	}
</script>

{#if focused}
	<div class="relative flex-grow">
		{#if open}
			<ul class="absolute z-10 -translate-y-full rounded border bg-white p-1 shadow">
				{#each filteredComponents as component, i}
					<li
						data-active={componentIndex === i}
						class="rounded px-2 py-1 data-[active=true]:bg-gray-100"
					>
						<button class="flex items-center gap-1" on:click={() => choose(component)}>
							<Box strokeWidth={1.4} class="text-gray-400" size={18} />
							{component}
						</button>
					</li>
				{/each}
			</ul>
		{/if}

		<textarea
			bind:this={textarea}
			class="w-full resize-none bg-white outline-none"
			placeholder="type / to add a new component"
			on:keydown={handleKeydown}
			on:keyup={handleKeyup}
		></textarea>
	</div>
{/if}
