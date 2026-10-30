import type { Component, Schema, Section } from '../api';

/** Lookups in the schema by handle. */

export function componentOf(
	schema: Schema,
	handle: string,
): Component | undefined {
	return schema.components.find(c => c.handle === handle);
}

/** The name of an entry type or block, its handle if it is unknown. */
export function componentName(schema: Schema, handle: string): string {
	return componentOf(schema, handle)?.name ?? handle;
}

export function sectionOf(schema: Schema, handle: string): Section | undefined {
	return schema.sections.find(s => s.handle === handle);
}

export function sectionName(schema: Schema, handle: string): string {
	return sectionOf(schema, handle)?.name ?? handle;
}
