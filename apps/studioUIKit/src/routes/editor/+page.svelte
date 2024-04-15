<script lang="ts">
	import Editor from '$lib/registry/editor/editor.svelte';
	import type { Component } from '$lib/registry/editor/editor.ts';

	const components: Component[] = [
		{
			name: 'Button',
			handle: 'button',
			fields: {
				label: {
					kind: 'text',
					settings: {
						max_length: 100
					}
				},
				link: {
					kind: 'component',
					settings: {
						component: ['internal-link', 'external-link'],
						min: 1,
						max: 1
					}
				}
			}
		},
		{
			name: 'Internal Link',
			handle: 'internal-link',
			fields: {
				page: {
					kind: 'relation'
				}
			}
		},
		{
			name: 'External Link',
			handle: 'external-link',
			fields: {
				url: {
					kind: 'text',
					settings: {
						rules: ['url']
					}
				}
			}
		},
		{
			name: 'Text',
			handle: 'text',
			fields: {
				content: {
					kind: 'text',
					settings: {
						max_length: 100
					}
				}
			}
		},
		{
			name: 'Page',
			handle: 'page',
			fields: {
				title: {
					kind: 'text',
					settings: {
						max_length: 100
					}
				},
				content: {
					kind: 'component',
					settings: {
						component: ['button', 'text', 'cta']
					}
				}
			}
		},
		{
			name: 'Call to action',
			handle: 'cta',
			fields: {
				headline: {
					kind: 'text',
					settings: {
						max_length: 100
					}
				},
				text: {
					kind: 'text',
					settings: {
						max_length: 100
					}
				},
				buttons: {
					kind: 'component',
					settings: {
						component: ['button'],
						max: 2
					}
				}
			}
		}
	];
</script>

<Editor {components} />
