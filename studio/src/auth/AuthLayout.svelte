<script lang="ts">
	import type { Snippet } from 'svelte';
	import LogoIcon from 'zipp-ui/LogoIcon.svelte';
	import logo from '../logo/full.svg';

	/**
	 * The pages without the frame, the login and the links of mails: the
	 * form on the left, the icon big on the accent on the right (only
	 * where there is room). The form's parts are styled here: `.intro`,
	 * `.password` (an input with the eye), `.submit`, `.note`.
	 */
	let { children }: { children: Snippet } = $props();
</script>

<div class="login">
	<main>
		<header>
			<a href="/admin/login">
				<img class="logo" src={logo} alt="zipp" />
			</a>
		</header>

		<div class="form">{@render children()}</div>

		<footer class="note">zipp CMS · v{__ZIPP_VERSION__}</footer>
	</main>

	<aside aria-hidden="true">
		<div class="mark"><LogoIcon /></div>
	</aside>
</div>

<style lang="scss">
	.login {
		position: fixed;
		inset: 0;
		display: grid;
		grid-template-columns: 1fr 1fr;
		background: var(--surface);
		overflow-y: auto;

		@include max-tablet {
			grid-template-columns: 1fr;
		}
	}

	main {
		display: flex;
		flex-direction: column;
		min-height: 100%;
		padding: 2rem;
	}

	.logo {
		display: block;
		width: 5rem;
	}

	.form {
		display: flex;
		width: min(22rem, 100%);
		margin: auto;
		padding: 3rem 0;

		> :global(*) {
			display: flex;
			flex-direction: column;
			gap: 1.25rem;
			width: 100%;
		}

		:global(.intro) {
			display: flex;
			flex-direction: column;
			gap: 0.375rem;
		}

		:global(.intro h1) {
			font-size: var(--fs-title);
			line-height: 1.2;
			font-weight: 700;
		}

		:global(.intro p) {
			margin: 0;
			font-size: var(--fs-lg);
		}

		:global(input:not([type='checkbox'])) {
			padding: 0.5625rem 0.75rem;
			font-size: var(--fs-lg);
		}

		// the eye inside the input, on its right
		:global(.password) {
			position: relative;
			display: block;
		}

		:global(.password input) {
			padding-right: 2.75rem;
		}

		:global(.password button) {
			position: absolute;
			top: 50%;
			right: 0.375rem;
			transform: translateY(-50%);
		}

		:global(.check) {
			font-size: var(--fs-lg);
		}

		:global(.submit) {
			justify-content: center;
			gap: 0.5rem;
			padding: 0.625rem;
			font-size: var(--fs-lg);
			font-weight: 600;
		}

		:global(.note) {
			margin: 0;
			color: var(--muted);
			text-align: center;
		}
	}

	footer {
		text-align: center;
		font-family: var(--mono);
		font-size: var(--fs-xs);
	}

	aside {
		display: grid;
		place-items: center;
		background: var(--accent);

		@include max-tablet {
			display: none;
		}
	}

	.mark {
		--ring: var(--accent-ring);
		--bolt: var(--on-accent);
		width: min(28rem, 60%);
	}
</style>
