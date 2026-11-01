import {
	NO_EDITS,
	assetUsage,
	deleteAsset,
	moveAssets,
	noEdits,
	replaceAsset,
	updateAsset,
	type Asset,
	type AssetUsage,
	type Edits,
} from './api';
import { Busy } from '../core/busy.svelte';
import type { Values } from '../core/schema';

/** The values without empty ones, as the server keeps them. */
function filled(v: Values): Values {
	return Object.fromEntries(
		Object.entries(v).filter(
			([, x]) =>
				x !== null &&
				x !== undefined &&
				x !== '' &&
				!(Array.isArray(x) && !x.length),
		),
	);
}

/** The alt texts without empty ones, as the server keeps them. */
function filledAlt(alt: Record<string, string>): Record<string, string> {
	return Object.fromEntries(
		Object.entries(alt)
			.map(([site, a]) => [site, a.trim()])
			.filter(([, a]) => a),
	);
}

/**
 * What the asset panel changes: title, alt text (per site), filename, focal point
 * the image's edits, the asset fields and the folder, saved together;
 * the errors of the asset fields are in `fields`; whether there is
 * something to save, replacing the file, where the asset is used.
 */
export class AssetForm extends Busy {
	current = $state<Asset>() as Asset;
	title = $state('');
	/** per site handle */
	alt = $state<Record<string, string>>({});
	filename = $state('');
	focal = $state({ x: 0.5, y: 0.5 });
	edits = $state<Edits>({ ...NO_EDITS });
	/** of the asset fields */
	values = $state<Values>({});
	/** null: no folder */
	folder = $state<number | null>(null);
	usage = $state<AssetUsage[] | null>(null);
	/** what Cancel would throw away */
	private initial = $state('');

	constructor(asset: Asset) {
		super();
		this.reset(asset);
		assetUsage(asset.id)
			.then(u => (this.usage = u))
			.catch(() => (this.usage = []));
	}

	private reset(asset: Asset) {
		this.current = asset;
		this.title = asset.title;
		this.alt = { ...asset.alt };
		this.filename = asset.filename;
		this.focal = { x: asset.focalX, y: asset.focalY };
		this.edits = { ...NO_EDITS, ...(asset.edits ?? {}) };
		this.values = { ...asset.fields };
		this.folder = asset.folderId;
		this.initial = this.form();
	}

	private form() {
		return JSON.stringify({
			title: this.title,
			alt: filledAlt(this.alt),
			filename: this.filename,
			focal: this.focal,
			edits: $state.snapshot(this.edits),
			values: filled($state.snapshot(this.values)),
			folder: this.folder,
		});
	}

	dirty = $derived(this.form() !== this.initial);

	/** The saved asset, undefined when it failed (see `error`). */
	save(): Promise<Asset | undefined> {
		const { id, folderId } = this.current;
		const folder = this.folder;
		return this.run(async () => {
			const saved = await updateAsset(id, {
				title: this.title,
				alt: filledAlt(this.alt),
				filename: this.filename,
				focalX: this.focal.x,
				focalY: this.focal.y,
				edits: noEdits(this.edits) ? null : $state.snapshot(this.edits),
				fields: filled($state.snapshot(this.values)),
			});
			if (folder === folderId) return saved;
			await moveAssets([id], folder);
			return { ...saved, folderId: folder };
		});
	}

	/**
	 * Another file; the title, alt text and asset fields stay, the rest
	 * starts new.
	 */
	replace(file: File) {
		return this.run(async () => {
			const asset = await replaceAsset(this.current.id, file);
			const { title, alt, values } = this;
			this.reset(asset);
			this.title = title;
			this.alt = alt;
			this.values = values;
			this.initial = this.form();
		});
	}

	/** Whether it was deleted. */
	async remove(): Promise<boolean> {
		let gone = false;
		await this.run(async () => {
			await deleteAsset(this.current.id);
			gone = true;
		});
		return gone;
	}
}
