<script>
	/**
	 * Talks with zipp's studio when the site is its preview (an iframe
	 * with a preview token): keeps the token and crelte's live preview
	 * mode on every page, tells the studio which page is shown and opens
	 * pages when it asks. The studio reloads the iframe for changes, live
	 * preview mode restores the scroll position.
	 */
	import { onMount } from 'svelte';
	import { getRouter, onRequest, onRoute } from 'crelte';

	const router = getRouter();
	const inStudio = typeof window !== 'undefined' && window.parent !== window;
	const token = inStudio
		? new URL(window.location.href).searchParams.get('token')
		: null;
	const PARAMS = { token, 'x-craft-live-preview': '1' };

	function tell(url) {
		const u = new URL(url);
		for (const p of Object.keys(PARAMS)) u.searchParams.delete(p);
		window.parent.postMessage({ type: 'zipp:location', url: u.href }, '*');
	}

	if (token) {
		// links of the site don't have them
		onRequest(cr => {
			for (const [k, v] of Object.entries(PARAMS))
				if (!cr.req.getSearchParam(k)) cr.req.setSearchParam(k, v);
		});
		onRoute(route => tell(route.url.href));
	}

	onMount(() => {
		if (!token) return;
		tell(window.location.href);
		const onMessage = e => {
			if (e.source !== window.parent) return;
			if (e.data?.type === 'zipp:open' && e.data.url) router.open(e.data.url);
		};
		window.addEventListener('message', onMessage);
		return () => window.removeEventListener('message', onMessage);
	});
</script>
