<script>
	import { getGlobal } from 'crelte';
	import { getTranslations } from 'crelte/translations';

	const t = getTranslations();

	const footer = getGlobal('footer');
</script>

<footer>
	<p>{$footer?.text ?? ''}</p>
	<p class="small">{$t('footer.made')}</p>
</footer>

<style lang="scss">
	footer {
		margin-top: 3rem;
		color: #666;
	}

	.small {
		font-size: 0.85rem;
	}
</style>
