<script lang="ts">
	import { onDestroy, untrack } from 'svelte';
	import type { Schema } from '../core/schema';
	import PageTop from '../ui/PageTop.svelte';
	import { hasAgent } from '../core/access';
	import { Chat } from './chat.svelte';
	import { markdown } from '../core/markdown';
	import { SitePreview } from './preview.svelte';
	import { entryPath, shown, site } from '../core/route.svelte';
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

	$effect(() => {
		shown.section = '';
	});

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

	function onkeydown(e: KeyboardEvent) {
		// not while an input method composes a character
		if (e.key === 'Enter' && !e.shiftKey && !e.isComposing) send(e);
	}

	// leaving the page stops the answer
	onDestroy(() => chat.stop());
</script>

<div class="split">
	<section class="chat">
		<PageTop title="AI" bleed={false}>
			{#snippet actions()}
				<button
					onclick={() => chat.clear()}
					disabled={chat.busy || !chat.lines.length}
				>
					New
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
								href="/admin{entryPath(
									line.action.section,
									line.action.entryId,
								)}"
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

		<form onsubmit={send}>
			<textarea
				bind:value={input}
				{onkeydown}
				rows="3"
				placeholder="What should change?"
				spellcheck="false"
			></textarea>
			<button class="primary" disabled={chat.busy || !input.trim()}>
				Send
			</button>
		</form>
	</section>

	<Preview {preview} />
</div>

<style lang="scss">
	.split {
		display: grid;
		grid-template-columns: minmax(320px, 420px) 1fr;
		height: 100%;
	}

	.chat {
		display: flex;
		flex-direction: column;
		min-height: 0;
		background: var(--surface);
		border-right: 1px solid var(--line);
	}

	.log {
		flex: 1;
		overflow-y: auto;
		padding: 0 16px;
	}

	.hint {
		color: var(--muted);
		line-height: 1.5;
	}

	.msg {
		margin: 12px 0;
		padding: 10px 12px;
		border-radius: var(--radius-sm);
		line-height: 1.45;
		overflow-wrap: anywhere;

		&.user {
			margin-left: 32px;
			background: var(--text);
			color: white;
			white-space: pre-wrap;
		}

		&.ai {
			margin-right: 32px;
			background: var(--input);

			:global(p) {
				margin: 0 0 8px;
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
		gap: 10px;
		margin: 8px 0;
		font-size: var(--fs-base);
		color: var(--muted);

		span {
			flex: 1;
		}

		&::before {
			content: '';
			width: 8px;
			height: 8px;
			border-radius: 50%;
			background: var(--changed);
		}
	}

	form {
		display: flex;
		gap: 8px;
		align-items: flex-end;
		padding: 16px;

		textarea {
			resize: none;
		}
	}

	@media (max-width: 800px) {
		.split {
			grid-template-columns: 1fr;
			grid-template-rows: 60vh 80vh;
		}
	}
</style>
