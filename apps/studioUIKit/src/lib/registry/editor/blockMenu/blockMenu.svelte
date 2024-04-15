<script lang="ts">
	import { tick } from 'svelte';
	import type { BlockOf, ComponentContext } from '../editor.ts';

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
	}

	// autofocus the textarea when focused
	let textarea: HTMLElement;
	$: if (focused) {
		tick().then(() => {
			textarea.focus();
		});
	}
</script>

{#if focused}
	<div class="relative flex-grow">
		{#if open}
			<ul class="absolute z-10 -translate-y-full rounded bg-white shadow">
				{#each block.settings.component as component}
					<li>
						<button on:click={() => choose(component)}>
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
			on:keydown={(e) => {
				if (e.key === '/') {
					open = true;
				}
			}}
		></textarea>
	</div>
{/if}
