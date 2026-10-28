import { mount, unmount } from 'svelte';
import { request, type Field } from '../api';
import HostField from './HostField.svelte';

/**
 * What the studio lends the modules of field types and plugins, which
 * bring their own framework: its api with the session, and its fields for
 * images and files, texts, dropdowns, checkboxes, tables and markdown.
 */
export const host = {
	/** A request to `/api<path>` as the logged in user. */
	request,

	/** Mounts a field of the studio, like the image picker. */
	mountField(
		target: HTMLElement,
		props: { field: Field; value: unknown; onchange: (v: unknown) => void },
	) {
		let current = $state({ ...props });
		const c = mount(HostField, {
			target,
			props: {
				get field() {
					return current.field;
				},
				get value() {
					return current.value;
				},
				onchange: (v: unknown) => current.onchange(v),
			},
		});
		return {
			update(p: Partial<typeof props>) {
				current = { ...current, ...p };
			},
			destroy() {
				unmount(c);
			},
		};
	},
};

export type Host = typeof host;
