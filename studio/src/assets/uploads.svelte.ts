import { t } from '../i18n/index.svelte';
import { createFolder, listFolders, uploadAsset, type Asset } from './api';
import { childFolder } from './folders';
import { fromDrop, type Upload } from './files';

/** What an asset tile carries when it is dragged: the asset ids. */
export const ASSETS_DRAG = 'application/x-zipp-assets';

/** How many files go up at the same time. */
const PARALLEL = 3;

/**
 * The uploads of the studio with their progress, shown by
 * UploadProgress: the files, the bytes sent of all of them, the ones
 * that failed. Uploads started while others run add to them.
 */
class Uploads {
	files = $state(0);
	done = $state(0);
	sent = $state(0);
	total = $state(0);
	/** the assets made, a zip makes several and skips some */
	uploaded = $state(0);
	errors = $state<string[]>([]);
	/** whether the last ones are through, shown a moment longer */
	finished = $state(false);
	active = $derived(this.done < this.files);
	private hide: ReturnType<typeof setTimeout> | undefined;
	/** the last zip, the server unpacks one at a time */
	private zip: Promise<unknown> = Promise.resolve();

	/**
	 * The files into `folder` (null: the top), the folders of `dir`
	 * made below it where missing. The uploaded assets, without the
	 * failed ones.
	 */
	async run(
		items: Upload[],
		folder: number | null,
		/** once the folders of `dir` are made, before the files go up */
		onfolders?: () => void,
	): Promise<Asset[]> {
		if (!items.length) return [];
		clearTimeout(this.hide);
		if (!this.active) this.reset();
		const bytes = items.reduce((n, i) => n + i.file.size, 0);
		this.files += items.length;
		this.total += bytes;

		let dirs: Map<string, number | null>;
		try {
			dirs = await makeFolders(items, folder);
		} catch (e) {
			this.errors.push((e as Error).message);
			this.done += items.length;
			this.sent += bytes;
			this.end();
			return [];
		}
		if (dirs.size > 1) onfolders?.();

		const added: Asset[] = [];
		const queue = [...items];
		const next = async () => {
			for (let item; (item = queue.shift());) {
				const into = dirs.get(item.dir.join('/')) ?? null;
				let sent = 0;
				const send = () =>
					uploadAsset(item.file, into, !!item.unzip, s => {
						// the form is a bit larger than the file
						const now = Math.min(s, item.file.size);
						this.sent += now - sent;
						sent = now;
					});
				try {
					const done = await (item.unzip
						? this.afterZips(send)
						: send());
					const assets = Array.isArray(done) ? done : done.assets;
					added.push(...assets);
					this.uploaded += assets.length;
					if (!Array.isArray(done)) this.errors.push(...done.skipped);
				} catch (e) {
					this.errors.push(
						`${item.file.name}: ${(e as Error).message}`,
					);
				}
				this.sent += item.file.size - sent;
				this.done++;
			}
		};
		await Promise.all(Array.from({ length: PARALLEL }, next));
		this.end();
		return added;
	}

	/** Runs `f` once the zips before it are through. */
	private afterZips<T>(f: () => Promise<T>): Promise<T> {
		const run = this.zip.then(f);
		this.zip = run.catch(() => {});
		return run;
	}

	private reset() {
		this.files = this.done = this.sent = this.total = this.uploaded = 0;
		this.finished = false;
		this.errors = [];
	}

	private end() {
		if (this.active) return;
		this.finished = true;
		// the errors stay until closed
		if (!this.errors.length)
			this.hide = setTimeout(() => this.reset(), 2500);
	}

	close() {
		clearTimeout(this.hide);
		if (!this.active) this.reset();
	}

	/** A problem before any file went up, like a folder it can't read. */
	report(message: string) {
		this.errors.push(message);
		this.finished = !this.active;
	}
}

export const uploads = new Uploads();

/** The files of a drop for `then`, a drop that can't be read is reported. */
export function readDrop(data: DataTransfer, then: (items: Upload[]) => void) {
	fromDrop(data)
		.then(then)
		.catch(err =>
			uploads.report(
				t('assets.dropFailed', { error: err.message ?? err }),
			),
		);
}

/**
 * The folders of the files' `dir`s below `folder`, the missing ones
 * made: their ids by the path, `''` is `folder` itself.
 */
async function makeFolders(
	items: Upload[],
	folder: number | null,
): Promise<Map<string, number | null>> {
	const ids = new Map<string, number | null>([['', folder]]);
	const paths = [...new Set(items.map(i => i.dir.join('/')))].filter(p => p);
	if (!paths.length) return ids;

	let { folders } = await listFolders();
	for (const path of paths) {
		// the server trims names, a name with spaces around is the same
		const parts = path.split('/').map(n => n.trim());
		for (let i = 1; i <= parts.length; i++) {
			const here = parts.slice(0, i).join('/');
			if (ids.has(here)) continue;
			const parent = ids.get(parts.slice(0, i - 1).join('/')) ?? null;
			const name = parts[i - 1];
			let f = childFolder(folders, parent, name);
			if (!f) {
				try {
					f = await createFolder(parent, name);
					folders.push(f);
				} catch (e) {
					// another upload made it meanwhile
					folders = (await listFolders()).folders;
					f = childFolder(folders, parent, name);
					if (!f) throw e;
				}
			}
			ids.set(here, f.id);
		}
	}
	return ids;
}

/**
 * Files dragged over an element, for its drop overlay: `over` while
 * they are; a drop gives the files to `ondrop`. Asset tiles dragged
 * onto folders are no files. A drop a folder of the tree took (its
 * default prevented) only ends the drag.
 */
export class FileDrag {
	over = $state(false);
	private depth = 0;

	constructor(
		private ondrop: (items: Upload[]) => void,
		private enabled: () => boolean,
	) {}

	private files(e: DragEvent) {
		const t = e.dataTransfer?.types ?? [];
		return (
			this.enabled() && t.includes('Files') && !t.includes(ASSETS_DRAG)
		);
	}

	dragenter = (e: DragEvent) => {
		if (!this.files(e)) return;
		e.preventDefault();
		this.depth++;
		this.over = true;
	};

	dragleave = () => {
		if (!this.over) return;
		if (--this.depth <= 0) this.stop();
	};

	dragover = (e: DragEvent) => {
		if (this.files(e)) e.preventDefault();
	};

	drop = (e: DragEvent) => {
		const took = e.defaultPrevented;
		this.stop();
		if (took || !this.files(e)) return;
		e.preventDefault();
		readDrop(e.dataTransfer!, this.ondrop);
	};

	private stop() {
		this.depth = 0;
		this.over = false;
	}
}
