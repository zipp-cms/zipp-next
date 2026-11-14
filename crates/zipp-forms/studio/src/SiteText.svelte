<script>
	import { textIn } from './formFields.js';

	/**
	 * A text of a form's field in the site on screen (`formSiteText`):
	 * the fields are the same in all sites, their texts are translated. A
	 * site without its own shows another's as the placeholder, the
	 * website shows the primary site's then.
	 */
	let { value, onchange, site, readonly } = $props();
	const uid = $props.id();

	const texts = $derived(value ?? {});
	const other = $derived(textIn({ ...texts, [site]: '' }, site));

	function oninput(e) {
		const text = e.currentTarget.value;
		const next = { ...texts, [site]: text };
		if (!text) delete next[site];
		onchange(Object.keys(next).length ? next : null);
	}
</script>

<input
	id={uid}
	value={texts[site] ?? ''}
	placeholder={other}
	disabled={readonly}
	{oninput}
/>
