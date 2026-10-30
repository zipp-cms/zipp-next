<script lang="ts">
	import type { LinkKind } from '../../core/schema';
	import Icon from '../../ui/Icon.svelte';
	import { LINK_NAMES } from './link';

	/** Adds a link of one of the kinds: `+ Entry | URL`. */
	let { to, onadd }: { to: LinkKind[]; onadd: (kind: LinkKind) => void } =
		$props();
</script>

<span class="link-add">
	{#each to as kind, i (kind)}
		<button type="button" onclick={() => onadd(kind)}>
			{#if i === 0}<Icon name="add" size={14} />{/if}
			{LINK_NAMES[kind]}
		</button>
	{/each}
</span>

<style lang="scss">
	// one bordered group, like the bar that adds blocks
	.link-add {
		display: inline-flex;
		flex-shrink: 0;

		button {
			border-radius: 0;
			color: var(--text);

			&:first-child {
				border-radius: var(--radius) 0 0 var(--radius);

				:global(.icon) {
					color: var(--accent);
				}
			}

			&:last-child {
				border-radius: 0 var(--radius) var(--radius) 0;
			}

			&:first-child:last-child {
				border-radius: var(--radius);
			}

			& + button {
				margin-left: -1px;
			}
		}
	}
</style>
