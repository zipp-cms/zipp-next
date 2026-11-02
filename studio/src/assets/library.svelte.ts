import { Latest } from '../core/latest.svelte';
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
	private loadingAssets = new Latest();
	private loadingFolders = new Latest();
	error = $derived(this.loadingAssets.error || this.loadingFolders.error);

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
			? t('ui.nothingFound')
			: this.folder === null
				? t('assets.noAssets')
				: t('assets.noFilesInFolder'),
	);

	/** The assets of the open folder, of all with a search. */
	load() {
		return this.loadingAssets.run(this.ask, assets => this.show(assets));
	}

	private ask = () =>
		listAssets({
			search: this.search,
			folder: this.search ? undefined : this.folder,
			kind: this.kind || undefined,
			...this.only,
		});

	private show(assets: Asset[]) {
		this.assets = assets;
		this.loaded = true;
	}

	/** The folders with their counts. */
	loadFolders() {
		return this.loadingFolders.run(listFolders, ({ folders, top }) => {
			this.folders = folders;
			this.top = top;
		});
	}

	/** Another folder, the search ends. */
	select(id: number | null) {
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
		this.loadingAssets.later(this.ask, assets => this.show(assets));
	}
}
