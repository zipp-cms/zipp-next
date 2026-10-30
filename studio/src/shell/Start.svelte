<script module lang="ts">
	import type { LoadProps } from '../core/router.svelte';
	import type { Schema } from '../core/schema';
	import { visibleNav } from '../core/access';
	import { navHref } from './nav';

	/** `/admin`: the first page of the navigation. */
	export function loadProps(props: { schema: Schema }, lp: LoadProps) {
		const first = visibleNav(props.schema)[0]?.sections[0];
		const href = first ? navHref(props.schema, first) : '/admin/assets';
		lp.setRedirect(href.slice('/admin'.length));
	}
</script>
