import { request } from '../core/client';
import type { Asset } from '../assets/api';

/** An entry in the trash, with the children that went with it. */
export type TrashedEntry = {
	id: number;
	section: string;
	type: string;
	/** in the primary site */
	title: string;
	children: number;
	deletedAt: number;
	/** the email of who deleted it */
	deletedBy: string | null;
};

export type TrashedAsset = Asset & {
	deletedAt: number;
	deletedBy: string | null;
};

/** What the user may restore or delete for good, the newest first. */
export type Trash = {
	entries: TrashedEntry[];
	assets: TrashedAsset[];
	/** how long something stays before it is deleted for good */
	keepDays: number;
};

export function getTrash(): Promise<Trash> {
	return request('GET', '/trash');
}

export function restore(what: 'entries' | 'assets', id: number): Promise<void> {
	return request('POST', `/trash/${what}/${id}/restore`);
}

/** Deletes it for good. */
export function purge(what: 'entries' | 'assets', id: number): Promise<void> {
	return request('DELETE', `/trash/${what}/${id}`);
}

/** Deletes everything the user may delete for good. */
export function emptyTrash(): Promise<void> {
	return request('DELETE', '/trash');
}
