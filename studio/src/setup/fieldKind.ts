import type { Field } from '../core/schema';
import { onlyVideos } from '../assets/api';
import { t } from '../i18n/index.svelte';

/** A field's kind and settings in a few words, for the content model. */
export function describeField(f: Field): string {
	const list = (xs: string[]) => xs.join(', ');
	const max = (n?: number | null) => (n ? `, ${t('fields.max', { n })}` : '');
	// the limit of entries and links to entries
	const inSections = (s?: string[]) =>
		s?.length ? t('fields.inSections', { sections: list(s) }) : '';
	const only = (s?: string[]) => (s?.length ? `, ${inSections(s)}` : '');
	switch (f.kind) {
		case 'text':
			return [
				f.multiline ? t('fields.kindTextLines') : t('fields.kindText'),
				f.max ? t('fields.max', { n: f.max }) : '',
			]
				.filter(Boolean)
				.join(', ');
		case 'number':
			return [
				f.min !== null || f.max !== null
					? t('fields.kindNumberRange', {
							min: f.min ?? '',
							max: f.max ?? '',
						})
					: t('fields.kindNumber'),
				f.decimals === 0
					? t('fields.whole')
					: f.decimals !== undefined
						? t('fields.decimals', { n: f.decimals })
						: '',
				f.unit ?? '',
			]
				.filter(Boolean)
				.join(', ');
		case 'boolean':
			return t('fields.kindBoolean');
		case 'email':
			return t('fields.kindEmail');
		case 'relation':
			return (
				t('fields.kindRelation', { types: list(f.types) }) +
				only(f.sections) +
				max(f.max)
			);
		case 'assets':
			return (
				t(
					f.images
						? 'fields.kindImages'
						: onlyVideos(f.extensions)
							? 'fields.kindVideos'
							: 'fields.kindFiles',
				) + max(f.max)
			);
		case 'link':
			return (
				t('fields.kindLink', { to: list(f.to) }) +
				only(f.sections) +
				(f.newTab ? '' : `, ${t('fields.sameTab')}`)
			);
		case 'richText': {
			const parts = [
				f.headings.length ? `h${f.headings.join(', h')}` : '',
				f.lists ? t('fields.lists') : '',
				f.links ? t('fields.links') : '',
				f.buttons ? t('fields.buttons') : '',
				inSections(f.sections),
				f.blocks.length
					? t('fields.blocksOf', { types: list(f.blocks) })
					: '',
			].filter(Boolean);
			return parts.length
				? t('fields.kindRichTextWith', { parts: parts.join(', ') })
				: t('fields.kindRichText');
		}
		case 'group':
			return t('fields.kindGroup', { component: f.component });
		case 'markdown':
			return 'Markdown';
		case 'date':
			return f.time ? t('fields.kindDateTime') : t('fields.kindDate');
		case 'checkboxes':
			return t('fields.kindCheckboxes', {
				options: list(f.options.map(o => o.value)),
			});
		case 'table':
			return t('fields.kindTable', {
				columns: list(f.columns.map(c => c.handle)),
			});
		case 'navigation':
			return (
				t('fields.kindNavigation', { to: list(f.to), n: f.depth }) +
				only(f.sections)
			);
		case 'dropdown':
			return t('fields.kindDropdown', {
				options: list(f.options.map(o => o.value)),
			});
		case 'texts':
			return t('fields.kindTexts', { n: f.keys.length });
		case 'blocks': {
			const n = [
				f.min ? t('fields.min', { n: f.min }) : '',
				f.max ? t('fields.max', { n: f.max }) : '',
			]
				.filter(Boolean)
				.join(', ');
			return (
				t('fields.kindBlocks', { types: list(f.types) }) +
				(n ? ', ' + n : '')
			);
		}
		case 'custom':
			return t('fields.kindCustom', { type: f.type });
		case 'note':
			return t('fields.kindNote');
	}
}
