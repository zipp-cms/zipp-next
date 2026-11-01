<script lang="ts">
	import PageTop from 'zipp-ui/PageTop.svelte';
	import { t } from '../i18n/index.svelte';

	/** A page that couldn't load: its error, and a way back. */
	let { loadError }: { loadError: unknown } = $props();
	const message = $derived(
		loadError instanceof Error ? loadError.message : String(loadError),
	);
</script>

<div class="page">
	<PageTop title={t('shell.pageFailed')} />
	<p class="error">{message}</p>
	<p class="empty"><a href="/admin">{t('shell.toStart')}</a></p>
</div>

<style lang="scss">
	.page {
		max-width: 40rem;
		margin: 0 auto;
	}
</style>
