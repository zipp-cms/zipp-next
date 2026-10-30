<script module lang="ts">
	import type { Schema as SchemaT } from '../core/schema';

	/** A doc of the code by its handle. */
	export function loadProps(props: { schema: SchemaT; handle: string }) {
		const doc = props.schema.docs.find(d => d.handle === props.handle);
		if (!doc) throw new Error(`There is no doc ${props.handle}`);
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

<PageTop title={doc.name} note="Read only, from the code" />

<div class="card markdown-body" use:external>
	{@html doc.html}
</div>

<style lang="scss">
	.card {
		max-width: 56.25rem;
		padding: 1.5rem 2rem;
	}
</style>
