import type { Asset, Folder } from './api';
import { pathDirs, prepare, type Upload } from './files';
import { folderByPath, missingFolders } from './folders';
import { Library } from './library.svelte';
import { uploads } from './uploads.svelte';

/** The folder the picker showed last, it opens there again. */
let lastFolder: number | null = null;

/** The id of the field's folder in the tree before it exists. */
export const NEW = -1;

type Options = {
	/** only images */
	images: boolean;
	/** only files that end so */
	extensions?: string[];
	/** the field's folder: the picker opens there, uploads go there */
	folderPath?: string;
	multiple: boolean;
	onpick: (a: Asset) => void;
};

/**
 * The asset picker: the library in the field's folder, else where it
 * was last. The field's folder before it exists (a new topic's
 * `{slug}`) is in the tree, empty, and made with the first upload into
 * it.
 */
export class AssetPickerState {
	readonly lib: Library;
	readonly #o: Options;
	#ready = $state(false);
	#inNew = $state(false);

	/** the field's folders that don't exist yet */
	readonly missing = $derived.by(() =>
		this.#ready
			? missingFolders(this.lib.folders, this.#o.folderPath)
			: null,
	);

	/** the tree, with the field's folder that doesn't exist yet */
	readonly folders = $derived.by((): Folder[] => {
		const { folders } = this.lib;
		if (!this.missing) return folders;
		const { parent, names } = this.missing;
		return [
			...folders,
			{ id: NEW, parentId: parent, name: names.join(' / '), assets: 0 },
		];
	});

	/** the field's new folder is open */
	readonly showNew = $derived.by(
		() => this.#inNew && !!this.missing && !this.lib.search,
	);

	constructor(o: Options) {
		this.#o = o;
		this.lib = new Library({ images: o.images, extensions: o.extensions });

		// the field's folder first, else where the picker was
		this.lib.loadFolders().then(() => {
			this.#ready = true;
			if (this.missing) {
				this.#inNew = true;
				return;
			}
			const own = folderByPath(this.lib.folders, o.folderPath);
			const last = this.lib.folders.some(f => f.id === lastFolder);
			this.lib.select(own ? own.id : last ? lastFolder : null);
		});
	}

	/** The folder the tree shows open. */
	get current(): number | null {
		return this.showNew ? NEW : this.lib.folder;
	}

	select(id: number | null) {
		this.#inNew = id === NEW;
		if (this.#inNew) {
			this.lib.search = '';
			return;
		}
		lastFolder = id;
		this.lib.select(id);
	}

	/**
	 * Into the open folder, and picked; into the field's new folder, made
	 * now; on the top with a folder of the field, into that (made when
	 * missing).
	 */
	async upload(items: Upload[]) {
		const { images, extensions, folderPath, multiple, onpick } = this.#o;
		const lib = this.lib;
		const intoNew = this.showNew ? this.missing : null;
		const dir = intoNew
			? intoNew.names
			: lib.folder === null
				? pathDirs(folderPath)
				: [];
		const room = multiple ? items.length : 1;
		items = prepare(items, { images, extensions }, room, dir);
		const added = await uploads.run(
			items,
			intoNew ? intoNew.parent : lib.folder,
			() => lib.loadFolders(),
		);
		for (const a of added) onpick(a);

		// the new folder is there now, it opens with its files
		if (intoNew) {
			await lib.loadFolders();
			const own = folderByPath(lib.folders, folderPath);
			if (this.#inNew && own) this.select(own.id);
			return;
		}
		lib.assets = [...added, ...lib.assets];
	}
}
