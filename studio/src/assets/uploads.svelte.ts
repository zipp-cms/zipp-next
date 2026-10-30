import { upload } from '../core/client';
import { createFolder, listFolders, type Asset } from './api';

/** What an asset tile carries when it is dragged: the asset ids. */
export const ASSETS_DRAG = 'application/x-zipp-assets';

/** A file to upload, `dir` the folders it was in, below the target. */
export type Upload = { file: File; dir: string[] };

/** The files of the file dialog, a chosen folder with its subfolders. */
export function fromInput(files: FileList): Upload[] {
	return [...files]
		.filter(f => !f.name.startsWith('.'))
		.map(file => ({
			file,
			// `Photos/Events/a.jpg` of a folder, empty for files
			dir: file.webkitRelativePath.split('/').slice(0, -1),
		}));
}

/**
 * The files of a drop, the dropped folders walked through. The entries
 * have to be taken before the first await, the browser empties the
 * DataTransfer after the event.
 */
export async function fromDrop(data: DataTransfer): Promise<Upload[]> {
	const entries = [...data.items]
		.map(i => i.webkitGetAsEntry?.())
		.filter((e): e is FileSystemEntry => !!e);
	if (!entries.length)
		return [...data.files].map(file => ({ file, dir: [] }));

	const out: Upload[] = [];
	const walk = async (entry: FileSystemEntry, dir: string[]) => {
		if (entry.name.startsWith('.')) return;
		if (entry.isFile) {
			const file = await new Promise<File>((res, rej) =>
				(entry as FileSystemFileEntry).file(res, rej),
			);
			out.push({ file, dir });
		} else if (entry.isDirectory) {
			const reader = (entry as FileSystemDirectoryEntry).createReader();
			// it gives the entries in batches, an empty one at the end
			for (;;) {
				const batch = await new Promise<FileSystemEntry[]>((res, rej) =>
					reader.readEntries(res, rej),
				);
				if (!batch.length) break;
				for (const e of batch) await walk(e, [...dir, entry.name]);
			}
		}
	};
	for (const e of entries) await walk(e, []);
	return out;
}

/** What an asset field takes: only images, only some extensions. */
export type Allowed = { images: boolean; extensions?: string[] };

/** Whether a file fits the field. */
export function allows(allowed: Allowed, file: File): boolean {
	if (allowed.images && !file.type.startsWith('image/')) return false;
	const exts = allowed.extensions ?? [];
	const ext = file.name.split('.').pop()?.toLowerCase() ?? '';
	return !exts.length || (file.name.includes('.') && exts.includes(ext));
}

/** The file dialog's `accept` of a field. */
export function acceptOf(allowed: Allowed): string | undefined {
	if (allowed.extensions?.length)
		return allowed.extensions.map(e => '.' + e).join(',');
	return allowed.images ? 'image/*' : undefined;
}

/** The folders of a path like `Photos/Events`, for `Upload.dir`. */
export function pathDirs(path: string | undefined): string[] {
	return (path ?? '').split('/').filter(n => n.trim());
}

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
	errors = $state<string[]>([]);
	/** whether the last ones are through, shown a moment longer */
	finished = $state(false);
	active = $derived(this.done < this.files);
	private hide: ReturnType<typeof setTimeout> | undefined;

	/**
	 * The files into `folder` (null: the top), the folders of `dir`
	 * made below it where missing. The uploaded assets, without the
	 * failed ones.
	 */
	async run(items: Upload[], folder: number | null): Promise<Asset[]> {
		if (!items.length) return [];
		clearTimeout(this.hide);
		if (!this.active) this.reset();
		this.files += items.length;
		this.total += items.reduce((n, i) => n + i.file.size, 0);

		let dirs: Map<string, number | null>;
		try {
			dirs = await makeFolders(items, folder);
		} catch (e) {
			this.errors.push((e as Error).message);
			this.done += items.length;
			this.end();
			return [];
		}

		const added: Asset[] = [];
		const queue = [...items];
		const next = async () => {
			for (let item; (item = queue.shift());) {
				const into = dirs.get(item.dir.join('/')) ?? null;
				let sent = 0;
				const form = new FormData();
				form.append('file', item.file, item.file.name);
				const path =
					into === null ? '/assets' : `/assets?folder=${into}`;
				try {
					const assets = await upload<Asset[]>(path, form, s => {
						// the form is a bit larger than the file
						const now = Math.min(s, item.file.size);
						this.sent += now - sent;
						sent = now;
					});
					added.push(...assets);
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

	private reset() {
		this.files = this.done = this.sent = this.total = 0;
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
}

export const uploads = new Uploads();

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

	const { folders } = await listFolders();
	for (const path of paths) {
		const parts = path.split('/');
		for (let i = 1; i <= parts.length; i++) {
			const here = parts.slice(0, i).join('/');
			if (ids.has(here)) continue;
			const parent = ids.get(parts.slice(0, i - 1).join('/')) ?? null;
			const name = parts[i - 1];
			let f = folders.find(
				f =>
					f.parentId === parent &&
					f.name.toLowerCase() === name.toLowerCase(),
			);
			if (!f) {
				f = await createFolder(parent, name);
				folders.push(f);
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
		fromDrop(e.dataTransfer!).then(this.ondrop);
	};

	private stop() {
		this.depth = 0;
		this.over = false;
	}
}
