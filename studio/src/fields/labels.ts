import type { Field } from '../core/schema';

/**
 * Whether the field is one input with the id `path`, so its label is a
 * `<label for>`. The others (lists, groups, a date, a markdown with its
 * tabs) are a group the label names, `labelId`.
 */
export function hasControl(field: Field): boolean {
	if (field.kind === 'dropdown') return field.options.length > 3;
	return ['text', 'number', 'email', 'boolean'].includes(field.kind);
}

/** The id of the label of a field that is a group. */
export const labelId = (path: string) => path + '-label';
