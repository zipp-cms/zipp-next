import { request, token } from './client';

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

/** `view` is the page the preview shows. */
export function aiChat(
	model: string,
	messages: AiMessage[],
	view: string | null,
): Promise<{ messages: AiMessage[]; text: string; actions: AiAction[] }> {
	return request('POST', '/ai/chat', { model, messages, view });
}
