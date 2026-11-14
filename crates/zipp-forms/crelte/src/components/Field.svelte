<script lang="ts">
	import type { FieldData } from '../types';
	import Errors from './Errors.svelte';
	import Input from './Input.svelte';
	import Instructions from './Instructions.svelte';
	import Label from './Label.svelte';

	/** A field with its label, input, instructions and errors. */
	let { field }: { field: FieldData } = $props();
</script>

<div class="zipp-form-field zipp-form-{field.type}">
	<Label {field} />
	<Input {field} />
	<Instructions {field} />
	<Errors {field} />
</div>
