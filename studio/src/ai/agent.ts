import { request, stream } from '../core/client';

/**
 * The coding agent of the plugin `agent` (development): a run per
 * message, its events come as the agent works.
 */

/** What the chat shows of a run. */
export type AgentEvent = {
	kind: 'text' | 'step' | 'error';
	text: string;
};

/** Starts a run with a message, `view` is the page the preview shows. */
export async function agentStart(
	message: string,
	view: string | null,
): Promise<string> {
	return (await request('POST', '/agent', { message, view })).id;
}

/** The next conversation starts new. */
export function agentForget(): Promise<void> {
	return request('DELETE', '/agent/session');
}

/**
 * Reads the events of a run from line `from` on, `line` gets each with
 * the number of its line; resolves with the agent's errors when the run is done. A
 * dropped connection (the cms restarts after a change of its code)
 * throws, read again from the next line.
 */
export async function agentEvents(
	id: string,
	from: number,
	line: (n: number, event: AgentEvent) => void,
	signal?: AbortSignal,
): Promise<string> {
	const path = `/agent/${id}?from=${from}`;
	for await (const ev of stream('GET', path, undefined, signal)) {
		if (ev.event === 'done') return ev.data;
		let event: AgentEvent;
		try {
			event = JSON.parse(ev.data);
		} catch {
			// not an event
			continue;
		}
		line(Number(ev.id), event);
	}
	throw new Error('reconnect');
}
