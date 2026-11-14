<script module>
	import { loadForm } from 'zipp-forms';

	// the form the editors chose, built in the studio's Forms; null where
	// it isn't live in the site
	/**
	 * @param {import('crelte').CrelteRequest} cr
	 * @param {{ form: { id: string }[] }} block
	 */
	export async function loadData(cr, block) {
		return { zippForm: await loadForm(cr, block.form[0]?.id) };
	}
</script>

<script>
	import { getTranslations } from 'crelte/translations';
	import { Back, Form, Fields, Submit } from 'zipp-forms';

	let { title, zippForm } = $props();
	const t = getTranslations();

	// the frontend's own texts of what the cms says in English
	$effect(() => {
		if (!zippForm) return;
		zippForm.messages = {
			failed: $t('contact.failed'),
			required: $t('contact.required'),
			email: $t('contact.email'),
			phone: $t('contact.phone'),
		};
	});
</script>

{#if zippForm}
	<section class="contact">
		{#if title}<h2>{title}</h2>{/if}
		<Form form={zippForm}>
			<!-- a form with pages: the step and its title -->
			{#if zippForm.pages.length > 1}
				<p class="step">
					{zippForm.page + 1} / {zippForm.pages.length}
					{#if zippForm.pages[zippForm.page].label}
						· {zippForm.pages[zippForm.page].label}
					{/if}
				</p>
			{/if}
			<Fields />
			<div class="buttons">
				<Back />
				<Submit />
			</div>
		</Form>
	</section>
{/if}

<style>
	.contact {
		max-width: 32rem;
		margin: 2rem 0;
	}

	.contact :global(form) {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.contact :global(.zipp-form-field) {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.contact :global(input),
	.contact :global(textarea) {
		padding: 0.5rem;
		font: inherit;
	}

	.contact :global(textarea) {
		min-height: 8rem;
	}

	.contact :global(.instructions) {
		margin: 0;
		font-size: 0.875rem;
		opacity: 0.7;
	}

	.contact :global(.error) {
		margin: 0;
		color: #b00020;
	}

	.step {
		margin: 0;
		font-weight: bold;
	}

	.buttons {
		display: flex;
		gap: 0.5rem;
	}

	.contact :global(button) {
		padding: 0.5rem 1rem;
		font: inherit;
	}
</style>
