<script lang="ts">
	import { onDestroy, untrack } from 'svelte';
	import type { Schema } from '../core/schema';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { sideNav } from '../shell/sideNav.svelte';
	import { hasAgent } from '../core/access';
	import { Chat } from './chat.svelte';
	import { markdown } from '../core/markdown';
	import { SitePreview } from './preview.svelte';
	import { entryPath, site } from '../core/router.svelte';
	import Preview from './Preview.svelte';

	/**
	 * The AI chat on the left, the website on the right with every draft.
	 * Changes reload the page, crelte's live preview mode keeps the
	 * scroll position.
	 */
	let { schema }: { schema: Schema } = $props();

	let log: HTMLElement;
	const scroll = () =>
		queueMicrotask(() => log?.scrollTo({ top: log.scrollHeight }));

	// development: the coding agent does everything, content and code;
	// else the first model, the choice is hidden for now (operator
	// 2026-10-28)
	const chat = untrack(
		() =>
			new Chat(
				hasAgent(schema) ? null : (schema.aiModels[0]?.id ?? ''),
				scroll,
			),
	);
	const agent = untrack(() => hasAgent(schema));
	const preview = untrack(
		() => new SitePreview(schema, site.current, chat.savedView),
	);
	let input = $state('');

	// the chat's width, kept in this browser
	const DEFAULT_WIDTH = 420;
	const MIN_WIDTH = 300;
	const WIDTH_KEY = 'zipp-ai-width';
	const maxWidth = () => Math.max(MIN_WIDTH, Math.round(innerWidth * 0.6));
	let width = $state(
		clampWidth(Number(localStorage.getItem(WIDTH_KEY)) || DEFAULT_WIDTH),
	);
	let resizing = $state(false);

	function clampWidth(w: number) {
		return Math.round(Math.min(Math.max(w, MIN_WIDTH), maxWidth()));
	}

	function setWidth(w: number) {
		width = clampWidth(w);
		localStorage.setItem(WIDTH_KEY, `${width}`);
	}

	function startResize(e: PointerEvent) {
		if (e.button !== 0) return;
		e.preventDefault();
		const el = e.currentTarget as HTMLElement;
		el.setPointerCapture(e.pointerId);
		const from = e.clientX - width;
		resizing = true;
		const move = (ev: PointerEvent) => setWidth(ev.clientX - from);
		const end = () => {
			resizing = false;
			el.removeEventListener('pointermove', move);
			el.removeEventListener('pointerup', end);
			el.removeEventListener('pointercancel', end);
		};
		el.addEventListener('pointermove', move);
		el.addEventListener('pointerup', end);
		el.addEventListener('pointercancel', end);
	}

	function resizeKey(e: KeyboardEvent) {
		const step = e.shiftKey ? 80 : 20;
		if (e.key === 'ArrowLeft') setWidth(width - step);
		else if (e.key === 'ArrowRight') setWidth(width + step);
		else return;
		e.preventDefault();
	}

	$effect(() => chat.save(preview.view));

	async function send(e: Event) {
		e.preventDefault();
		const text = input.trim();
		if (!text || chat.busy) return;
		input = '';
		const { ok, after } = await chat.send(text, preview.view);
		if (!ok) input = text;
		if (after && 'open' in after) preview.open(after.open);
		else if (after) preview.refresh();
	}

	/** The textarea as high as its text, up to a third of the window. */
	function grow(el: HTMLTextAreaElement, _value: string) {
		const fit = () => {
			el.style.height = 'auto';
			el.style.height = `${Math.min(el.scrollHeight, innerHeight / 3)}px`;
		};
		fit();
		return { update: fit };
	}

	function onkeydown(e: KeyboardEvent) {
		// not while an input method composes a character
		if (e.key === 'Enter' && !e.shiftKey && !e.isComposing) send(e);
	}

	// leaving the page stops the answer
	onDestroy(() => chat.stop());
</script>

