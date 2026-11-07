import type { Component } from '../../core/schema';
import type { RichOptions } from './editor';

/** A tool of the bar over a rich text, in its order. */
export type Tool =
	| {
			tool:
				| '|'
				| 'format'
				| 'styles'
				| 'bold'
				| 'italic'
				| 'bulletList'
				| 'orderedList'
				| 'link'
				| 'button';
	  }
	/** inserts a block of this type */
	| { tool: 'block'; type: Component }
	/** the Insert menu with what has no tool of its own */
	| { tool: 'insert'; button: boolean; types: Component[] };

/** The bar without a `toolbar` of the field: what it allows. */
const DEFAULT = [
	'format',
	'|',
	'styles',
	'|',
	'bold',
	'italic',
	'bulletList',
	'orderedList',
	'link',
	'|',
	'button',
	'|',
	'insert',
];

/**
 * The tools of a field's bar: its `toolbar` (the schema checked it) or
 * the default; buttons and blocks without a tool go into the Insert
 * menu, at the end unless the bar places it.
 */
export function toolsOf(opts: RichOptions, blockTypes: Component[]): Tool[] {
	const allowed: Record<string, boolean> = {
		format: opts.headings.length > 0,
		styles: !!opts.styles?.length,
		bulletList: opts.lists,
		orderedList: opts.lists,
		link: opts.links,
		button: opts.buttons,
	};
	let names = opts.toolbar?.length
		? opts.toolbar
		: DEFAULT.filter(n => allowed[n] ?? true);

	const button = opts.buttons && !names.includes('button');
	const types = blockTypes.filter(c => !names.includes(`block:${c.handle}`));
	const left = button || types.length > 0;
	if (!left) names = names.filter(n => n !== 'insert');
	else if (!names.includes('insert')) names = [...names, '|', 'insert'];

	const tools: Tool[] = [];
	for (const name of names) {
		// a line only between tools
		if (name === '|' && (!tools.length || tools.at(-1)!.tool === '|'))
			continue;

		const block = name.startsWith('block:')
			? blockTypes.find(c => `block:${c.handle}` === name)
			: undefined;
		if (block) tools.push({ tool: 'block', type: block });
		else if (name === 'insert')
			tools.push({ tool: 'insert', button, types });
		else if (!name.startsWith('block:'))
			tools.push({
				tool: name as Exclude<Tool['tool'], 'block' | 'insert'>,
			});
	}
	if (tools.at(-1)?.tool === '|') tools.pop();
	return tools;
}
