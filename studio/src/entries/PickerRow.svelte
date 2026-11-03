<script lang="ts">
	import Icon from 'zipp-ui/Icon.svelte';
	import type { Schema } from '../core/schema';
	import { componentName, sectionOf } from '../core/schema';
	import { date } from '../core/format';
	import EntryBadges from './EntryBadges.svelte';
	import EntryState from './EntryState.svelte';
	import type { EntryPickerState } from './entryPicker.svelte';
	import type { Row } from './picker';
	import { t } from '../i18n/index.svelte';

	/**
	 * An entry of the picker: in a tree with its caret and level, in a
	 * search with its path and section, in a channel with its date.
	 */
	let {
		picker,
		schema,
		r,
		inTree,
		multiple,
		types,
	}: {
		picker: EntryPickerState;
		schema: Schema;
		r: Row;
		inTree: boolean;
		multiple: boolean;
		/** the type shows when there are several */
		types: string[];
	} = $props();

	const on = $derived(picker.isChosen(r));
	const shut = $derived(picker.closed.has(r.e.id));
	const below = $derived(inTree && r.parent);
	const toggleLabel = $derived(
		shut ? t('entries.showBelow') : t('entries.hideBelow'),
	);
</script>

<div class="line" style:--depth={inTree ? r.depth : 0}>
	{#if below}
		<button
			type="button"
			class="icon caret"
			tabindex="-1"
			aria-expanded={!shut}
			aria-label={toggleLabel}
			title={toggleLabel}
			onclick={() => picker.toggle(r.e.id)}
		>
			<Icon name={shut ? 'chevron-right' : 'chevron-down'} size={14} />
		</button>
	{:else if inTree}
		<span class="caret"></span>
	{/if}
	<button
		type="button"
		class="pick"
		class:chosen={on}
		aria-disabled={!picker.canPick(r)}
		aria-pressed={multiple ? on : undefined}
		aria-current={!multiple && on ? 'true' : undefined}
		aria-expanded={below ? !shut : undefined}
		data-id={r.e.id}
		title={picker.why(r)}
		onclick={() => picker.pick(r)}
	>
		<EntryState entry={r.e} />
		<span class="truncate name">
			{#if !inTree && r.path.length}
				<span class="path">{r.path.join(' › ')} ›</span>
			{/if}
			{r.e.title}
		</span>
		{#if on}<Icon name="check" size={14} />{/if}
		<EntryBadges {schema} draft={r.e.draft} />
		<span class="meta">
			{#if picker.searching && picker.groups.length > 1}
				{sectionOf(schema, r.e.section)?.name}
			{:else if types.length > 1 || !r.pickable}
				{componentName(schema, r.e.type)}
			{/if}
			{#if !inTree && !picker.searching && r.e.postDate}
				<span class="date">{date(r.e.postDate)}</span>
			{/if}
		</span>
	</button>
</div>

<style lang="scss">
	.line {
		--depth: 0;
		display: flex;
		align-items: center;
		flex-shrink: 0;
		padding-left: calc(var(--depth) * 1.3125rem);
	}

	// the open and close of a tree, room for it where there is nothing below
	.caret {
		display: grid;
		flex-shrink: 0;
		place-items: center;
		width: 1.375rem;
		height: 1.375rem;
	}

	.pick {
		display: flex;
		flex: 1;
		align-items: center;
		gap: 0.5rem;
		min-width: 0;
		min-height: 2.25rem;
		padding: 0.375rem 0.625rem;
		border: none;
		border-radius: var(--radius);
		background: none;
		color: var(--text);
		font-weight: 500;
		text-align: left;

		@include hover {
			&:hover {
				background: var(--hover);
			}
		}

		&.chosen {
			background: var(--accent-soft);

			:global(.icon) {
				color: var(--accent);
			}
		}

		&[aria-disabled='true'] {
			color: var(--muted);
			cursor: default;

			@include hover {
				&:hover {
					background: none;
				}
			}
		}

		// a chosen one stays marked when it can't be taken back
		&.chosen[aria-disabled='true'] {
			color: var(--text);

			@include hover {
				&:hover {
					background: var(--accent-soft);
				}
			}
		}
	}

	.name {
		min-width: 0;
	}

	.path {
		color: var(--muted);
		font-weight: 400;
	}

	.meta {
		display: flex;
		gap: 0.75rem;
		flex-shrink: 0;
		margin-left: auto;
		color: var(--subtle);
		font-size: var(--fs-md);
		font-weight: 400;
	}

	@include max-tablet {
		.date {
			display: none;
		}
	}
</style>
