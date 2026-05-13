<script>
	import { t } from '$lib/localization';
	import PageShell from './PageShell.svelte';
</script>

<PageShell
	eyebrow={$t('aboutPage.eyebrow')}
	title={$t('aboutPage.title')}
	description={$t('aboutPage.description')}
>
	<div class="about-band">
		<p>Design language: editorial type, warm neutrals, sharp accents, and enough negative space that the products feel expensive before any decorative treatment appears.</p>
		<p>Operational language: small-batch production, pre-order rhythms, membership-first conversion, and a future account/order surface aligned with the backend APIs.</p>
	</div>
</PageShell>

<style lang="postcss">
	@reference '../../../app.css';

	.about-band {
		@apply mt-10 grid gap-5 md:grid-cols-2;

		& p {
			@apply border border-espresso/10 bg-parchment-light p-6 text-lg leading-relaxed text-mahogany;
		}
	}
</style>