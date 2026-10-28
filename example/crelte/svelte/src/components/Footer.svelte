<script>
	import { getGlobal } from 'crelte';
	import { getTranslations } from 'crelte/translations';

	const t = getTranslations();

	const footer = getGlobal('footer');
</script>

<footer>
	<p>{$footer?.text ?? ''}</p>
	<!-- a table field: rows of label and url -->
	{#if $footer?.links?.length}
		<p>
			{#each $footer.links as link (link.url)}
				<a href={link.url}>{link.label}</a>
			{/each}
		</p>
	{/if}
	<p class="small">{$t('footer.made')}</p>
</footer>

<style lang="scss">
	footer {
		margin-top: 3rem;
		color: #666;
	}

	a {
		margin-right: 1rem;
		color: inherit;
	}

	.small {
		font-size: 0.85rem;
	}
</style>
