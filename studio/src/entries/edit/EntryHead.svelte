<script lang="ts">
	import { t } from '../../i18n/index.svelte';

	/** The title, large on top, with the dot of a change and its error. */
	let {
		title = $bindable(),
		changed,
		errors = {},
	}: {
		title: string;
		changed: Set<string>;
		errors?: Record<string, string>;
	} = $props();
</script>

<label class="head">
	<span class="label">
		{#if changed.has('title')}<span class="changed-dot"></span>{/if}{t(
			'common.title',
		)}
		<span class="req">*</span>
	</span>
	<input placeholder={t('common.title')} bind:value={title} />
	{#if errors.title}<span class="error">{errors.title}</span>{/if}
</label>

<style lang="scss">
	.head {
		display: flex;
		flex-direction: column;
		gap: 2px;
		margin-bottom: 1.125rem;
	}

	.label {
		color: var(--muted);
		font-size: var(--fs-xs);
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
	}

	.req {
		color: var(--danger);
	}

	input {
		padding: 0.25rem 0 0.5rem;
		font-size: var(--fs-title);
		font-weight: 700;
		line-height: 1.25;
		border: none;
		border-bottom: 1px solid var(--border);
		border-radius: 0;
		background: none;

		&:focus {
			outline: none;
			border-bottom-color: var(--accent);
		}
	}

	// a narrow editor, see Edit's .main
	@include container-max(30rem) {
		.head {
			margin-bottom: 0.875rem;
		}

		input {
			font-size: var(--fs-xl);
		}
	}
</style>
