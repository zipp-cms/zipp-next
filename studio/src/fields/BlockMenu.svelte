<script lang="ts">
	import type { Component } from '../core/schema';
	import AddGroup from 'zipp-ui/AddGroup.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Popover from 'zipp-ui/Popover.svelte';
	import BlockTypeList from './BlockTypeList.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * Adds a block: as a bar with the types (the first ones as buttons,
	 * the rest under More), or `inline` a small + between blocks.
	 */
	let {
		types,
		onpick,
		inline = false,
	}: {
		types: Component[];
		onpick: (type: string) => void;
		inline?: boolean;
	} = $props();

	// as many buttons as fit a line, the rest in the popover
	const SHOWN = 6;
	const buttons = $derived(inline ? [] : types.slice(0, SHOWN));
	const rest = $derived(inline ? types : types.slice(SHOWN));

	let open = $state(false);

	function toggle(popover: () => void) {
		// with a single type there is nothing to choose
		if (inline && types.length === 1) return onpick(types[0].handle);
		popover();
	}

	function pick(handle: string) {
		open = false;
		onpick(handle);
	}
</script>

{#if inline}
	<div class="inline" class:open>
		<Popover
			bind:open
			align="center"
			width="14rem"
			label={t('fields.addBlock')}
		>
			{#snippet trigger({ toggle: show })}
				<button
					type="button"
					class="insert"
					title={t('fields.addBlockHere')}
					onclick={() => toggle(show)}
				>
					<Icon name="add" size={14} />
				</button>
			{/snippet}
			<BlockTypeList types={rest} onpick={pick} />
		</Popover>
	</div>
{:else}
	<div class="bar">
		<AddGroup
			options={buttons.map(c => ({ value: c.handle, label: c.name }))}
			onadd={pick}
		>
			{#snippet more()}
				{#if rest.length}
					<Popover
						bind:open
						width="14rem"
						label={t('fields.moreBlocks')}
					>
						{#snippet trigger({ toggle: show })}
							<button type="button" onclick={() => toggle(show)}>
								{t('fields.more')}
								<Icon name="chevron-down" size={14} />
							</button>
						{/snippet}
						<BlockTypeList types={rest} onpick={pick} />
					</Popover>
				{/if}
			{/snippet}
		</AddGroup>
	</div>
{/if}

<style lang="scss">
	// between two blocks, it shows when the pointer is near
	.inline {
		position: relative;
		z-index: var(--z-raised);
		display: flex;
		justify-content: center;
		height: 0;
	}

	// in the middle of the space above, between two blocks
	.insert {
		transform: translateY(calc(-50% - 0.375rem));
		padding: 2px;
		border-radius: 50%;
		color: var(--accent);
	}

	// only where a pointer can come near, always on touch
	@include hover {
		.insert {
			opacity: 0;
			transition: opacity 0.15s;
		}

		.inline:hover .insert,
		.insert:focus-visible,
		.open .insert {
			opacity: 1;
		}
	}
</style>
