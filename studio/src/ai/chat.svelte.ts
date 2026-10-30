import { agentEvents, agentForget, agentStart, type AgentEvent } from './agent';
import { aiChat, type AiAction, type AiMessage } from './api';
import { ApiError } from '../core/client';

/** what the chat shows, the conversation for the model is apart */
export type Line =
	{ who: 'user' | 'ai'; text: string } | { who: 'action'; action: AiAction };

/** What the preview should do after an answer. */
export type After = { open: string } | { refresh: true } | null;

const KEY = 'zipp-ai';

/**
 * The AI chat: the conversation with a model, or in development with the
 * coding agent. It is kept in the session storage, so a reload of the
 * studio doesn't lose it.
 */
export class Chat {
	messages = $state<AiMessage[]>([]);
	lines = $state<Line[]>([]);
	busy = $state(false);
	error = $state('');
	/** the page of the preview, from before */
	readonly savedView: string | null;

	constructor(
		/** the model, null for the coding agent */
		private readonly model: string | null,
		/** after a line: the log scrolls down */
		private readonly onLine: () => void,
	) {
		const saved = readSaved();
		this.messages = saved?.messages ?? [];
		this.lines = saved?.lines ?? [];
		this.savedView = saved?.view ?? null;
	}

	save(view: string) {
		sessionStorage.setItem(
			KEY,
			JSON.stringify({
				messages: $state.snapshot(this.messages),
				lines: $state.snapshot(this.lines),
				view,
			}),
		);
	}

	/**
	 * Sends a message, `view` is the page the preview shows; what the
	 * preview should do comes back. On an error the message wasn't
	 * answered, the caller gets false to keep the text.
	 */
	async send(
		text: string,
		view: string,
	): Promise<{ ok: boolean; after: After }> {
		this.error = '';
		const from = this.lines.length;
		this.lines.push({ who: 'user', text });
		this.onLine();
		this.busy = true;
		try {
			const after =
				this.model === null
					? await this.sendAgent(text, view)
					: await this.sendModel(this.model, text, view);
			return { ok: true, after };
		} catch (e: any) {
			if (this.abort.signal.aborted) return { ok: true, after: null };
			this.error = e.message;
			// the model's conversation didn't get it, or the agent didn't
			// start: it can be sent again
			const nothing = this.lines.length === from + 1;
			if (this.model !== null || nothing) {
				this.lines.splice(from);
				return { ok: false, after: null };
			}
			return { ok: true, after: null };
		} finally {
			this.busy = false;
			this.onLine();
		}
	}

	private async sendModel(model: string, text: string, view: string) {
		const sent = [
			...$state.snapshot(this.messages),
			{ kind: 'user', text } as AiMessage,
		];
		const actions: AiAction[] = [];
		const added = await aiChat(
			model,
			sent,
			view || null,
			{
				// the answer as it comes, a new bubble after an action
				delta: t => {
					const last = this.lines[this.lines.length - 1];
					if (last?.who === 'ai') last.text += t;
					else this.lines.push({ who: 'ai', text: t });
					this.onLine();
				},
				action: a => {
					actions.push(a);
					if (!a.navigate)
						this.lines.push({ who: 'action', action: a });
					this.onLine();
				},
			},
			this.abort.signal,
		);
		this.messages = [...sent, ...added];
		// the AI opens a page where the change shows, else this one reloads
		const nav = actions.findLast(a => a.navigate && a.url);
		if (nav?.url) return { open: nav.url };
		return actions.length ? { refresh: true as const } : null;
	}

	/** Development: the agent works, its text and steps come as lines. */
	private async sendAgent(text: string, view: string) {
		const id = await agentStart(text, view || null);
		let next = 0;
		let changed = false;
		let failures = 0;
		const onEvent = (n: number, ev: AgentEvent) => {
			next = n + 1;
			if (ev.kind === 'text') {
				this.lines.push({ who: 'ai', text: ev.text });
			} else if (ev.kind === 'step') {
				changed = true;
				const action = {
					text: ev.text,
					entryId: null,
					section: null,
					url: null,
					navigate: false,
				};
				this.lines.push({ who: 'action', action });
			} else {
				this.error = ev.text;
			}
			this.onLine();
		};
		// a restart of the cms ends the stream, read on after it
		for (;;) {
			try {
				const err = await agentEvents(
					id,
					next,
					onEvent,
					this.abort.signal,
				);
				if (err && !this.lines.some(l => l.who === 'ai'))
					this.error = err;
				break;
			} catch (e) {
				// only a dropped stream is read again, an error of the api
				// or leaving the page is the end
				if (e instanceof ApiError || this.abort.signal.aborted) throw e;
				if (++failures > 120) throw e;
				await new Promise(r => setTimeout(r, 1000));
			}
		}
		return changed ? { refresh: true as const } : null;
	}

	/** Leaving the page: the stream stops. */
	private abort = new AbortController();

	stop() {
		this.abort.abort();
	}

	clear() {
		this.messages = [];
		this.lines = [];
		if (this.model === null)
			agentForget().catch(e => (this.error = e.message));
	}
}

/** The chat kept from before, nothing when it can't be read. */
function readSaved(): any {
	try {
		return JSON.parse(sessionStorage.getItem(KEY) ?? 'null');
	} catch {
		return null;
	}
}
