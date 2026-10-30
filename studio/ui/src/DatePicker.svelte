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

<span class="date-picker" class:with-time={time}>
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
				<span class:placeholder={!shown}>{shown || placeholder}</span>
				<Icon name="calendar" size={16} />
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
	{#if time}
		<input
			type="time"
			aria-label="{label}: time"
			value={clock}
			disabled={disabled || !day}
			onchange={e => set(day, e.currentTarget.value)}
		/>
	{/if}
</span>

<style lang="scss">
	@use '../vars' as *;

	.date-picker {
		display: inline-flex;
		gap: 0.5rem;
		width: 100%;
		max-width: 14rem;

		&.with-time {
			max-width: 22.5rem;
		}

		:global(.popover) {
			flex: 1;
			min-width: 0;
		}
	}

	// looks like an input, opens the calendar
	.field {
		justify-content: space-between;
		width: 100%;
		padding: 0.4375rem 0.625rem;
		border: 1px solid var(--border-strong);
		border-radius: var(--radius);
		background: var(--input);
		color: var(--text);
		font-size: var(--fs-base);
		font-weight: 400;

		:global(.icon) {
			color: var(--muted);
		}

		@include hover {
			&:hover:not(:disabled) {
				border-color: var(--accent);
				background: var(--input);
			}
		}

		&:focus-visible {
			@include focus-ring(-1px);
		}

		// one line, a narrow column cuts it
		span:first-child {
			overflow: hidden;
			text-overflow: ellipsis;
			white-space: nowrap;
		}

		.placeholder {
			color: var(--subtle);
		}
	}

	input[type='time'] {
		width: 8rem;
		flex-shrink: 0;
	}

	.foot {
		display: flex;
		justify-content: space-between;
		padding-top: 0.375rem;
		border-top: 1px solid var(--line);
	}
</style>
