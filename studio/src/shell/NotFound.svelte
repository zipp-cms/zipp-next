<script lang="ts">
	import PageTop from 'zipp-ui/PageTop.svelte';
	import { t } from '../i18n/index.svelte';
</script>

<main class="page">
	<PageTop title={t('shell.notFound')} bleed={false} />
	<p class="empty">
		{t('shell.noPage')}
		<a href="/admin">{t('shell.toStart')}</a>
	</p>
</main>

<style lang="scss">
	.page {
		max-width: 40rem;
		margin: 0 auto;
	}
</style>
