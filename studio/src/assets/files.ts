/** The files a user gives the studio: from the dialog, a drop, for a field. */

/**
 * A file to upload, `dir` the folders it was in, below the target;
 * `unzip`: a zip the server unpacks into folders there.
 */
export type Upload = { file: File; dir: string[]; unzip?: boolean };

/** The files of the file dialog, a chosen folder with its subfolders. */
export function fromInput(files: FileList): Upload[] {
	return (
		[...files]
			// `Photos/Events/a.jpg` of a folder, `a.jpg` of files
			.map(file => ({
				file,
				parts: (file.webkitRelativePath || file.name).split('/'),
			}))
			// hidden files and folders stay home, like in a drop
			.filter(({ parts }) => !parts.some(p => p.startsWith('.')))
			.map(({ file, parts }) => ({ file, dir: parts.slice(0, -1) }))
	);
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
type Allowed = { images: boolean; extensions?: string[] };

/** The extension of a file name in lower case, empty without one. */
export function extOf(name: string): string {
	const dot = name.lastIndexOf('.');
	return dot < 0 ? '' : name.slice(dot + 1).toLowerCase();
}

/** Whether a file fits the field. */
function allows(allowed: Allowed, file: File): boolean {
	if (allowed.images && !file.type.startsWith('image/')) return false;
	const exts = allowed.extensions ?? [];
	return !exts.length || exts.includes(extOf(file.name));
}

/** The file dialog's `accept` of a field. */
export function acceptOf(allowed: Allowed): string | undefined {
	if (allowed.extensions?.length)
		return allowed.extensions.map(e => '.' + e).join(',');
	return allowed.images ? 'image/*' : undefined;
}

/** The folders of a path like `Photos/Events`, for `Upload.dir`. */
export function pathDirs(path: string | undefined): string[] {
	return (path ?? '')
		.split('/')
		.map(n => n.trim())
		.filter(n => n);
}

/**
 * The files a field or the picker takes: the allowed ones, as many as
 * there is room for, into `dir` (the field's folder; the folders of a
 * dropped folder are left out).
 */
export function prepare(
	items: Upload[],
	allowed: Allowed,
	room: number,
	dir: string[],
): Upload[] {
	return items
		.filter(i => allows(allowed, i.file))
		.slice(0, Math.max(room, 0))
		.map(i => ({ ...i, dir }));
}
