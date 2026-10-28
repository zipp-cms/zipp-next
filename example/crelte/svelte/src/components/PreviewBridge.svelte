<script>
	/**
	 * Tells zipp's studio which page its preview shows and opens pages
	 * when it asks, for the AI chat. The links keep the preview token by
	 * themselves: zipp adds it to every url of a preview, like Craft.
	 */
	import { onMount } from 'svelte';
	import { getRouter, onRoute } from 'crelte';

	const router = getRouter();
	const inPreview =
		typeof window !== 'undefined' &&
		window.parent !== window &&
		new URL(window.location.href).searchParams.has('token');

	function tell(url) {
		const u = new URL(url);
		u.searchParams.delete('token');
		u.searchParams.delete('x-craft-live-preview');
		window.parent.postMessage({ type: 'zipp:location', url: u.href }, '*');
	}

	if (inPreview) onRoute(route => tell(route.url.href));

	onMount(() => {
		if (!inPreview) return;
		tell(window.location.href);
		const onMessage = e => {
			if (e.source !== window.parent) return;
			const url = e.data?.type === 'zipp:open' && e.data.url;
			// only pages of this site, whoever framed it
			if (url && new URL(url, location.href).origin === location.origin)
				router.open(url);
		};
		window.addEventListener('message', onMessage);
		return () => window.removeEventListener('message', onMessage);
	});
</script>
