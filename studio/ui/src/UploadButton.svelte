<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from './Icon.svelte';

	/**
	 * A button that opens the file dialog: uploads and replacing a file;
	 * with `directory` a folder, its files with their `webkitRelativePath`.
	 */
	let {
		onfiles,
		multiple = false,
		directory = false,
		accept,
		primary = false,
		icon = 'add',
		children,
	}: {
		onfiles: (files: FileList) => void;
		multiple?: boolean;
		directory?: boolean;
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
		webkitdirectory={directory || undefined}
		onchange={e => {
			const files = e.currentTarget.files;
			if (files?.length) onfiles(files);
			e.currentTarget.value = '';
		}}
	/>
</label>

<style lang="scss">
	@use '../vars' as *;

	.upload {
		cursor: pointer;
		white-space: nowrap;

		input {
			display: none;
		}

		&:focus-within {
			@include focus-ring;
		}
	}

	.primary {
		border-color: var(--accent);
		background: var(--accent);
		color: var(--on-accent);

		@include hover {
			&:hover {
				color: var(--on-accent);
				border-color: var(--accent-hover);
				background: var(--accent-hover);
			}
		}
	}
</style>
