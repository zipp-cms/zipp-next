import { mount, unmount } from 'svelte';
import type { Host, HostFieldProps } from 'zipp-ui/field';
import { request } from '../core/client';
import { type Field } from '../core/schema';
import HostField from './HostField.svelte';

/**
 * What the studio lends the modules of field types and plugins, which
 * bring their own framework: its api with the session, and its fields for
 * images and files, texts, dropdowns, checkboxes, tables and markdown.
 * It is checked against zipp-ui's `Host`, the type the modules see.
 */
export const host = {
	/** A request to `/api<path>` as the logged in user. */
	request,

	/** Mounts a field of the studio, like the image picker. */
	mountField(target: HTMLElement, props: HostFieldProps) {
		let current = $state({ ...props });
		const c = mount(HostField, {
			target,
			props: {
				// a field like those of the schema, the module's word
				get field() {
					return current.field as Field;
				},
				get value() {
					return current.value;
				},
				onchange: (v: unknown) => current.onchange(v),
			},
		});
		return {
			update(p: Partial<HostFieldProps>) {
				current = { ...current, ...p };
			},
			destroy() {
				unmount(c);
			},
		};
	},
} satisfies Host;
