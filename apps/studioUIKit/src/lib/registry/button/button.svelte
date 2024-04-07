<script lang="ts">
	import { cn } from '../../utils.js';
	import { Button as ButtonPrimitive } from 'bits-ui';
	import { buttonVariants } from './index.ts';

	export let variant: 'primary' | 'default' | 'outline' | 'ghost' = 'fill';

	let extraClasses = '';
	export { extraClasses as class };
</script>

<ButtonPrimitive.Root class={cn(buttonVariants({ variant }), extraClasses)} on:click on:keydown>
	<slot />
</ButtonPrimitive.Root>
