<script lang="ts">
	import { onDestroy, untrack } from 'svelte';
	import type { Schema } from '../core/schema';
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import SplitHandle from 'zipp-ui/SplitHandle.svelte';
	import { hasAgent } from '../core/access';
	import { Chat } from './chat.svelte';
	import { markdown } from '../core/markdown';
	import { SitePreview } from '../entries/preview.svelte';
	import { entryPath, site } from '../core/router.svelte';
	import { sitePreviewToken } from '../entries/drafts';
	import Preview from '../entries/Preview.svelte';
	import { t } from '../i18n/index.svelte';

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
		() =>
			new SitePreview(
				schema,
				site.current,
				chat.savedView,
				sitePreviewToken,
			),
	);
	let input = $state('');

	// the chat's width, kept in this browser
	let width = $state(0);
	let resizing = $state(false);

	// kept when an answer is done or the page changes, not with every
	// word of the answer
	$effect(() => {
		const view = preview.view;
		if (!chat.busy) untrack(() => chat.save(view));
	});

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

	// leaving the page stops the answer, what came is kept
	onDestroy(() => chat.stop(preview.view));
</script>

<div class="split" style:--chat="{width}px">
	<section class="chat">
		<SplitHandle
			bind:width
			bind:resizing
			key="zipp-ai-width"
			initial={420}
			min={300}
			label={t('ai.width')}
		/>
		<PageTop title={t('ai.title')} bleed={false}>
			{#snippet actions()}
				<button
					onclick={() => chat.clear()}
					disabled={chat.busy || !chat.lines.length}
					title={t('ai.startOver')}
				>
					<Icon name="add" size={14} />
					{t('ai.newChat')}
				</button>
			{/snippet}
		</PageTop>

		<div class="log" bind:this={log}>
			{#if !chat.lines.length}
				<p class="hint">
					{#if agent}
						{t('ai.hintAgent')}
					{:else}
						{t('ai.hint')}
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
								{t('common.open')}
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
			{#if chat.error}
				<p class="error">{chat.error}</p>
			{/if}
		</div>

		<!-- one box: the text grows with it, the send button in its corner -->
		<form onsubmit={send}>
			<div class="composer">
				<textarea
					bind:value={input}
					{onkeydown}
					rows="1"
					placeholder={t('ai.placeholder')}
					aria-label={t('ai.message')}
					spellcheck="false"
					use:grow={input}
				></textarea>
				<div class="bottom">
					<span class="keys">
						{t('ai.keys')}
					</span>
					<button
						class="send"
						title={t('ai.send')}
						aria-label={t('ai.send')}
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
