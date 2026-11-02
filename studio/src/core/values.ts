import type { Block, Values } from './schema';

/**
 * Field values as plain JSON: what is a list of blocks, a group, and the
 * values without the empty ones, as the server keeps them.
 */

export function isBlocks(v: unknown): v is Block[] {
	return (
		Array.isArray(v) &&
		v.every(b => b && typeof b === 'object' && 'fields' in b)
	);
}

/** The values of a group; a link is an object too, with its `type`. */
export function isObject(v: unknown): v is Values {
	return !!v && typeof v === 'object' && !Array.isArray(v) && !('type' in v);
}

/**
 * A value as publishing keeps it: empty texts, lists and groups are
 * dropped, also inside groups and blocks, so `{}` in a draft is the same
 * as nothing live. Null when nothing is left.
 */
export function clean(v: unknown): unknown {
	if (v === undefined || v === null || v === '') return null;
	if (!Array.isArray(v)) return cleanValue(v);
	if (!v.length) return null;
	// blocks: their fields like the entry's, also in a draft from before
	if (isBlocks(v))
		return v.map(b => ({ ...b, fields: clean(b.fields) ?? {} }));
	return v;
}

function cleanValue(v: unknown): unknown {
	if (isObject(v)) {
		const out: Values = {};
		for (const [k, x] of Object.entries(v)) {
			const c = clean(x);
			if (c !== null) out[k] = c;
		}
		return Object.keys(out).length ? out : null;
	}
	return v;
}
