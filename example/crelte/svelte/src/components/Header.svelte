<script>
	import { getGlobal, getRouter, getSite } from 'crelte';
	import { getTranslations } from 'crelte/translations';

	const t = getTranslations();

	// the navigation of the header single, from global.graphql
	const header = getGlobal('header');
	const site = getSite();
	const sites = getRouter().sites;
</script>

<header>
	<nav>
		<a href={$site.url.href}>{$t('nav.home')}</a>
		{#each $header?.navigation ?? [] as page (page.url)}
			<a href={page.url}>{page.title}</a>
		{/each}
	</nav>
	<nav>
		{#each sites as s (s.id)}
			<a href={s.url.href} class:active={s.id === $site.id}>
				{s.language.slice(0, 2).toUpperCase()}
			</a>
		{/each}
	</nav>
</header>

<style lang="scss">
	header {
		display: flex;
		justify-content: space-between;
		margin-bottom: 2rem;
	}

	nav {
		display: flex;
		gap: 1rem;
	}

	.active {
		font-weight: bold;
	}
</style>
