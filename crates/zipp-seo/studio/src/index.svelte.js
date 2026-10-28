// The studio module of the SEO plugin: `mount` is the editor of the SEO
// field, `sectionSetup` the card on the setup page of a section.
import { mount as svelteMount, unmount } from 'svelte';
import SeoField from './SeoField.svelte';
import SectionSetup from './SectionSetup.svelte';

export function mount(target, initial) {
	const props = $state({ ...initial });
	const app = svelteMount(SeoField, { target, props });
	return {
		update(next) {
			Object.assign(props, next);
		},
		destroy() {
			unmount(app);
		},
	};
}

export function sectionSetup(target, props) {
	const app = svelteMount(SectionSetup, { target, props });
	return {
		destroy() {
			unmount(app);
		},
	};
}
