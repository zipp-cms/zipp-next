import { writable, type Writable } from 'svelte/store';
import type { Block } from './editor.ts';

export interface BlocksUI {
	selection: Selection;
	openContextMenu: (block: Block, event: MouseEvent) => void;
}

export function createBlocksUI(): BlocksUI {
	const selection = createSelection();

	function openContextMenu(block: Block, event: MouseEvent) {
		console.log('openContextMenu', block, event);

		// do something
	}

	return {
		selection,
		openContextMenu
	};
}

interface Selection {
	blocks: Writable<string[]>;
	activeBlock: Writable<string | null>;
}

export function createSelection(): Selection {
	const blocks = writable<string[]>([]);
	const activeBlock = writable(null);

	return {
		blocks,
		activeBlock
	};
}
