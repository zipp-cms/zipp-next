// The studio module of the forms plugin: `page` is its page
// Submissions, `mount` the editor of the fields of a form (formFields).
import { mount as svelteMount, unmount } from 'svelte';
import { fieldModule } from 'zipp-ui/field';
import FormFields from './FormFields.svelte';
import Submissions from './Submissions.svelte';

export const mount = fieldModule(FormFields);

export function page(target, props) {
	const app = svelteMount(Submissions, { target, props });
	return {
		destroy() {
			unmount(app);
		},
	};
}