<div class="split" style:--chat="{width}px">
	<section class="chat">
		<!-- the border to the website: drag it, or the arrow keys; a
		     focusable separator is ARIA's window splitter -->
		<!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions -->
		<div
			class="resize"
			class:resizing
			role="separator"
			aria-orientation="vertical"
			aria-label="Width of the chat"
			aria-valuenow={width}
			aria-valuemin={MIN_WIDTH}
			aria-valuemax={maxWidth()}
			tabindex="0"
			title="Drag to change the width, double click for the default"
			onpointerdown={startResize}
			ondblclick={() => setWidth(DEFAULT_WIDTH)}
			onkeydown={resizeKey}
		></div>
		<PageTop title="AI" bleed={false}>
			{#snippet lead()}
				<button
					type="button"
					class="icon"
					title="Menu"
					aria-label="Menu"
					aria-expanded={sideNav.open}
					onclick={() => (sideNav.open = true)}
				>
					<Icon name="sidebar" size={16} />
				</button>
			{/snippet}
			{#snippet actions()}
				<button
					onclick={() => chat.clear()}
					disabled={chat.busy || !chat.lines.length}
					title="Start over, this chat is only kept in this tab"
				>
					<Icon name="add" size={14} /> New chat
				</button>
			{/snippet}
		</PageTop>

		<div class="log" bind:this={log}>
			{#if !chat.lines.length}
				<p class="hint">
					{#if agent}
						Development: ask for anything, content, design or new
						fields, e.g. “Add a field for the opening hours” or
						“Make the buttons round”. Code changes show on the right
						right away, content changes are drafts you publish.
					{:else}
						Ask for anything on the website, e.g. “Write a better
						intro for Über uns”, “Add an event in Basel” or
						“Translate the footer into English”. Changes are drafts:
						you see them on the right and publish them in the
						editor.
					{/if}
				</p>
			{/if}
			{#each chat.lines as line, i (i)}
				{#if line.who === 'action'}
					<div class="action">
						<span>{line.action.text}</span>
						{#if line.action.entryId && line.action.section}
							<a
								href={entryPath(
									line.action.section,
									line.action.entryId,
								)}
							>
								Open
							</a>
						{/if}
					</div>
				{:else if line.who === 'ai'}
					<div class="msg ai">
						{@html markdown(line.text)}
					</div>
				{:else}
					<div class="msg user">{line.text}</div>
				{/if}
			{/each}
			{#if chat.busy && chat.lines[chat.lines.length - 1]?.who !== 'ai'}
				<div class="msg ai busy">…</div>
			{/if}
			{#if chat.error || preview.error}
				<p class="error">{chat.error || preview.error}</p>
			{/if}
		</div>

		<!-- one box: the text grows with it, the send button in its corner -->
		<form onsubmit={send}>
			<div class="composer">
				<textarea
					bind:value={input}
					{onkeydown}
					rows="1"
					placeholder="What should change?"
					aria-label="Message"
					spellcheck="false"
					use:grow={input}
				></textarea>
				<div class="bottom">
					<span class="keys">
						Enter sends, Shift+Enter a new line
					</span>
					<button
						class="send"
						title="Send"
						aria-label="Send"
						disabled={chat.busy || !input.trim()}
					>
						<Icon name="arrow-up" size={16} />
					</button>
				</div>
			</div>
		</form>
	</section>

	<Preview {preview} {resizing} />
</div>

<style lang="scss">
	.split {
		display: grid;
		grid-template-columns: var(--chat) minmax(0, 1fr);
		height: 100%;
	}

	.chat {
		position: relative;
		display: flex;
		flex-direction: column;
		min-height: 0;
		background: var(--surface);
		border-right: 1px solid var(--line);
	}

	// over the chat's right border, a line in the accent on hover
	.resize {
		position: absolute;
		top: 0;
		right: -4px;
		bottom: 0;
		z-index: var(--z-raised);
		width: 7px;
		cursor: col-resize;
		touch-action: none;

		&::after {
			content: '';
			position: absolute;
			top: 0;
			bottom: 0;
			left: 3px;
			width: 1px;
			transition: background 0.15s;
		}

		&:hover::after,
		&:focus-visible::after,
		&.resizing::after {
			left: 2px;
			width: 3px;
			background: var(--accent);
		}

		&:focus-visible {
			outline: none;
		}
	}

	.log {
		flex: 1;
		overflow-y: auto;
		padding: 0 1rem;
	}

	.hint {
		color: var(--muted);
		line-height: 1.5;
	}

	.msg {
		margin: 0.75rem 0;
		padding: 0.625rem 0.75rem;
		border-radius: var(--radius-sm);
		line-height: 1.45;
		overflow-wrap: anywhere;

		&.user {
			margin-left: 2rem;
			background: var(--text);
			color: var(--surface);
			white-space: pre-wrap;
		}

		&.ai {
			margin-right: 2rem;
			background: var(--input);

			:global(p) {
				margin: 0 0 0.5rem;
			}

			:global(p:last-child) {
				margin: 0;
			}
		}

		&.busy {
			color: var(--muted);
		}
	}

	.action {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		margin: 0.5rem 0;
		font-size: var(--fs-base);
		color: var(--muted);

		span {
			flex: 1;
		}

		&::before {
			content: '';
			width: 0.5rem;
			height: 0.5rem;
			border-radius: 50%;
			background: var(--changed);
		}
	}

	form {
		padding: 0.75rem 1rem 1rem;
	}

	.composer {
		border: 1px solid var(--border-strong);
		border-radius: var(--radius-lg);
		background: var(--input);

		&:focus-within {
			@include focus-ring(-1px);
			border-color: var(--accent);
		}

		// the box has the border and the focus
		textarea {
			min-height: 0;
			padding: 0.625rem 0.75rem 0.25rem;
			border: none;
			background: none;
			line-height: 1.45;
			resize: none;

			&:focus {
				outline: none;
				box-shadow: none;
			}
		}
	}

	.bottom {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
		padding: 0 0.375rem 0.375rem 0.75rem;
	}

	.keys {
		color: var(--subtle);
		font-size: var(--fs-xs);
	}

	.send {
		display: grid;
		place-items: center;
		width: 2rem;
		height: 2rem;
		padding: 0;
		border: none;
		border-radius: 50%;
		background: var(--accent);
		color: var(--on-accent);

		@include hover {
			&:hover:not(:disabled) {
				background: var(--accent-hover);
				color: var(--on-accent);
			}
		}

		&:disabled {
			background: var(--surface-2);
			color: var(--subtle);
		}
	}

	@include max-tablet {
		.split {
			grid-template-columns: 1fr;
			grid-template-rows: 60vh 80vh;
		}
	}
</style>
