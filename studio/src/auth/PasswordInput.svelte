<script lang="ts">
	import Icon from 'zipp-ui/Icon.svelte';
	import { t } from '../i18n/index.svelte';

	/** A password with an eye inside on its right that shows it. */
	let {
		value = $bindable(''),
		autocomplete,
		placeholder,
		autofocus = false,
	}: {
		value?: string;
		autocomplete: 'current-password' | 'new-password';
		placeholder?: string;
		autofocus?: boolean;
	} = $props();

	let shown = $state(false);
</script>

<span class="password">
	<!-- svelte-ignore a11y_autofocus -->
	<input
		class="secret"
		type={shown ? 'text' : 'password'}
		{autocomplete}
		{placeholder}
		{autofocus}
		bind:value
		required
	/>
	<button
		type="button"
		class="icon"
		title={shown ? t('auth.hidePassword') : t('auth.showPassword')}
		onclick={() => (shown = !shown)}
	>
		<Icon name={shown ? 'hide' : 'view'} />
	</button>
</span>

<style lang="scss">
	.password {
		position: relative;
		display: block;

		// over the padding of the forms' inputs
		.secret {
			width: 100%;
			padding-right: 2.75rem;
		}

		button {
			position: absolute;
			top: 50%;
			right: 0.375rem;
			transform: translateY(-50%);
		}
	}
</style>
