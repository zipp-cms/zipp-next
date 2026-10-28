<script>
	import { getGlobal } from 'crelte';

	const footer = getGlobal('footer');
</script>

<footer>
	<p>{$footer?.text ?? ''}</p>
	<p class="small">A crelte frontend, the content comes from zipp.</p>
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
