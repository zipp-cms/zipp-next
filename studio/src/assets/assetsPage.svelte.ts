import { Busy } from '../core/busy.svelte';
import {
	assetUsage,
	deleteAsset,
	moveAssets,
	renameAsset,
	type Asset,
	type Folder,
} from './api';
import { confirmDelete, confirmUnzip, downloadZip, isZip } from './actions';
import type { Upload } from './files';
import { Library } from './library.svelte';
import { uploads } from './uploads.svelte';

/**
 * The Assets page: the library with the folder of the url, the asset
 * open in its panel and what the page does with assets and folders.
 */
export class AssetsPageState {
	readonly lib = new Library();
	readonly action = new Busy();
	/** the asset in the panel */
	current = $state<Asset | null>(null);
	/** the folder whose zip is being made */
	zipping = $state('');

	/** With what came with the page. */
	constructor(first: {
		folders: Folder[];
		top: number;
		folder: number | null;
		assets: Asset[];
	}) {
		Object.assign(this.lib, first);
		this.lib.loaded = true;
	}

	/** All again: what the list shows and the counts of the folders. */
	reload() {
		this.lib.load();
		this.lib.loadFolders();
	}

	/** Another folder, in the url too, without loading the page again. */
	select(id: number | null) {
		if (id === this.lib.folder && !this.lib.search) return;
		this.lib.select(id);
		const url = id === null ? location.pathname : `?folder=${id}`;
		history.replaceState(history.state, '', url);
	}

	async move(ids: number[], to: number | null) {
		await this.action.run(() => moveAssets(ids, to));
		this.reload();
	}

	/**
	 * Files, or folders with theirs, into a folder of the tree; zips are
	 * unpacked into folders when the editor wants.
	 */
	async upload(items: Upload[], into = this.lib.folder) {
		const zips = items.filter(i => isZip(i.file)).map(i => i.file);
		if (zips.length && confirmUnzip(zips))
			items = items.map(i => (isZip(i.file) ? { ...i, unzip: true } : i));
		// the new folders show while the files go up
		await uploads.run(items, into, () => this.lib.loadFolders());
		this.reload();
	}

	async download(f: Folder) {
		this.zipping = f.name;
		await this.action.run(() => downloadZip({ folder: f.id }, f.name));
		this.zipping = '';
	}

	/** Whether it is renamed. */
	async rename(a: Asset, filename: string): Promise<boolean> {
		const saved = await this.action.run(() => renameAsset(a, filename));
		if (saved) this.#replace(saved);
		return !!saved;
	}

	async remove(a: Asset) {
		const n = (await assetUsage(a.id).catch(() => [])).length;
		if (!confirmDelete(a, n)) return;
		await this.action.run(() => deleteAsset(a.id));
		this.reload();
	}

	/** The panel saved it; moved, it left the list. */
	saved(a: Asset) {
		const moved = a.folderId !== this.current?.folderId;
		this.#replace(a);
		this.current = null;
		if (moved) this.reload();
	}

	deleted() {
		this.current = null;
		this.reload();
	}

	closed() {
		// a replaced file has a new thumbnail
		this.current = null;
		this.lib.load();
	}

	#replace(a: Asset) {
		this.lib.assets = this.lib.assets.map(x => (x.id === a.id ? a : x));
	}
}
