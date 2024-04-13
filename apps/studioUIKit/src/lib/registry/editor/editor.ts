import { get, writable, type Writable } from 'svelte/store';

type Component = {
	name: string;
	handle: string;
	fields: {
		[key: string]: Field;
	};
};

type Field = NumberField | TextField | BooleanField | ComponentField | RelationField;

type BaseField = {
	kind: string;
	settings: Record<string, unknown>;
};

// define fields based on kind
type NumberField = BaseField & {
	kind: 'number';
	settings: {
		min?: number;
		max?: number;
	};
};

type TextField = BaseField & {
	kind: 'text';
	settings: {
		max_length?: number;
	};
};

type BooleanField = BaseField & {
	kind: 'boolean';
};

type ComponentField = BaseField & {
	kind: 'component';
	settings: {
		component: string[];
		min?: number;
		max?: number;
	};
};

type RelationField = BaseField & {
	kind: 'relation';
	settings: {
		Entity: string[];
		min?: number;
		max?: number;
	};
};

export type {
	Component,
	Field,
	NumberField,
	TextField,
	BooleanField,
	ComponentField,
	RelationField
};

export type BlockOf<T extends BaseField> = T & {
	id: string;
	name: string;
	belongsTo: string; // handle of the component this block belongs to
	subComponentOptions: string[]; // list of allowed sub-components
	level: number; // level of nesting
	properties: Record<string, unknown>;
	content: string[];
};
export type Block = BlockOf<Field>;

export interface ComponentContext {
	choose: (parent: Block, child: string) => void;
	setProperty: (block: Block, key: string, value: unknown) => void;
	blocks: Writable<Map<string, Block>>;
	rootBlocks: Writable<string[]>;
}

export function componentContext(components: Component[], rootHandle: string): ComponentContext {
	const componentMap = new Map(components.map((component) => [component.handle, component]));

	const blockMap = new Map<string, Block>();
	initBlocks();
	const blocks = writable(blockMap);
	let rootBlocks: Writable<string[]> = writable(Array.from(blockMap.keys()));

	function getComponent(handle: string): Component | null {
		return componentMap.get(handle) ?? null;
	}

	function initBlocks() {
		const blocks = toBlocks(getComponent(rootHandle));
		blocks.forEach((block) => blockMap.set(block.id, block));
		return blocks;
	}

	function toBlocks(component: Component | null, level = 0): Block[] {
		if (component === null) {
			return [];
		}

		return Object.entries(component.fields).map(([key, field]) => {
			const subComponentOptions = field.kind === 'component' ? field.settings.component : [];
			return {
				...field,
				name: key,
				belongsTo: component.handle,
				subComponentOptions,
				level,
				id: Math.random().toString(),
				properties: {},
				content: []
			};
		});
	}

	function setProperty(block: Block, key: string, value: unknown) {
		blocks.update((bs) => {
			const updated = bs.map((b) => {
				if (b === block) {
					return {
						...b,
						properties: {
							...b.properties,
							[key]: value
						}
					};
				}
				blockMap.set(b.id, b);
				return b;
			});

			return updated;
		});
	}

	function choose(parent: Block, child: string) {
		console.log('choosing', parent, child);

		const component = getComponent(child);
		const blocks = toBlocks(component, parent.level + 1);

		parent.content.push(...blocks.map((b) => b.id));

		blockMap.set(parent.id, parent);
		blocks.forEach((block) => blockMap.set(block.id, block));

		if (blocks[0].belongsTo === rootHandle) {
			rootBlocks.update((r) => {
				r.push(...blocks.map((b) => b.id));
				return r;
			});
		}

		rebuildBlocks();
	}

	function rebuildBlocks() {
		blocks.set(new Map(blockMap));
	}

	// function dfs(roots: string[], cb: (block: Block) => void) {
	// 	const stack = [...roots];

	// 	while (stack.length > 0) {
	// 		const current = stack.shift();
	// 		if (current === undefined) {
	// 			continue;
	// 		}

	// 		const block = blockMap.get(current);
	// 		if (block === undefined) {
	// 			continue;
	// 		}

	// 		cb(block);

	// 		for (const child of block.content) {
	// 			stack.push(child);
	// 			// stack.unshift(child);
	// 		}
	// 	}
	// }

	return {
		rootBlocks,
		blocks,
		choose,
		setProperty
	};
}
