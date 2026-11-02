<script lang="ts">
	import { ui } from './texts.svelte';
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
		label: givenLabel,
		placeholder: givenPlaceholder,
		min = null,
	}: {
		value: string;
		onchange: (value: string) => void;
		time?: boolean;
		/** the name for screen readers, the label of the field has it */
		label?: string;
		placeholder?: string;
		/** the first day that can be picked, `2026-11-14` */
		min?: string | null;
	} = $props();

	const label = $derived(givenLabel ?? ui.texts.date);
	const placeholder = $derived(givenPlaceholder ?? ui.texts.noDate);
	let open = $state(false);

	const day = $derived(value.slice(0, 10));
	const clock = $derived(value.slice(11, 16));
	const shown = $derived.by(() => {
		const d = parseDay(day);
		return d
			? d.toLocaleDateString(ui.locale, { dateStyle: 'medium' })
			: '';
	});

	/** Day and time as one value; a new day without a time gets midnight. */
	function set(d: string, t: string) {
		if (!d) return onchange('');
		onchange(time ? `${d}T${t || '00:00'}` : d);
	}
</script>

<!-- one box like an input: the day, its time, the x that empties it -->
<span class="date-picker" class:with-time={time}>
	<Popover bind:open width="18rem" label={ui.texts.pickDay}>
		{#snippet trigger({ toggle, open })}
			<button
				type="button"
				class="field"
				aria-label="{label}: {shown || placeholder}"
				aria-haspopup="dialog"
				aria-expanded={open}
				onclick={toggle}
			>
				<Icon name="calendar" size={15} />
				<span class="truncate" class:placeholder={!shown}>
					{shown || placeholder}
				</span>
			</button>
		{/snippet}
		{#key open}
			<Calendar
				value={day}
				{min}
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
				{ui.texts.today}
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
					{ui.texts.clear}
				</button>
			{/if}
		</div>
	</Popover>
	<!-- a time only for a day, no empty grey box next to None -->
	{#if time && day}
		<input
			type="time"
			aria-label="{label}: {ui.texts.time}"
			value={clock}
			onchange={e => set(day, e.currentTarget.value)}
		/>
	{/if}
	{#if value}
		<button
			type="button"
			class="icon clear"
			aria-label="{label}: {ui.texts.clear}"
			title={ui.texts.clear}
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
			&:hover:not(:has(.field:disabled)) {
				border-color: var(--accent);
			}
		}

		// in a disabled fieldset, read only
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
