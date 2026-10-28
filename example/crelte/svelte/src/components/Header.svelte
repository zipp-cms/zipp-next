<script>
	import { getGlobal, getRouter, getSite } from 'crelte';

	// top pages of the structure, from global.graphql
	const pages = getGlobal('pages');
	const site = getSite();
	const sites = getRouter().sites;
</script>

<header>
	<nav>
		<a href={$site.url.href}>Home</a>
		{#each $pages.filter(p => !p.parent) as page (page.url)}
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
