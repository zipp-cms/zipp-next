import {
	componentOf,
	type Block,
	type Component,
	type Condition,
	type Schema,
	type Tab,
	type Values,
} from '../core/schema';
import { isEmpty } from './link/link';

/** The conditions of a field: its own and the one of its tab. */
function conditionsOf(c: Component, handle: string): Condition[] {
	const own = c.fields.find(f => f.handle === handle)?.when;
	const tab = c.tabs?.find(t => t.fields.includes(handle))?.when;
	return [own, tab].filter(w => w !== undefined);
}

function holds(c: Component, cond: Condition, values: Values): boolean {
	const field = c.fields.find(f => f.handle === cond.field);
	if (!field) return true;
	const v = values[cond.field];
	switch (cond.test) {
		case 'oneOf': {
			// a boolean that was never set is off
			const value = field.kind === 'boolean' && v == null ? false : v;
			return cond.values.includes(value);
		}
		case 'filled':
			return !isEmpty(field.kind, v);
		case 'empty':
			return isEmpty(field.kind, v);
	}
}

/**
 * Whether a field shows with these values, like the server says: its
 * conditions hold and the fields they look at show too.
 */
export function isShown(c: Component, handle: string, values: Values): boolean {
	return conditionsOf(c, handle).every(
		w => holds(c, w, values) && isShown(c, w.field, values),
	);
}

/** Whether a tab shows: its condition holds for a field that shows. */
export function tabShown(c: Component, tab: Tab, values: Values): boolean {
	const w = tab.when;
	return !w || (holds(c, w, values) && isShown(c, w.field, values));
}

/**
 * Whether the field of a path like `body.0.text` shows, through blocks
 * and groups; the errors of hidden fields don't show.
 */
export function pathShown(
	schema: Schema,
	c: Component,
	values: Values,
	path: string,
): boolean {
	const [handle, ...rest] = path.split('.');
	if (!isShown(c, handle, values)) return false;
	const field = c.fields.find(f => f.handle === handle);
	const v = values[handle];
	if (field?.kind === 'group' && rest.length) {
		const g = componentOf(schema, field.component);
		return !g || pathShown(schema, g, (v ?? {}) as Values, rest.join('.'));
	}
	if (field?.kind === 'blocks' && rest.length > 1) {
		const b = (v as Block[] | undefined)?.[+rest[0]];
		const bc = b && componentOf(schema, b.type);
		return !bc || pathShown(schema, bc, b.fields, rest.slice(1).join('.'));
	}
	// `grid.0.block.image`: the block of an item
	if (field?.kind === 'custom' && field.blocks && rest.length > 2) {
		const b = (v as { block?: Block }[] | undefined)?.[+rest[0]]?.block;
		const bc = b && componentOf(schema, b.type);
		return !bc || pathShown(schema, bc, b.fields, rest.slice(2).join('.'));
	}
	return true;
}
