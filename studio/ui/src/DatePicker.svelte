<script lang="ts">
	import Calendar from './Calendar.svelte';
	import Icon from './Icon.svelte';
	import Popover from './Popover.svelte';
	import { formatDay, parseDay } from './date';

	/**
	 * A date, and with `time` its time, like hilterfingen's picker: the day
	 * shows as the browser writes it (14.11.2026), a click opens the
	 * calendar. The value is `2026-11-14`, with a time `2026-11-14T20:00`,
	 * empty for none; `onchange` gets it.
	 */
	let {
		value,
		onchange,
		time = false,
		label = 'Date',
		placeholder = 'No date',
		min = null,
		max = null,
		disabled = false,
	}: {
		value: string;
		onchange: (value: string) => void;
		time?: boolean;
		/** the name for screen readers, the label of the field has it */
		label?: string;
		placeholder?: string;
		/** the first and last day that can be picked, `2026-11-14` */
		min?: string | null;
		max?: string | null;
		disabled?: boolean;
	} = $props();

	let open = $state(false);

	const day = $derived(value.slice(0, 10));
	const clock = $derived(value.slice(11, 16));
	const shown = $derived.by(() => {
		const d = parseDay(day);
		return d
			? d.toLocaleDateString(undefined, { dateStyle: 'medium' })
			: '';
	});

	/** Day and time as one value; a new day without a time gets midnight. */
	function set(d: string, t: string) {
		if (!d) return onchange('');
		onchange(time ? `${d}T${t || '00:00'}` : d);
	}
</script>

<!-- one box like an input: the day, its time, the x that empties it -->
<span class="date-picker" class:with-time={time} class:disabled>
	<Popover bind:open width="18rem" label="Pick a day">
		{#snippet trigger({ toggle, open })}
			<button
				type="button"
				class="field"
				aria-label="{label}: {shown || placeholder}"
				aria-haspopup="dialog"
				aria-expanded={open}
				{disabled}
				onclick={toggle}
			>
				<Icon name="calendar" size={15} />
				<span class:placeholder={!shown}>{shown || placeholder}</span>
			</button>
		{/snippet}
		{#key open}
			<Calendar
				value={day}
				{min}
				{max}
				onpick={d => {
					set(d, clock);
					open = false;
				}}
			/>
		{/key}
		<div class="foot">
			<button
				type="button"
				class="link"
				onclick={() => {
					set(formatDay(new Date()), clock);
					open = false;
				}}
			>
				Today
			</button>
			{#if value}
				<button
					type="button"
					class="link"
					onclick={() => {
						set('', '');
						open = false;
					}}
				>
					Clear
				</button>
			{/if}
		</div>
	</Popover>
	<!-- a time only for a day, no empty grey box next to None -->
	{#if time && day}
		<input
			type="time"
			aria-label="{label}: time"
			value={clock}
			{disabled}
			onchange={e => set(day, e.currentTarget.value)}
		/>
	{/if}
	{#if value && !disabled}
		<button
			type="button"
			class="icon clear"
			aria-label="{label}: clear"
			title="Clear"
			onclick={() => set('', '')}
		>
			<Icon name="close" size={14} />
		</button>
	{/if}
</span>

<style lang="scss">
	@use '../vars' as *;

	.date-picker {
		display: flex;
		align-items: center;
		width: 100%;
		max-width: 14rem;
		border: 1px solid var(--border-strong);
		border-radius: var(--radius);
		background: var(--input);

		&.with-time {
			max-width: 22.5rem;
		}

		&:focus-within {
			border-color: var(--accent);
		}

		@include hover {
			&:hover:not(.disabled, :has(.field:disabled)) {
				border-color: var(--accent);
			}
		}

		// also in a disabled fieldset, read only
		&.disabled,
		&:has(.field:disabled) {
			background: var(--surface-2);
		}

		:global(.popover) {
			flex: 1;
			min-width: 0;
		}
	}

	// the day, opens the calendar
	.field {
		justify-content: flex-start;
		gap: 0.5rem;
		width: 100%;
		padding: 0.4375rem 0.625rem;
		border: none;
		border-radius: var(--radius);
		background: none;
		color: var(--text);
		font-size: var(--fs-base);
		font-weight: 400;

		:global(.icon) {
			color: var(--muted);
		}

		@include hover {
			&:hover:not(:disabled) {
				background: none;
			}
		}

		&:focus-visible {
			@include focus-ring(-2px);
		}

		&:disabled {
			opacity: 1;
			color: var(--text-2);
		}

		// one line, a narrow column cuts it
		span {
			overflow: hidden;
			text-overflow: ellipsis;
			white-space: nowrap;
		}

		.placeholder {
			color: var(--muted);
		}
	}

	// in the box after a hairline, as light as the day
	input[type='time'] {
		flex-shrink: 0;
		width: auto;
		padding: 0.25rem 0.375rem;
		border: none;
		border-left: 1px solid var(--line);
		border-radius: 0;
		background: none;
		font-variant-numeric: tabular-nums;

		// the box shows the focus
		&:focus {
			outline: none;
		}
	}

	.clear {
		flex-shrink: 0;
		margin-right: 0.25rem;

		&:disabled {
			display: none;
		}
	}

	.foot {
		display: flex;
		justify-content: space-between;
		padding-top: 0.375rem;
		border-top: 1px solid var(--line);
	}
</style>
