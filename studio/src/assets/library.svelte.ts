import { SEARCH_DELAY_MS } from 'zipp-ui/SearchInput.svelte';
import { t } from '../i18n/index.svelte';
import {
	root,
	folderRows,
	listAssets,
	listFolders,
	type Asset,
	type AssetKind,
	type Folder,
} from './api';

/**
 * What the Assets page and the picker show of the library: its folders,
 * the open one (null: the top), a search that looks in all of them, a
 * type (images, videos, documents) and the assets of that. A response
 * that came after a newer request is dropped, an error shows until the
 * next answer of its kind.
 */
export class Library {
	folders = $state<Folder[]>([]);
	/** the assets in no folder */
	top = $state(0);
	folder = $state<number | null>(null);
	search = $state('');
	/** empty: all types */
	kind = $state<AssetKind | ''>('');
	assets = $state<Asset[]>([]);
	loaded = $state(false);
	private assetsError = $state('');
	private foldersError = $state('');
	error = $derived(this.assetsError || this.foldersError);
	private wait: ReturnType<typeof setTimeout> | undefined;
	private asked = 0;
	private askedFolders = 0;

	/** only images, only files that end so */
	constructor(
		private only: { images?: boolean; extensions?: string[] } = {},
	) {}

	/** `Library / Photos / Events` */
	path = $derived(
		[
			root(),
			folderRows(this.folders).find(r => r.folder.id === this.folder)
				?.path,
		]
			.filter(Boolean)
			.join(' / '),
	);

	/** What an empty list says. */
	empty = $derived(
		this.search || this.kind
			? t('common.nothingFound')
			: this.folder === null
				? t('assets.noAssets')
				: t('assets.noFilesInFolder'),
	);

	/** The assets of the open folder, of all with a search. */
	async load() {
		const n = ++this.asked;
		try {
			const assets = await listAssets({
				search: this.search,
				folder: this.search ? undefined : this.folder,
				kind: this.kind || undefined,
				...this.only,
			});
			if (n !== this.asked) return;
			this.assets = assets;
			this.loaded = true;
			this.assetsError = '';
		} catch (e: any) {
			if (n === this.asked) this.assetsError = e.message;
		}
	}

	/** The folders with their counts. */
	async loadFolders() {
		const n = ++this.askedFolders;
		try {
			const { folders, top } = await listFolders();
			if (n !== this.askedFolders) return;
			this.folders = folders;
			this.top = top;
			this.foldersError = '';
		} catch (e: any) {
			if (n === this.askedFolders) this.foldersError = e.message;
		}
	}

	/** Another folder, the search ends. */
	select(id: number | null) {
		clearTimeout(this.wait);
		this.folder = id;
		this.search = '';
		this.load();
	}

	/** Only assets of a type, or all with none. */
	filter(kind: AssetKind | '') {
		this.kind = kind;
		this.load();
	}

	/** What was typed, searched a moment after the last key. */
	type(search: string) {
		this.search = search;
		clearTimeout(this.wait);
		this.wait = setTimeout(() => this.load(), SEARCH_DELAY_MS);
	}
}
