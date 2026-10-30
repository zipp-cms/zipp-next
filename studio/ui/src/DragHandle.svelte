<script lang="ts">
	import Icon from './Icon.svelte';
	import Popover from './Popover.svelte';
	import type { Sortable, Step } from './sortable.svelte';

	/**
	 * Moves a row: drag it, or click it for a menu of the moves, or the
	 * arrow keys while it has the focus (moving shouldn't need dragging).
	 * It shows when the pointer is on its row (`data-sort`), always on
	 * touch screens.
	 */
	let {
		sortable,
		index,
		label = 'Move',
	}: {
		sortable: Sortable;
		/** its row, or how to find it when it changes, like in a rich text */
		index: number | (() => number);
		label?: string;
	} = $props();

	const at = () => (typeof index === 'function' ? index() : index);

	let open = $state(false);

	const STEPS: [Step, string, string][] = [
		['up', 'Move up', 'arrow-up'],
		['down', 'Move down', 'arrow-down'],
		['out', 'Move out a level', 'outdent'],
		['in', 'Move under the one above', 'indent'],
	];
	const KEYS: Record<string, Step> = {
		ArrowUp: 'up',
		ArrowDown: 'down',
		ArrowLeft: 'out',
		ArrowRight: 'in',
	};
	// the moves there are, `in` and `out` only in a tree
	const steps = $derived(
		open ? STEPS.filter(([s]) => sortable.stepDrop(at(), s)) : [],
	);

	function onkeydown(e: KeyboardEvent) {
		const step = KEYS[e.key];
		if (!step) return;
		e.preventDefault();
		sortable.step(at(), step);
	}
</script>

<Popover bind:open width="13rem" {label}>
	{#snippet trigger()}
		<button
			type="button"
			class="icon drag-handle"
			title="{label}: drag, or click for the moves"
			aria-label={label}
			aria-expanded={open}
			style:touch-action="none"
			onpointerdown={e => sortable.start(e, at())}
			onclick={() => !sortable.dragged && (open = !open)}
			{onkeydown}
		>
			<Icon name="grip" size={15} />
		</button>
	{/snippet}
	{#each steps as [step, text, icon] (step)}
		<button
			type="button"
			class="popup-item"
			role="menuitem"
			onclick={() => {
				open = false;
				sortable.step(at(), step);
			}}
		>
			<Icon name={icon} size={14} />
			{text}
		</button>
	{:else}
		<span class="popup-item note">It can't move</span>
	{/each}
</Popover>

<style lang="scss">
	@use '../vars' as *;

	.drag-handle {
		cursor: grab;
		color: var(--subtle);
	}

	// only on the row under the pointer, or with the focus
	@include hover {
		.drag-handle {
			opacity: 0;
			transition: opacity 0.15s;
		}

		:global([data-sort]:hover) .drag-handle,
		.drag-handle:focus-visible,
		.drag-handle[aria-expanded='true'] {
			opacity: 1;
		}
	}
</style>
