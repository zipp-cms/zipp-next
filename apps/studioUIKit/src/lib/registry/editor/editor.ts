import { get, writable, type Writable } from 'svelte/store';
import { createBlocksUI, type BlocksUI } from './ui.ts';

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

export type BlockOf<T extends BaseField | Component> = T & {
	id: string;
	name: string;
	type: 'component' | 'field';
	level: number; // level of nesting
	properties: Record<string, unknown>;
	content: string[];
	parent: string;
};
export type Block = BlockOf<Field | Component>;

export interface ComponentContext {
	choose: (parent: Block, child: string) => void;
	setProperty: (block: Block, key: string, value: unknown) => void;
	blocks: Writable<Map<string, Block>>;
	rootBlock: Writable<string>;
	ui: BlocksUI;
}

let id = 0;

export function componentContext(components: Component[], rootHandle: string): ComponentContext {
	const componentMap = new Map(components.map((component) => [component.handle, component]));

	const blockMap = new Map<string, Block>();
	const blocks = writable(blockMap);
	let rootBlock: Writable<string> = writable(initBlocks()[0].id);

	function getComponent(handle: string): Component | null {
		return componentMap.get(handle) ?? null;
	}

	function initBlocks() {
		const blocks = toBlocks(getComponent(rootHandle));
		blocks.forEach((block) => blockMap.set(block.id, block));
		return blocks;
	}

	function generateBlockId(): string {
		return id++ + '';
	}

	function toBlocks(component: Component | null, level = 0): Block[] {
		if (component === null) {
			return [];
		}

		const componentId = generateBlockId();
		const fieldBlocks = Object.entries(component.fields).map(([key, field]) => {
			return {
				...field,
				type: 'field',
				name: key,

				level,
				id: generateBlockId(),
				properties: {},
				content: [],
				parent: componentId
			};
		});

		const componentBlock: BlockOf<Component> = {
			...component,
			type: 'component',
			id: componentId,
			level,
			properties: {},
			parent: '',
			content: [...fieldBlocks.map((b) => b.id)]
		};
		return [componentBlock, ...fieldBlocks];
	}

	function setProperty(block: Block, key: string, value: unknown) {
		blockMap.set(block.id, {
			...block,
			properties: {
				...block.properties,
				[key]: value
			}
		});
		rebuildBlocks();
	}

	function choose(parent: Block, child: string) {
		console.log('choosing', parent, child);

		const component = getComponent(child);
		const blocks = toBlocks(component, parent.level + 1);

		parent.content.push(blocks[0].id);
		blocks[0].parent = parent.id;

		blockMap.set(parent.id, parent);
		blocks.forEach((block) => blockMap.set(block.id, block));

		// if (blocks[0].parent === rootHandle) {
		// 	rootBlocks.update((r) => {
		// 		r.push(...blocks.map((b) => b.id));
		// 		return r;
		// 	});
		// }

		rebuildBlocks();
	}

	function rebuildBlocks() {
		blocks.set(new Map(blockMap));
	}

	const ui = createBlocksUI();

	return {
		rootBlock,
		blocks,
		choose,
		setProperty,
		ui
	};
}
