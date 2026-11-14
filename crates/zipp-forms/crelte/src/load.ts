import type { CrelteRequest } from 'crelte';
import { ZippForm, type FormOptions } from './form.svelte.js';
import type { FormData } from './types';

/**
 * Loads a form of zipp-forms in the site of the request, null if it
 * isn't live there. `id` is the form's entry, like what a form field of a
 * page gives: `loadForm(cr, entry.form[0]?.id)`. Needs `formRoutes` in
 * the server.
 *
 * ```js
 * export const loadData = {
 * 	form: (cr, entry) => loadForm(cr, entry.form[0]?.id),
 * };
 * ```
 */
export async function loadForm(
	cr: CrelteRequest,
	id: number | string | null | undefined,
	opts: Omit<FormOptions, 'site'> = {},
): Promise<ZippForm | null> {
	if (id === null || id === undefined) return null;

	const site = cr.site.handle;
	const data = (await cr.query('zippForm', { id: Number(id), site })) as {
		form: FormData | null;
	} | null;
	if (!data?.form) return null;
	return new ZippForm(data.form, { ...opts, site });
}
