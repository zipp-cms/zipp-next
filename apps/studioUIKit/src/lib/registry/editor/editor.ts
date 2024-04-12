import { writable } from 'svelte/store';

type Component = {
	name: string;
	handle: string;
	fields: {
		[key: string]: Field;
	};
};

type Field = NumberField | TextField | BooleanField | ComponentField;

// define fields based on kind
type NumberField = {
	kind: 'number';
	settings: {
		min?: number;
		max?: number;
	};
};

type TextField = {
	kind: 'text';
	settings: {
		max_length?: number;
	};
};

type BooleanField = {
	kind: 'boolean';
};

type ComponentField = {
	kind: 'component';
	settings: {
		component: string[];
		min?: number;
		max?: number;
	};
};

export type { Component, Field, NumberField, TextField, BooleanField, ComponentField };

type Block = Field & {
	name: string;
	belongsTo: string; // handle of the component this block belongs to
	subComponentOptions: string[]; // list of allowed sub-components
	level: number; // level of nesting
};

export function componentContext(components: Component[], rootHandle: string) {
	const componentMap = new Map(components.map((component) => [component.handle, component]));

	const blocks = writable(initBlocks());

	function getComponent(handle: string): Component | null {
		return componentMap.get(handle) ?? null;
	}

	function initBlocks() {
		return toBlocks(getComponent(rootHandle));
	}

	function toBlocks(component: Component | null, level = 0): Block[] {
		if (component === null) {
			return [];
		}

		return Object.entries(component.fields).map(([key, field]) => {
			const subComponentOptions = field.kind === 'component' ? field.settings.component : [];
			return { ...field, name: key, belongsTo: component.handle, subComponentOptions, level };
		});
	}

	function choose(parent: Block, child: string) {
		console.log('choosing', parent, child);

		blocks.update((bs) => {
			const updated: Block[] = [];

			bs.forEach((block) => {
				updated.push(block);

				if (block.name === parent.name) {
					const component = getComponent(child);
					updated.push(...toBlocks(component, block.level + 1));
				}
			});

			return updated;
		});
	}

	return {
		blocks,
		choose
	};
}

// // dfs components traversal, returning a list of all visited components
// function dfs(root: string, components: Map<string, Component>): Component[] {
// 	const visited: Component[] = [];
// 	const stack = [root];

// 	while (stack.length > 0) {
// 		const current = stack.pop();
// 		if (current === undefined) {
// 			continue;
// 		}

// 		const component = components.get(current);
// 		if (component === undefined) {
// 			continue;
// 		}

// 		visited.push(component);

// 		for (const field of Object.values(component.fields)) {
// 			if (field.kind === 'component') {
// 				stack.push(field.settings.component);
// 			}
// 		}
// 	}

// 	return visited;
// }
