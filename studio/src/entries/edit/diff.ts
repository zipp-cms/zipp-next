import type { Values } from '../../core/schema';
import { clean, isBlocks, isObject } from '../../core/values';

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
		if (JSON.stringify(clean(a)) === JSON.stringify(clean(b))) continue;
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
