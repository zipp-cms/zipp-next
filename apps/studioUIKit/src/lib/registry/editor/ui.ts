import { writable, type Writable } from 'svelte/store';
import type { Block } from './editor.ts';

type ContextMenuHandler = (block: Block, event: MouseEvent) => void;

export interface BlocksUI {
	selection: Selection;
	openContextMenu: (block: Block, event: MouseEvent) => void;
	onContextMenuOpen: (handler: ContextMenuHandler) => void;
}

export function createBlocksUI(): BlocksUI {
	const selection = createSelection();

	const contextMenuHandlers: ContextMenuHandler[] = [];

	function openContextMenu(block: Block, event: MouseEvent) {
		for (const handler of contextMenuHandlers) {
			handler(block, event);
		}
	}

	return {
		selection,
		openContextMenu,
		onContextMenuOpen(handler) {
			contextMenuHandlers.push(handler);
		}
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
