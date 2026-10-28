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
		<!-- the navigation field: links with a second level -->
		{#each $header?.navigation ?? [] as item (item.url)}
			<span class="item">
				<a href={item.url} target={item.target}>
					{item.label ?? item.defaultLabel}
				</a>
				{#if item.children.length}
					<span class="sub">
						{#each item.children as child (child.url)}
							<a href={child.url} target={child.target}>
								{child.label ?? child.defaultLabel}
							</a>
						{/each}
					</span>
				{/if}
			</span>
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

	.item {
		position: relative;
	}

	.sub {
		display: none;
		position: absolute;
		top: 100%;
		left: 0;
		flex-direction: column;
		gap: 0.25rem;
		padding: 0.5rem;
		background: white;
		box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
		white-space: nowrap;
	}

	.item:hover .sub,
	.item:focus-within .sub {
		display: flex;
	}
</style>
