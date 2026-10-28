import type { Values } from '../api';

type Block = { id?: string; type: string; fields: Values };

function isBlocks(v: unknown): v is Block[] {
	return (
		Array.isArray(v) &&
		v.every(b => b && typeof b === 'object' && 'fields' in b)
	);
}

function isObject(v: unknown): v is Values {
	return !!v && typeof v === 'object' && !Array.isArray(v) && !('type' in v);
}

/**
 * The paths of the fields that differ, like the error paths: `intro`,
 * `body`, `body.2` for a new block, `body.2.content` inside a block.
 */
export function changedPaths(
	old: Values,
	now: Values,
	prefix = '',
	out = new Set<string>(),
): Set<string> {
	const keys = new Set([
		...Object.keys(old ?? {}),
		...Object.keys(now ?? {}),
	]);
	for (const key of keys) {
		const a = old?.[key];
		const b = now?.[key];
		if (JSON.stringify(a ?? null) === JSON.stringify(b ?? null)) continue;
		out.add(prefix + key);

		// a group: its fields
		if (isObject(a) && isObject(b)) {
			changedPaths(a, b, `${prefix}${key}.`, out);
			continue;
		}
		if (!isBlocks(b)) continue;
		const before = isBlocks(a) ? a : [];
		b.forEach((block, i) => {
			// the same block by its id, new blocks have none yet
			const was = block.id
				? before.find(x => x.id === block.id)
				: before[i];
			const path = `${prefix}${key}.${i}`;
			if (!was || was.type !== block.type) out.add(path);
			else changedPaths(was.fields, block.fields, path + '.', out);
		});
	}
	return out;
}
