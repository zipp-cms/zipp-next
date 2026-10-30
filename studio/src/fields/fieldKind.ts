import type { Field } from '../core/schema';

/** A field's kind and settings in a few words, for the content model. */
export function describeField(f: Field): string {
	const list = (xs: string[]) => xs.join(', ');
	switch (f.kind) {
		case 'text':
			return [
				f.multiline ? 'Text, several lines' : 'Text',
				f.max ? `max ${f.max}` : '',
			]
				.filter(Boolean)
				.join(', ');
		case 'number':
			return f.min !== null || f.max !== null
				? `Number ${f.min ?? ''}–${f.max ?? ''}`
				: 'Number';
		case 'boolean':
			return 'Yes / no';
		case 'relation':
			return `Entries: ${list(f.types)}${f.max ? `, max ${f.max}` : ''}`;
		case 'assets':
			return `${f.images ? 'Images' : 'Files'}${f.max ? `, max ${f.max}` : ''}`;
		case 'link':
			return `Link to ${list(f.to)}${f.newTab ? '' : ', same tab'}`;
		case 'richText': {
			const parts = [
				f.headings.length ? `h${f.headings.join(', h')}` : '',
				f.lists ? 'lists' : '',
				f.links ? 'links' : '',
				f.buttons ? 'buttons' : '',
				f.blocks.length ? `blocks: ${list(f.blocks)}` : '',
			].filter(Boolean);
			return `Rich text${parts.length ? ': ' + parts.join(', ') : ''}`;
		}
		case 'group':
			return `Group of ${f.component}`;
		case 'markdown':
			return 'Markdown';
		case 'date':
			return f.time ? 'Date and time' : 'Date';
		case 'checkboxes':
			return `Checkboxes: ${list(f.options.map(o => o.value))}`;
		case 'table':
			return `Table: ${list(f.columns.map(c => c.handle))}`;
		case 'navigation':
			return `Navigation to ${list(f.to)}, ${f.depth} levels`;
		case 'dropdown':
			return `Dropdown: ${list(f.options.map(o => o.value))}`;
		case 'texts':
			return `Texts: ${f.keys.length} keys`;
		case 'blocks': {
			const n = [f.min ? `min ${f.min}` : '', f.max ? `max ${f.max}` : '']
				.filter(Boolean)
				.join(', ');
			return `Blocks: ${list(f.types)}${n ? ', ' + n : ''}`;
		}
		case 'custom':
			return `${f.type} (plugin)`;
	}
}
