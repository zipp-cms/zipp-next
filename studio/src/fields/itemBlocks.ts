import { mount, unmount } from 'svelte';
import type { ItemBlocks } from 'zipp-ui/field';
import { componentOf, defaults } from '../core/schema';
import type { FieldOf, Schema, Values } from '../core/schema';
import type { Refs } from '../entries/api';
import { newBlockId, withNewIds } from './blocks.svelte';
import ItemBlock from './ItemBlock.svelte';

/** What a custom field's editor shows its blocks with, read when used. */
export type Source = {
	schema: Schema;
	field: FieldOf<'custom'>;
	values: Values;
	errors: Record<string, string>;
	refs: Refs;
	path: string;
	changed?: Set<string>;
};

/**
 * What the studio lends the module of a field whose items hold blocks:
 * their types, new ones and the studio's editor of one, mounted with the
 * context of the field (the entry for `{slug}` folders, the site).
 */
export function itemBlocks(
	src: Source,
	context: Map<unknown, unknown>,
): ItemBlocks {
	const { schema, field } = src;
	return {
		types: (field.blocks ?? []).map(handle => {
			const c = componentOf(schema, handle);
			return { handle, name: c?.name ?? handle, icon: c?.icon ?? null };
		}),
		create: type => ({
			id: newBlockId(),
			type,
			fields: defaults(schema, type),
		}),
		copy(block) {
			// a plain copy, also of a block in the module's state
			const b = JSON.parse(JSON.stringify(block));
			withNewIds(schema, b);
			return b;
		},
		mount(target, id) {
			const c = mount(ItemBlock, { target, context, props: { src, id } });
			return { destroy: () => unmount(c) };
		},
	};
}
