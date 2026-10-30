// The studio module of the SEO plugin: `mount` is the editor of the SEO
// field, `sectionSetup` the card on the setup page of a section.
import { mount as svelteMount, unmount } from 'svelte';
import { fieldModule } from 'zipp-ui/field';
import SeoField from './SeoField.svelte';
import SectionSetup from './SectionSetup.svelte';

export const mount = fieldModule(SeoField);

export function sectionSetup(target, props) {
	const app = svelteMount(SectionSetup, { target, props });
	return {
		destroy() {
			unmount(app);
		},
	};
}
