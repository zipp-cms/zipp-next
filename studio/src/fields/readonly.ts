/** Whether a fieldset around `el` is disabled (a revision, a role). */
export function isReadonly(el: HTMLElement): boolean {
	return !!el.closest('fieldset[disabled]');
}

/**
 * Calls `onchange` when a fieldset around `el` is disabled or enabled
 * again (a conflict, publishing), for fields that aren't inputs a
 * fieldset disables itself. Returns the stop.
 */
export function watchReadonly(
	el: HTMLElement,
	onchange: (readonly: boolean) => void,
): () => void {
	const watch = new MutationObserver(() => onchange(isReadonly(el)));
	for (
		let f = el.closest('fieldset');
		f;
		f = f.parentElement?.closest('fieldset') ?? null
	)
		watch.observe(f, { attributes: true, attributeFilter: ['disabled'] });
	return () => watch.disconnect();
}
