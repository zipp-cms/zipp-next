# zipp-forms (crelte)

The [crelte](https://github.com/crelte/crelte) frontend of the forms of
[zipp](https://github.com/zipp-cms/zipp-next), the plugin `zipp-forms`:
load a form editors built in the studio, show it and send it.

```js
// server.js
import { formRoutes } from 'zipp-forms/server';

export async function routes(router) {
	formRoutes(router);
}
```

```svelte
<script module>
	import { loadForm } from 'zipp-forms';

	export async function loadData(cr, block) {
		return { zippForm: await loadForm(cr, block.form[0]?.id) };
	}
</script>

<script>
	import { Form, Fields, Submit } from 'zipp-forms';

	let { zippForm } = $props();
</script>

{#if zippForm}
	<Form form={zippForm}>
		<Fields />
		<Submit />
	</Form>
{/if}
```

It is Svelte and TypeScript source, the site's vite builds it. More in
[docs/forms.md](https://github.com/zipp-cms/zipp-next/blob/main/docs/forms.md).

## License

MIT or Apache-2.0, at your option.
