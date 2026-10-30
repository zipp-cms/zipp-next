import { SEARCH_DELAY_MS } from 'zipp-ui/SearchInput.svelte';
import {
	ROOT,
	folderRows,
	listAssets,
	listFolders,
	type Asset,
	type Folder,
} from './api';

/**
 * What the Assets page and the picker show of the library: its folders,
 * the open one (null: the top), a search that looks in all of them, and
 * the assets of that. A response that came after a newer request is
 * dropped.
 */
export class Library {
	folders = $state<Folder[]>([]);
	/** the assets in no folder */
	top = $state(0);
	folder = $state<number | null>(null);
	search = $state('');
	assets = $state<Asset[]>([]);
	loaded = $state(false);
	error = $state('');
	private wait: ReturnType<typeof setTimeout> | undefined;
	private asked = 0;

	/** only images, only files that end so */
	constructor(
		private only: { images?: boolean; extensions?: string[] } = {},
	) {}

	/** `Library / Photos / Events` */
	path = $derived(
		[
			ROOT,
			folderRows(this.folders).find(r => r.folder.id === this.folder)
				?.path,
		]
			.filter(Boolean)
			.join(' / '),
	);

	/** What an empty list says. */
	empty = $derived(
		this.search
			? 'Nothing found'
			: this.folder === null
				? 'No assets yet'
				: 'No files in this folder',
	);

	/** The assets of the open folder, of all with a search. */
	async load() {
		const n = ++this.asked;
		try {
			const assets = await listAssets({
				search: this.search,
				folder: this.search ? undefined : this.folder,
				...this.only,
			});
			if (n !== this.asked) return;
			this.assets = assets;
			this.loaded = true;
		} catch (e: any) {
			this.error = e.message;
		}
	}

	/** The folders with their counts. */
	async loadFolders() {
		try {
			const { folders, top } = await listFolders();
			this.folders = folders;
			this.top = top;
		} catch (e: any) {
			this.error = e.message;
		}
	}

	/** Another folder, the search ends. */
	select(id: number | null) {
		clearTimeout(this.wait);
		this.folder = id;
		this.search = '';
		this.load();
	}

	/** What was typed, searched a moment after the last key. */
	type(search: string) {
		this.search = search;
		clearTimeout(this.wait);
		this.wait = setTimeout(() => this.load(), SEARCH_DELAY_MS);
	}
}
