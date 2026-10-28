<script lang="ts">
	import { markdown } from '../lib/markdown';
	import {
		aiChat,
		sitePreviewToken,
		type AiAction,
		type AiMessage,
		type Schema,
	} from '../api';
	import { untrack } from 'svelte';
	import { entryPath, shown, site } from '../route.svelte';

	/**
	 * The AI chat on the left, the website on the right with every draft.
	 * A site with zipp's preview bridge (see the crelte example) tells
	 * which page it shows and opens pages without a reload. Changes reload
	 * the page, crelte's live preview mode keeps the scroll position.
	 */
	let { schema }: { schema: Schema } = $props();

	/** what the chat shows, the conversation for the model is apart */
	type Line =
		| { who: 'user' | 'ai'; text: string }
		| { who: 'action'; action: AiAction };

	const KEY = 'zipp-ai';
	const saved = JSON.parse(sessionStorage.getItem(KEY) ?? 'null');
	let messages = $state<AiMessage[]>(saved?.messages ?? []);
	let lines = $state<Line[]>(saved?.lines ?? []);
	// the schema does not change while the page is open
	const home = untrack(() => schema.siteUrls[site.current] ?? '');
	const models = untrack(() => schema.aiModels.map(m => m.id));
	const stored = localStorage.getItem('zipp-ai-model') ?? '';
	let model = $state(models.includes(stored) ? stored : models[0]);
	let input = $state('');
	let busy = $state(false);
	let error = $state('');
	// the page the website shows, without the token
	let view = $state<string>(saved?.view ?? home);
	let token = $state('');
	// the src of the iframe, only set to open the site or without bridge
	let src = $state('');
	let bridge = $state(false);
	let frame: HTMLIFrameElement;
	let log: HTMLElement;

	$effect(() => {
		shown.section = '';
	});

	$effect(() => {
		sessionStorage.setItem(
			KEY,
			JSON.stringify({ messages, lines, view: $state.snapshot(view) }),
		);
	});

	sitePreviewToken()
		.then(t => {
			token = t;
			src = withToken(untrack(() => view));
		})
		.catch(e => (error = e.message));

	// only pages of the website may talk to the studio, the preview could
	// have navigated anywhere
	const bases = untrack(() => Object.values(schema.siteUrls));
	const origins = new Set(bases.map(b => new URL(b).origin));
	function ofSite(url: unknown): url is string {
		return (
			typeof url === 'string' &&
			bases.some(b => url === b || url.startsWith(b.replace(/\/?$/, '/')))
		);
	}

	// from before, it may be of a site that changed
	if (!ofSite(untrack(() => view))) view = home;

	$effect(() => {
		const onMessage = (e: MessageEvent) => {
			if (e.source !== frame?.contentWindow) return;
			if (!origins.has(e.origin)) return;
			if (e.data?.type === 'zipp:location' && ofSite(e.data.url)) {
				bridge = true;
				view = e.data.url;
			}
		};
		window.addEventListener('message', onMessage);
		return () => window.removeEventListener('message', onMessage);
	});

	function withToken(url: string) {
		if (!url) return '';
		const u = new URL(url);
		u.searchParams.set('token', token);
		u.searchParams.set('x-craft-live-preview', '1');
		return u.href;
	}

	/** To the website only, the url has the token. */
	function post(msg: object, url: string) {
		frame?.contentWindow?.postMessage(msg, new URL(url).origin);
	}

	/** Opens a page in the preview. */
	function open(url: string) {
		if (!ofSite(url)) return;
		view = url;
		if (bridge) post({ type: 'zipp:open', url: withToken(url) }, url);
		else src = withToken(url);
	}

	/** Shows the changes: setting src loads the page again. */
	function refresh() {
		if (frame) frame.src = withToken(view);
	}

	function scroll() {
		queueMicrotask(() => log?.scrollTo({ top: log.scrollHeight }));
	}

	async function send(e: Event) {
		e.preventDefault();
		const text = input.trim();
		if (!text || busy) return;
		input = '';
		error = '';
		lines.push({ who: 'user', text });
		scroll();
		busy = true;
		const sent = [
			...$state.snapshot(messages),
			{ kind: 'user', text } as AiMessage,
		];
		try {
			const r = await aiChat(model, sent, view || null);
			messages = [...sent, ...r.messages];
			for (const action of r.actions)
				if (!action.navigate) lines.push({ who: 'action', action });
			if (r.text) lines.push({ who: 'ai', text: r.text });
			// the AI opens a page where the change shows, else this one
			// reloads
			const nav = r.actions.findLast(a => a.navigate && a.url);
			if (nav?.url) open(nav.url);
			else if (r.actions.length) refresh();
		} catch (e: any) {
			error = e.message;
			// the message was not answered, it can be sent again
			lines.pop();
			input = text;
		} finally {
			busy = false;
			scroll();
		}
	}

	function onKey(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) send(e);
	}

	function clear() {
		messages = [];
		lines = [];
	}

	function setModel(m: string) {
		model = m;
		localStorage.setItem('zipp-ai-model', m);
	}
</script>

<div class="split">
	<section class="chat">
		<div class="top">
			<h1>AI</h1>
			<select
				value={model}
				onchange={e => setModel(e.currentTarget.value)}
				title="Model"
			>
				{#each schema.aiModels as m (m.id)}
					<option value={m.id}>{m.name}</option>
				{/each}
			</select>
			<button onclick={clear} disabled={busy || !lines.length}>
				New
			</button>
		</div>

		<div class="log" bind:this={log}>
			{#if !lines.length}
				<p class="hint">
					Ask for anything on the website, e.g. “Write a better intro
					for Über uns”, “Add an event in Basel” or “Translate the
					footer into English”. Changes are drafts: you see them on
					the right and publish them in the editor.
				</p>
			{/if}
			{#each lines as line, i (i)}
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
			{#if busy}<div class="msg ai busy">…</div>{/if}
			{#if error}<p class="error">{error}</p>{/if}
		</div>

		<form onsubmit={send}>
			<textarea
				bind:value={input}
				onkeydown={onKey}
				rows="3"
				placeholder="What should change?"
				spellcheck="false"
			></textarea>
			<button class="primary" disabled={busy || !input.trim()}>
				Send
			</button>
		</form>
	</section>

	<section class="site">
		<div class="bar">
			<span class="url">{decodeURI(view)}</span>
			<button class="link" onclick={refresh}>Reload</button>
			<a href={withToken(view)} target="_blank" rel="noreferrer">Open</a>
		</div>
		<iframe bind:this={frame} {src} title="Website"></iframe>
	</section>
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
		background: white;
		border-right: 1px solid var(--line);
	}

	.top {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px;

		h1 {
			flex: 1;
		}

		select {
			width: auto;
		}
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
		border-radius: 2px;
		line-height: 1.45;
		overflow-wrap: anywhere;

		&.user {
			margin-left: 32px;
			background: var(--dark);
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
		font-size: 14px;
		color: var(--muted);

		span {
			flex: 1;
		}

		&::before {
			content: '';
			width: 8px;
			height: 8px;
			border-radius: 50%;
			background: #e8a33d;
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

	.site {
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.bar {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 8px 16px;
		background: white;
		font-size: 14px;

		.url {
			flex: 1;
			overflow: hidden;
			color: var(--muted);
			white-space: nowrap;
			text-overflow: ellipsis;
		}
	}

	iframe {
		flex: 1;
		width: 100%;
		border: none;
		background: white;
	}

	@media (max-width: 800px) {
		.split {
			grid-template-columns: 1fr;
			grid-template-rows: 60vh 80vh;
		}
	}
</style>
