// The studio module of the forms plugin: `page` is its page
// Submissions, `mount` the editors of its field types: the fields of a
// form (formFields) and a text of a field per site (formSiteText).
import { mount as svelteMount, unmount } from 'svelte';
import { fieldModule } from 'zipp-ui/field';
import FormFields from './FormFields.svelte';
import SiteText from './SiteText.svelte';
import Submissions from './Submissions.svelte';

// the editors of the plugin's two field types
const builder = fieldModule(FormFields);
const siteText = fieldModule(SiteText);

export function mount(target, props) {
	const editor = props.field.type === 'formSiteText' ? siteText : builder;
	return editor(target, props);
}

export function page(target, props) {
	const app = svelteMount(Submissions, { target, props });
	return {
		destroy() {
			unmount(app);
		},
	};
}
