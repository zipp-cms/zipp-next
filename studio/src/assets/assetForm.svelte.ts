import {
	NO_EDITS,
	assetUsage,
	deleteAsset,
	noEdits,
	replaceAsset,
	updateAsset,
	type Asset,
	type AssetUsage,
	type Edits,
} from './api';
import { Busy } from '../core/busy.svelte';

/**
 * What the asset panel changes: title, alt text, filename, focal point
 * and the image's edits, saved together; whether there is something to
 * save, replacing the file, where the asset is used.
 */
export class AssetForm extends Busy {
	current = $state<Asset>() as Asset;
	title = $state('');
	alt = $state('');
	filename = $state('');
	focal = $state({ x: 0.5, y: 0.5 });
	edits = $state<Edits>({ ...NO_EDITS });
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
		this.alt = asset.alt;
		this.filename = asset.filename;
		this.focal = { x: asset.focalX, y: asset.focalY };
		this.edits = { ...NO_EDITS, ...(asset.edits ?? {}) };
		this.initial = this.form();
	}

	private form() {
		return JSON.stringify({
			title: this.title,
			alt: this.alt,
			filename: this.filename,
			focal: this.focal,
			edits: $state.snapshot(this.edits),
		});
	}

	dirty = $derived(this.form() !== this.initial);

	/** The saved asset, undefined when it failed (see `error`). */
	save(): Promise<Asset | undefined> {
		return this.run(() =>
			updateAsset(this.current.id, {
				title: this.title,
				alt: this.alt,
				filename: this.filename,
				focalX: this.focal.x,
				focalY: this.focal.y,
				edits: noEdits(this.edits) ? null : $state.snapshot(this.edits),
			}),
		);
	}

	/** Another file; the title and alt text stay, the rest starts new. */
	replace(file: File) {
		return this.run(async () => {
			const asset = await replaceAsset(this.current.id, file);
			const { title, alt } = this;
			this.reset(asset);
			this.title = title;
			this.alt = alt;
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
