<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from './Icon.svelte';

	/** A button that opens the file dialog: uploads and replacing a file. */
	let {
		onfiles,
		multiple = false,
		accept,
		primary = false,
		icon = 'add',
		children,
	}: {
		onfiles: (files: FileList) => void;
		multiple?: boolean;
		accept?: string;
		primary?: boolean;
		icon?: string;
		children: Snippet;
	} = $props();
</script>

<label class="upload light" class:primary>
	<Icon name={icon} size={14} />
	{@render children()}
	<input
		type="file"
		{multiple}
		{accept}
		onchange={e => {
			const files = e.currentTarget.files;
			if (files?.length) onfiles(files);
			e.currentTarget.value = '';
		}}
	/>
</label>

<style lang="scss">
	.upload {
		cursor: pointer;
		white-space: nowrap;

		input {
			display: none;
		}

		&:focus-within {
			outline: 2px solid var(--accent);
			outline-offset: 2px;
		}
	}

	.primary {
		border-color: var(--accent);
		background: var(--accent);
		color: white;

		&:hover {
			color: white;
			border-color: var(--accent-hover);
			background: var(--accent-hover);
		}
	}
</style>
