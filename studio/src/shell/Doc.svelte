<script module lang="ts">
	import { docOf, type Schema as SchemaT } from '../core/schema';
	import { t } from '../i18n/index.svelte';

	/** A doc of the code by its handle. */
	export function loadProps(props: { schema: SchemaT; handle: string }) {
		const doc = docOf(props.schema, props.handle);
		if (!doc) throw new Error(t('shell.noDoc', { handle: props.handle }));
		return { doc };
	}
</script>

<script lang="ts">
	import PageTop from 'zipp-ui/PageTop.svelte';
	import type { Schema } from '../core/schema';

	/** A markdown file of the code, read only. */
	let { doc }: { doc: Schema['docs'][number] } = $props();

	// links out of the studio open in a new tab, many sites can't be framed
	function external(node: HTMLElement) {
		$effect(() => {
			void doc.html;
			for (const a of node.querySelectorAll('a')) {
				if (a.origin !== location.origin) {
					a.target = '_blank';
					a.rel = 'noreferrer';
				}
			}
		});
	}
</script>

<PageTop title={doc.name} note={t('shell.readOnly')} />

<div class="card markdown-body" use:external>
	{@html doc.html}
</div>

<style lang="scss">
	.card {
		max-width: 56.25rem;
		padding: 1.5rem 2rem;
	}
</style>
