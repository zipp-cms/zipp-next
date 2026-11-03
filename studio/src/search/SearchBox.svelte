<script lang="ts">
	import { tick } from 'svelte';
	import { layer, trapTab } from 'zipp-ui/layers';
	import Icon from 'zipp-ui/Icon.svelte';
	import { go, site } from '../core/router.svelte';
	import { openAsset } from '../core/flyout.svelte';
	import type { Schema } from '../core/schema';
	import Thumb from '../assets/Thumb.svelte';
	import { altOf } from '../assets/api';
	import { Search, highlight, searchBox, type Hit } from './search.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * The search of the studio over the page, like Craft's: entries of
	 * the site and files as you type, by section. The arrow keys choose,
	 * Enter opens an entry in its editor and a file in the flyout,
	 * Escape or a click outside closes it. Like a combobox for screen
	 * readers: the focus stays in the input, which names the chosen line;
	 * Tab stays in the box, closing gives the focus back (`layer`).
	 */
	let { schema }: { schema: Schema } = $props();

	// the schema of a page doesn't change while it is open
	// svelte-ignore state_referenced_locally
	const s = new Search(schema);
	let list = $state<HTMLElement>();
	let input = $state<HTMLInputElement>();
	let box = $state<HTMLElement>();
	// the ids of the lines, for aria-activedescendant
	const uid = $props.id();

	// also from a field that has the focus, unlike autofocus
	$effect(() => input?.focus());

	const close = () => {
		s.stop();
		searchBox.open = false;
	};

	function open(hit: Hit) {
		close();
		if (hit.kind === 'entry') go(hit.entry.path);
		else openAsset(hit.asset.id);
	}

	async function onkeydown(e: KeyboardEvent) {
		if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
			e.preventDefault();
			s.move(e.key === 'ArrowDown' ? 1 : -1);
			await tick();
			list?.querySelector('.selected')?.scrollIntoView({
				block: 'nearest',
			});
		} else if (e.key === 'Enter' && s.hits[s.selected]) {
			e.preventDefault();
			open(s.hits[s.selected]);
		}
	}

	const key = (h: Hit) => (h.kind === 'entry' ? 'e' : 'a') + idOf(h);
	const idOf = (h: Hit) => (h.kind === 'entry' ? h.entry.id : h.asset.id);

	const chosen = $derived(
		s.q.trim() && s.hits[s.selected]
			? `${uid}-${key(s.hits[s.selected])}`
			: undefined,
	);
</script>

{#snippet marked(text: string)}
	{#each highlight(text, s.q) as part, i (i)}
		{#if part.hit}<mark>{part.text}</mark>{:else}{part.text}{/if}
	{/each}
{/snippet}

<div class="backdrop" role="presentation" onclick={close}></div>
<div
	class="dialog"
	role="dialog"
	aria-modal="true"
	aria-label={t('ui.search')}
	tabindex="-1"
	bind:this={box}
	use:layer={close}
	onkeydown={e => box && trapTab(e, box)}
>
	<label class="input">
		<Icon name="search" />
		<input
			type="search"
			placeholder={t('search.placeholder')}
			aria-label={t('search.placeholder')}
			autocomplete="off"
			role="combobox"
			aria-autocomplete="list"
			aria-controls="{uid}-results"
			aria-expanded={!!s.q.trim()}
			aria-activedescendant={chosen}
			spellcheck="false"
			bind:this={input}
			value={s.q}
			oninput={e => s.type(e.currentTarget.value, site.current)}
			{onkeydown}
		/>
	</label>

	{#if s.q.trim()}
		<div
			class="results"
			id="{uid}-results"
			role="listbox"
			aria-label={t('ui.search')}
			bind:this={list}
		>
			{#if s.error}
				<p class="error">{s.error}</p>
			{:else if !s.hits.length}
				<p class="empty">
					{s.loading ? t('search.searching') : t('ui.nothingFound')}
				</p>
			{/if}
			{#each s.groups as g (g.name)}
				<!-- a listbox has only groups and options, the group is named -->
				<div role="group" aria-label={g.name}>
					<div class="name overline" aria-hidden="true">{g.name}</div>
					{#each g.hits as hit (key(hit))}
						{@const i = s.hits.indexOf(hit)}
						<!-- chosen with the arrow keys, not Tab -->
						<button
							type="button"
							class="popup-item hit"
							id="{uid}-{key(hit)}"
							role="option"
							aria-selected={i === s.selected}
							tabindex="-1"
							class:selected={i === s.selected}
							onmousemove={() => (s.selected = i)}
							onclick={() => open(hit)}
						>
							{#if hit.kind === 'entry'}
								<span class="text">
									<span class="title">
										<span class="truncate">
											{@render marked(hit.entry.title)}
										</span>
										{#if !hit.entry.published}
											<span class="badge draft">
												{t('search.notPublished')}
											</span>
										{/if}
									</span>
									{#if hit.entry.snippet}
										<span class="snippet note">
											{@render marked(hit.entry.snippet)}
										</span>
									{/if}
								</span>
							{:else}
								{@const alt = altOf(hit.asset, site.current)}
								<span class="thumb">
									<Thumb asset={hit.asset} small />
								</span>
								<span class="text">
									<span class="truncate">
										{@render marked(
											hit.asset.title ||
												hit.asset.filename,
										)}
									</span>
									{#if hit.asset.title || alt}
										<span class="snippet note truncate">
											{@render marked(
												[
													hit.asset.title &&
														hit.asset.filename,
													alt,
												]
													.filter(t => t)
													.join(' · '),
											)}
										</span>
									{/if}
								</span>
							{/if}
						</button>
					{/each}
				</div>
			{/each}
		</div>
	{/if}
</div>

<style lang="scss">
	.dialog {
		position: fixed;
		top: 12vh;
		left: 50%;
		z-index: var(--z-overlay);
		display: flex;
		flex-direction: column;
		width: 36rem;
		max-width: calc(100vw - 2rem);
		max-height: 76vh;
		transform: translateX(-50%);
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		box-shadow: var(--shadow-pop);
		overflow: hidden;

		// a phone: from the top, the keyboard takes the bottom
		@include max-tablet {
			top: 0.5rem;
			max-height: calc(100dvh - 1rem);
		}
	}

	.input {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		padding: 0 1rem;
		color: var(--subtle);

		input {
			flex: 1;
			min-width: 0;
			padding: 0.875rem 0;
			border: none;
			outline: none;
			box-shadow: none;
			background: none;
			font-size: var(--fs-lg);
		}
	}

	.results {
		padding: 0.25rem 0.5rem 0.5rem;
		border-top: 1px solid var(--line);
		overflow-y: auto;
	}

	.name {
		margin: 0.625rem 0.625rem 0.25rem;
		color: var(--subtle);
	}

	.empty,
	.error {
		margin: 0.75rem 0.625rem;
	}

	.hit {
		align-items: flex-start;

		&.selected {
			background: var(--hover);
		}
	}

	.thumb {
		flex-shrink: 0;
		width: 2rem;
		height: 2rem;
		border-radius: var(--radius-sm);
		overflow: hidden;
	}

	.text {
		display: flex;
		flex-direction: column;
		gap: 1px;
		min-width: 0;
	}

	.title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		min-width: 0;
	}

	// two lines of where it matched at most
	.snippet {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
		font-weight: 400;
		white-space: pre-line;
	}

	// also on the chosen line's background
	mark {
		color: var(--accent);
		background: none;
		font-weight: 600;
	}
</style>
