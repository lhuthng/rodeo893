<script>
	import { getRoute } from '$lib/navigation/index.js';
	import { t } from '$lib/localization';
	import PageShell from './PageShell.svelte';

	let { category, products, featuredProduct } = $props();
	const details = $derived($t(`productsPage.categories.${category}`));
</script>

<PageShell
	eyebrow={$t('productsPage.eyebrow')}
	title={details}
	description={$t('productsPage.description')}
>
	<div class="category-hero">
		<div class="category-hero__copy">
			<p>{$t('cta.exploreProducts')}</p>
			<a href={$getRoute(featuredProduct)}>{$t('cta.order')}</a>
		</div>
		<div class="category-hero__list">
			{#each products as product}
				<a href={$getRoute(product.route)}>
					<img src={product.imageSrc} alt={product.imageAlt} loading="lazy" />
					<span>{$t(`productDetails.${product.route}.name`)}</span>
					<small>{product.priceFrom}</small>
				</a>
			{/each}
		</div>
	</div>
</PageShell>

<style lang="postcss">
	@reference '../../../app.css';

	.category-hero {
		@apply mt-10 grid gap-6 md:grid-cols-[0.85fr_1.15fr];
	}

	.category-hero__copy {
		@apply border border-espresso/10 bg-espresso px-6 py-8 text-parchment-light;

		& p {
			@apply font-display text-4xl tracking-tight;
		}

		& a {
			@apply mt-6 inline-flex w-fit items-center border border-parchment-dark px-5 py-3 text-sm uppercase tracking-[0.18em] transition-colors duration-150 hover:border-crimson-bright hover:text-crimson-bright;
		}
	}

	.category-hero__list {
		@apply grid gap-4;

		& a {
			@apply border border-espresso/10 bg-parchment-light p-5 transition-transform duration-150 hover:-translate-y-1;
		}

		& img {
			@apply mb-4 h-36 w-full border border-dashed border-parchment-dark bg-parchment object-cover;
		}

		& span {
			@apply block font-display text-2xl text-espresso;
		}

		& small {
			@apply mt-2 block text-lg leading-relaxed text-mahogany;
		}
	}
</style>