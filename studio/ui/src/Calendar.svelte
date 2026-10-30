<script lang="ts">
	import { untrack } from 'svelte';
	import Icon from './Icon.svelte';
	import {
		formatDay,
		monthDays,
		monthNames,
		monthOf,
		parseDay,
		weekdays,
	} from './date';

	/**
	 * A month to pick a day of, like hilterfingen's date picker: the
	 * month's name opens its year's months, the year the years around it;
	 * picking one goes back down to the days. `min` and `max` are days
	 * (`2026-11-14`) that can't be passed.
	 */
	let {
		value,
		onpick,
		min = null,
		max = null,
	}: {
		/** the chosen day, `2026-11-14`, or none */
		value: string | null;
		onpick: (day: string) => void;
		min?: string | null;
		max?: string | null;
	} = $props();

	const chosen = $derived(parseDay(value));
	const first = $derived(parseDay(min));
	const last = $derived(parseDay(max));

	// the month of the chosen day at the start, then where it was moved
	let shown = $state(untrack(() => monthOf(parseDay(value) ?? new Date())));
	let view = $state<'days' | 'months' | 'years'>('days');

	const months = monthNames();
	const days = weekdays();
	const title = new Intl.DateTimeFormat(undefined, {
		month: 'long',
		year: 'numeric',
	});
	const spoken = new Intl.DateTimeFormat(undefined, { dateStyle: 'full' });
	const today = formatDay(new Date());

	const decade = $derived(Math.floor(shown.getFullYear() / 12) * 12);
	const period = $derived(
		view === 'days'
			? title.format(shown)
			: view === 'months'
				? String(shown.getFullYear())
				: `${decade}–${decade + 11}`,
	);

	/** Whether a range of days is all before `min` or after `max`. */
	const outside = (from: Date, to: Date) =>
		(!!first && to < first) || (!!last && from > last);

	/** Back or forth a month, a year or twelve years, as the view shows. */
	function move(by: number) {
		const [y, m] = [shown.getFullYear(), shown.getMonth()];
		shown =
			view === 'days'
				? new Date(y, m + by, 1)
				: new Date(y + by * (view === 'months' ? 1 : 12), m, 1);
	}
</script>

<div class="calendar">
	<div class="head">
		<button
			type="button"
			class="icon"
			aria-label={view === 'days' ? 'Month before' : 'Back'}
			onclick={() => move(-1)}
		>
			<Icon name="chevron-left" size={16} />
		</button>
		<button
			type="button"
			class="period plain"
			disabled={view === 'years'}
			aria-label={view === 'days' ? 'Pick a month' : 'Pick a year'}
			onclick={() => (view = view === 'days' ? 'months' : 'years')}
		>
			<span aria-live="polite">{period}</span>
		</button>
		<button
			type="button"
			class="icon"
			aria-label={view === 'days' ? 'Month after' : 'Forward'}
			onclick={() => move(1)}
		>
			<Icon name="chevron-right" size={16} />
		</button>
	</div>

	{#if view === 'days'}
		<div class="days">
			{#each days as d (d)}<span class="weekday">{d}</span>{/each}
			{#each monthDays(shown) as day, i (i)}
				{#if day}
					{@const iso = formatDay(day)}
					<button
						type="button"
						class="day"
						class:on={chosen && iso === formatDay(chosen)}
						class:today={iso === today}
						aria-label={spoken.format(day)}
						aria-pressed={!!chosen && iso === formatDay(chosen)}
						aria-current={iso === today ? 'date' : undefined}
						disabled={outside(day, day)}
						onclick={() => onpick(iso)}
					>
						{day.getDate()}
					</button>
				{:else}
					<span></span>
				{/if}
			{/each}
		</div>
	{:else if view === 'months'}
		<div class="options">
			{#each months as name, m (m)}
				{@const y = shown.getFullYear()}
				<button
					type="button"
					class="option"
					class:on={chosen?.getFullYear() === y &&
						chosen.getMonth() === m}
					disabled={outside(new Date(y, m, 1), new Date(y, m + 1, 0))}
					onclick={() => {
						shown = new Date(y, m, 1);
						view = 'days';
					}}
				>
					{name}
				</button>
			{/each}
		</div>
	{:else}
		<div class="options">
			{#each Array.from({ length: 12 }, (_, i) => decade + i) as y (y)}
				<button
					type="button"
					class="option"
					class:on={chosen?.getFullYear() === y}
					disabled={outside(new Date(y, 0, 1), new Date(y, 11, 31))}
					onclick={() => {
						shown = new Date(y, shown.getMonth(), 1);
						view = 'months';
					}}
				>
					{y}
				</button>
			{/each}
		</div>
	{/if}
</div>

<style lang="scss">
	@use '../vars' as *;

	.head {
		display: grid;
		grid-template-columns: 2rem 1fr 2rem;
		align-items: center;
		gap: 0.25rem;
		padding-bottom: 0.375rem;
		border-bottom: 1px solid var(--line);
	}

	.period {
		justify-content: center;
		padding: 0.375rem;
		border-radius: var(--radius);
		font-weight: 600;
		text-transform: capitalize;

		@include hover {
			&:hover:not(:disabled) {
				background: var(--hover);
			}
		}

		&:disabled {
			opacity: 1;
			cursor: default;
		}
	}

	.days {
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		gap: 0.125rem;
		padding-top: 0.5rem;
	}

	.weekday {
		padding-bottom: 0.25rem;
		color: var(--muted);
		font-size: var(--fs-xs);
		font-weight: 600;
		text-align: center;
	}

	.day,
	.option {
		justify-content: center;
		padding: 0;
		border: 1px solid transparent;
		background: none;
		color: var(--text);
		font-weight: 400;

		@include hover {
			&:hover:not(:disabled) {
				border-color: var(--border-strong);
				background: none;
			}
		}

		&.on {
			border-color: var(--text);
			background: var(--text);
			color: var(--surface);
		}

		&:disabled {
			color: var(--subtle);
			opacity: 0.55;
		}
	}

	.day {
		aspect-ratio: 1;
		border-radius: 50%;
		font-size: var(--fs-md);

		&.today:not(.on) {
			border-color: var(--accent);
			color: var(--accent);
			font-weight: 600;
		}
	}

	.options {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 0.375rem;
		padding-top: 0.5rem;
	}

	.option {
		height: 2.5rem;
		border-radius: var(--radius);
		text-transform: capitalize;
	}
</style>
