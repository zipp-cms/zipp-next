// The contract of a field editor of zipp's studio: mount(target, props)
// with props { field, value, onchange }, returning update and destroy.
import { mount as svelteMount, unmount } from 'svelte';
import ButtonGroup from './ButtonGroup.svelte';

export function mount(target, initial) {
	const props = $state({ ...initial });
	const app = svelteMount(ButtonGroup, { target, props });
	return {
		update(next) {
			Object.assign(props, next);
		},
		destroy() {
			unmount(app);
		},
	};
}
