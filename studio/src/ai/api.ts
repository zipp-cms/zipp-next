import { stream } from '../core/client';

/** A message of the AI chat, the studio keeps the conversation. */
export type AiMessage =
	| { kind: 'user'; text: string }
	| { kind: 'assistant'; text: string }
	| { kind: 'toolCall'; id: string; name: string; input: unknown }
	| { kind: 'toolOutput'; id: string; output: string };

/** A draft the AI changed, with a link that previews it. */
export type AiAction = {
	text: string;
	entryId: number | null;
	section: string | null;
	/** a page of the website, without a token */
	url: string | null;
	/** whether the preview opens `url`, else it reloads its page */
	navigate: boolean;
};

/** What the chat streams while the model works. */
export type AiEvents = {
	/** more text of the answer */
	delta: (text: string) => void;
	/** a tool did something */
	action: (action: AiAction) => void;
};

/**
 * Sends the conversation, `view` is the page the preview shows. The
 * answer streams to `on`; resolves with the messages to append.
 */
export async function aiChat(
	model: string,
	messages: AiMessage[],
	view: string | null,
	on: AiEvents,
	signal?: AbortSignal,
): Promise<AiMessage[]> {
	const body = { model, messages, view };
	for await (const { data } of stream('POST', '/ai/chat', body, signal)) {
		if (!data) continue;
		const ev = JSON.parse(data);
		if (ev.type === 'delta') on.delta(ev.text);
		else if (ev.type === 'action') on.action(ev.action);
		else if (ev.type === 'error') throw new Error(ev.message);
		else if (ev.type === 'done') return ev.messages;
	}
	throw new Error('The answer ended early');
}
