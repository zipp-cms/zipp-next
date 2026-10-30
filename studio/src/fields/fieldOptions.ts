import type { Field, Schema } from '../core/schema';
import { LINK_NAMES } from './link/link';

/**
 * All options of a field, set or not, for Setup: what the code says and
 * what else it could say. A value may link to another part of Setup.
 */
export type FieldOption = {
	name: string;
	value: string;
	links?: { text: string; href: string }[];
};

const yes = (b: boolean) => (b ? 'Yes' : 'No');
const or = (v: number | null, none = '–') => (v === null ? none : String(v));

/** Links to the entry types or blocks of a list of handles. */
function components(schema: Schema, handles: string[]) {
	return handles.map(h => {
		const c = schema.components.find(c => c.handle === h);
		const tab = c?.entry === false ? 'blocks' : 'types';
		return { text: c?.name ?? h, href: `/admin/setup/${tab}/${h}` };
	});
}

export function fieldOptions(
	schema: Schema,
	f: Field,
	entry: boolean,
): FieldOption[] {
	const out: FieldOption[] = [
		{ name: 'Required', value: yes(f.required) },
		...(entry
			? [
					{
						name: 'Sites',
						value: f.translatable
							? 'Per site, translated'
							: 'The same in all sites',
					},
				]
			: []),
		{
			name: 'Indexed',
			value: f.indexed
				? 'Yes, lists and GraphQL filter and sort by it'
				: 'No',
		},
	];
	const add = (name: string, value: string, links?: FieldOption['links']) =>
		out.push({ name, value, links });

	switch (f.kind) {
		case 'text':
			add('Lines', f.multiline ? 'Several' : 'One');
			add('Max length', or(f.max));
			break;
		case 'number':
			add('Min', or(f.min));
			add('Max', or(f.max));
			break;
		case 'date':
			add(
				'Value',
				f.time
					? 'A day and time, 2026-11-14T20:00'
					: 'A day, 2026-11-14',
			);
			break;
		case 'relation':
			add('Entry types', '', components(schema, f.types));
			add('Max entries', or(f.max, 'Any'));
			break;
		case 'assets':
			add(
				'Files',
				f.extensions?.length
					? f.extensions.map(e => e.toUpperCase()).join(', ')
					: f.images
						? 'Images only'
						: 'Any file',
			);
			add('Max files', or(f.max, 'Any'));
			add('Upload to', f.folder ?? 'Library');
			break;
		case 'link':
			add('Links to', f.to.map(k => LINK_NAMES[k]).join(', '));
			add('Label', yes(f.labels));
			add(
				'External links',
				f.newTab ? 'In a new tab' : 'In the same tab',
			);
			break;
		case 'navigation':
			add('Links to', f.to.map(k => LINK_NAMES[k]).join(', '));
			add('Levels', String(f.depth));
			break;
		case 'richText':
			add(
				'Headings',
				f.headings.length
					? f.headings.map(h => `h${h}`).join(', ')
					: 'No',
			);
			add('Lists', yes(f.lists));
			add('Links', yes(f.links));
			add('Buttons', yes(f.buttons));
			add(
				'Blocks in the text',
				f.blocks.length ? '' : 'No',
				components(schema, f.blocks),
			);
			break;
		case 'blocks':
			add('Block types', '', components(schema, f.types));
			add('Min blocks', or(f.min));
			add('Max blocks', or(f.max, 'Any'));
			break;
		case 'group':
			add('Fields of', '', components(schema, [f.component]));
			break;
		case 'dropdown':
		case 'checkboxes':
			add(
				'Options',
				f.options.map(o => `${o.label} (${o.value})`).join(', '),
			);
			break;
		case 'table':
			add(
				'Columns',
				f.columns.map(c => `${c.label} (${c.kind})`).join(', '),
			);
			break;
		case 'texts':
			add('Keys', f.keys.map(([k]) => k).join(', '));
			break;
		case 'custom':
			add('Field type', `${f.type}, of a plugin`);
			add('Options', JSON.stringify(f.options) ?? '–');
			break;
	}
	add('GraphQL', f.handle);
	return out;
}
