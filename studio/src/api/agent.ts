import { ApiError, request, token } from './client';

/**
 * The coding agent of development (`serve --agent`): a run per message,
 * its output are the JSON lines of the agent (Claude Code's stream-json).
 */
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
 * Reads the lines of a run from `from` on, `line` gets each with its
 * number; resolves with the agent's errors when the run is done. A
 * dropped connection (the cms restarts after a change of its code)
 * throws, read again from the next line.
 */
export async function agentEvents(
	id: string,
	from: number,
	line: (n: number, data: any) => void,
): Promise<string> {
	const resp = await fetch(`/api/agent/${id}?from=${from}`, {
		headers: { 'session-token': token() ?? '' },
	});
	if (!resp.ok || !resp.body) {
		const data = await resp.json().catch(() => null);
		throw new ApiError(resp.status, data?.message ?? resp.statusText, []);
	}
	const reader = resp.body.pipeThrough(new TextDecoderStream()).getReader();
	let buf = '';
	for (;;) {
		const { value, done } = await reader.read();
		if (done) throw new Error('reconnect');
		buf += value;
		let end;
		while ((end = buf.indexOf('\n\n')) >= 0) {
			const block = buf.slice(0, end).split('\n');
			buf = buf.slice(end + 2);
			const field = (name: string) =>
				block
					.filter(l => l.startsWith(name + ':'))
					.map(l => l.slice(name.length + 1).trimStart())
					.join('\n');
			const data = field('data');
			if (field('event') === 'done') return data;
			if (!data) continue;
			try {
				line(Number(field('id')), JSON.parse(data));
			} catch {
				// not JSON, like a warning of the agent
			}
		}
	}
}
