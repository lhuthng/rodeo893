<script>
	import { t } from '$lib/localization';
	import { getRoute } from '$lib/navigation/index.js';
	import PageShell from './PageShell.svelte';
</script>

<PageShell
	eyebrow={$t('contactPage.eyebrow')}
	title={$t('contactPage.title')}
	description={$t('contactPage.description')}
>
	<div class="contact-grid">
		<div class="contact-card">
			{#each $t('contactPage.channels') as channel}
				<p>{channel}</p>
			{/each}
		</div>
		<div class="contact-card contact-card--cta">
			<p>Keep this page calm. The stronger conversion should still point back toward products and membership.</p>
			<div>
				<a href={$getRoute('products')}>{$t('cta.order')}</a>
				<a href={$getRoute('membership')}>{$t('cta.membership')}</a>
			</div>
		</div>
	</div>
</PageShell>

<style lang="postcss">
	@reference '../../../app.css';

	.contact-grid {
		@apply mt-10 grid gap-6 md:grid-cols-2;
	}

	.contact-card {
		@apply grid gap-4 border border-espresso/10 bg-parchment-light p-6 text-lg leading-relaxed text-mahogany;
	}

	.contact-card--cta {
		& div {
			@apply flex flex-wrap gap-4 pt-2;
		}

		& a {
			@apply inline-flex items-center px-5 py-3 text-sm uppercase tracking-[0.18em] border border-espresso/20 transition-colors duration-150 hover:border-crimson-bright hover:text-crimson-bright;
		}
	}
</style>