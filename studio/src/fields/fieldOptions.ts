import type { Component, Field, Schema } from '../core/schema';
import { describeCondition } from './conditions';
import { linkName } from './link/link';
import { t } from '../i18n/index.svelte';

/**
 * All options of a field, set or not, for Setup: what the code says and
 * what else it could say. A value may link to another part of Setup.
 */
export type FieldOption = {
	name: string;
	value: string;
	links?: { text: string; href: string }[];
};

const yes = (b: boolean) => (b ? t('common.yes') : t('common.no'));
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
	c: Component,
): FieldOption[] {
	const layout = [
		{
			name: t('fields.optWidth'),
			value: t('fields.widthOfRow', { n: f.width ?? 100 }),
		},
		...(f.when
			? [
					{
						name: t('fields.optShownWhen'),
						value: describeCondition(c, f.when),
					},
				]
			: []),
	];
	// a note only has its text
	if (f.kind === 'note')
		return [{ name: t('fields.optText'), value: f.text }, ...layout];

	const out: FieldOption[] = [
		...(f.instructions
			? [{ name: t('fields.optInstructions'), value: f.instructions }]
			: []),
		{ name: t('fields.optRequired'), value: yes(f.required) },
		...layout,
		...(c.entry
			? [
					{
						name: t('fields.optSites'),
						value: f.translatable
							? t('fields.perSite')
							: t('fields.sameInAllSites'),
					},
				]
			: []),
		{
			name: t('fields.optIndexed'),
			value: f.indexed ? t('fields.indexedYes') : t('common.no'),
		},
		...(f.default !== undefined
			? [
					{
						name: t('fields.optDefault'),
						value: JSON.stringify(f.default),
					},
				]
			: []),
	];
	const add = (name: string, value: string, links?: FieldOption['links']) =>
		out.push({ name, value, links });

	switch (f.kind) {
		case 'text':
			add(
				t('fields.optLines'),
				f.multiline ? t('fields.several') : t('fields.one'),
			);
			add(t('fields.optMaxLength'), or(f.max));
			break;
		case 'number':
			add(t('fields.optMin'), or(f.min));
			add(t('fields.optMax'), or(f.max));
			add(
				t('fields.optDecimals'),
				or(f.decimals ?? null, t('fields.any')),
			);
			add(t('fields.optUnit'), f.unit ?? '–');
			break;
		case 'date':
			add(
				t('fields.optValue'),
				f.time ? t('fields.dayAndTime') : t('fields.day'),
			);
			break;
		case 'relation':
			add(t('fields.optEntryTypes'), '', components(schema, f.types));
			add(t('fields.optMaxEntries'), or(f.max, t('fields.any')));
			break;
		case 'assets':
			add(
				t('fields.optFiles'),
				f.extensions?.length
					? f.extensions.map(e => e.toUpperCase()).join(', ')
					: f.images
						? t('fields.imagesOnly')
						: t('fields.anyFile'),
			);
			add(t('fields.optMaxFiles'), or(f.max, t('fields.any')));
			add(t('fields.optUploadTo'), f.folder ?? t('fields.library'));
			break;
		case 'link':
			add(t('fields.optLinksTo'), f.to.map(linkName).join(', '));
			add(t('fields.label'), yes(f.labels));
			add(
				t('fields.optExternalLinks'),
				f.newTab ? t('fields.inNewTab') : t('fields.inSameTab'),
			);
			break;
		case 'navigation':
			add(t('fields.optLinksTo'), f.to.map(linkName).join(', '));
			add(t('fields.optLevels'), String(f.depth));
			break;
		case 'richText':
			add(
				t('fields.optHeadings'),
				f.headings.length
					? f.headings.map(h => `h${h}`).join(', ')
					: t('common.no'),
			);
			add(t('fields.optLists'), yes(f.lists));
			add(t('fields.optLinks'), yes(f.links));
			add(t('fields.optButtons'), yes(f.buttons));
			add(
				t('fields.optBlocksInText'),
				f.blocks.length ? '' : t('common.no'),
				components(schema, f.blocks),
			);
			add(
				t('fields.styles'),
				f.styles?.length
					? f.styles
							.map(
								st =>
									`${st.label} (${st.inline ? t('fields.styleText') : t('fields.styleParagraph')}, .${st.class})`,
							)
							.join(', ')
					: t('common.no'),
			);
			break;
		case 'blocks':
			add(t('fields.optBlockTypes'), '', components(schema, f.types));
			add(t('fields.optMinBlocks'), or(f.min));
			add(t('fields.optMaxBlocks'), or(f.max, t('fields.any')));
			break;
		case 'group':
			add(t('fields.optFieldsOf'), '', components(schema, [f.component]));
			break;
		case 'dropdown':
		case 'checkboxes':
			add(
				t('fields.optOptions'),
				f.options.map(o => `${o.label} (${o.value})`).join(', '),
			);
			break;
		case 'table':
			add(
				t('fields.optColumns'),
				f.columns.map(c => `${c.label} (${c.kind})`).join(', '),
			);
			break;
		case 'texts':
			add(t('fields.optKeys'), f.keys.map(([k]) => k).join(', '));
			break;
		case 'custom':
			add(
				t('fields.optFieldType'),
				t('fields.ofPlugin', { type: f.type }),
			);
			add(t('fields.optOptions'), JSON.stringify(f.options) ?? '–');
			break;
	}
	add('GraphQL', f.handle);
	return out;
}
