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
