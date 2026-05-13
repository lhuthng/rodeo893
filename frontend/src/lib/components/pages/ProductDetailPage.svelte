<script>
	import { getRoute } from '$lib/navigation/index.js';
	import { t } from '$lib/localization';
	import PageShell from './PageShell.svelte';

	let { product } = $props();
	const details = $derived($t(`productDetails.${product.route}`));
</script>

<PageShell
	eyebrow={details('category')}
	title={details('name')}
	description={details('description')}
>
	<div class="detail-grid">
		<div class="detail-image">
			<img src={product.imageSrc} alt={product.imageAlt} loading="lazy" />
		</div>
		<div class="detail-copy">
			<div>
				<p class="detail-price">{product.priceFrom}</p>
				<ul>
					{#each details('notes') as note}
						<li>{note}</li>
					{/each}
				</ul>
			</div>
			<p class="detail-plating">{product.plating}</p>
			<div class="detail-actions">
				<a href={$getRoute('products')}>{$t('cta.exploreProducts')}</a>
				<a href={$getRoute('membership')}>{$t('cta.membership')}</a>
			</div>
		</div>
	</div>
</PageShell>

<style lang="postcss">
	@reference '../../../app.css';

	.detail-grid {
		@apply mt-10 grid gap-8 md:grid-cols-[1.05fr_0.95fr];
	}

	.detail-image {
		@apply min-h-90 overflow-hidden border border-dashed border-parchment-dark bg-parchment;

		& img {
			@apply h-full w-full object-cover;
		}
	}

	.detail-copy {
		@apply flex flex-col justify-between gap-8 border border-espresso/10 bg-parchment-light p-6;

		& ul {
			@apply grid gap-3;
		}

		& li {
			@apply border-b border-espresso/10 pb-3 text-lg leading-relaxed text-mahogany;
		}
	}

	.detail-price {
		@apply font-mono text-xs uppercase tracking-[0.22em] text-crimson-bright;
	}

	.detail-plating {
		@apply text-lg leading-relaxed text-mahogany;
	}

	.detail-actions {
		@apply flex flex-wrap gap-4;

		& a {
			@apply inline-flex items-center px-5 py-3 text-sm uppercase tracking-[0.18em] border border-espresso/20 transition-colors duration-150 hover:border-crimson-bright hover:text-crimson-bright;
		}
	}
</style>