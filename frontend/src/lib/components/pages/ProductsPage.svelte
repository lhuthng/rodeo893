<script>
	import { getRoute } from '$lib/navigation/index.js';
	import { t } from '$lib/localization';
	import PageShell from './PageShell.svelte';
	import { productCategoriesByRoute, productsByCategory } from '$lib/content/products/index.js';

	const categories = ['productsCookies', 'productsIceCream', 'productsCoffee'];
</script>

<PageShell
	eyebrow={$t('productsPage.eyebrow')}
	title={$t('productsPage.title')}
	description={$t('productsPage.description')}
>
	<div class="products-hero">
		<div class="products-hero__copy">
			<p class="products-hero__eyebrow">{$t('productsPage.eyebrow')}</p>
			<h2>{$t('productsPage.title')}</h2>
			<p>{$t('productsPage.description')}</p>
			<div class="products-hero__actions">
				<a href={$getRoute('productsCookies')}>{$t('cta.exploreProducts')}</a>
				<a href={$getRoute('membership')}>{$t('cta.order')}</a>
			</div>
		</div>
		<div class="products-hero__panel">
			<p>{$t('productsPage.categories.cookies')}</p>
			<p>{$t('productsPage.categories.iceCream')}</p>
			<p>{$t('productsPage.categories.coffee')}</p>
		</div>
	</div>

	<div class="product-groups">
		{#each categories as category}
			<section class="product-group">
				<div class="product-group__heading">
					<p>{$t(`productsPage.categories.${productCategoriesByRoute[category].category}`)}</p>
				</div>
				<div class="product-grid">
					{#each productsByCategory[productCategoriesByRoute[category].category] ?? [] as product}
						<a class="product-card" href={$getRoute(product.route)}>
							<div class="product-card__image">
								<img src={product.imageSrc} alt={product.imageAlt} loading="lazy" />
							</div>
							<h2>{$t(`productDetails.${product.route}.name`)}</h2>
							<p>{$t(`productDetails.${product.route}.description`)}</p>
							<span>{product.priceFrom}</span>
						</a>
					{/each}
				</div>
			</section>
		{/each}
	</div>
</PageShell>

<style lang="postcss">
	@reference '../../../app.css';

	.product-groups {
		@apply mt-10 grid gap-10;
	}

	.products-hero {
		@apply grid gap-6 border border-espresso/10 bg-parchment-light p-7 md:grid-cols-[1.05fr_0.95fr] md:items-end;

		& h2 {
			@apply mt-4 font-display text-[clamp(2.8rem,5vw,4.5rem)] leading-[0.95] tracking-tight text-espresso;
		}

		& p {
			@apply mt-4 max-w-110 text-lg leading-relaxed text-mahogany;
		}
	}

	.products-hero__eyebrow {
		@apply font-mono text-xs uppercase tracking-[0.22em] text-crimson-bright;
	}

	.products-hero__actions {
		@apply mt-8 flex flex-wrap gap-4;

		& a {
			@apply inline-flex items-center px-5 py-3 text-sm uppercase tracking-[0.18em] transition-colors duration-150;
		}

		& a:first-child {
			@apply bg-espresso text-parchment-light hover:bg-crimson-bright;
		}

		& a:last-child {
			@apply border border-espresso/20 text-espresso hover:border-crimson-bright hover:text-crimson-bright;
		}
	}

	.products-hero__panel {
		@apply grid gap-3 border border-dashed border-parchment-dark bg-parchment p-5 text-lg leading-relaxed text-mahogany;
	}

	.product-group__heading {
		@apply border-b border-espresso/10 pb-4 font-mono text-xs uppercase tracking-[0.22em] text-crimson-bright;
	}

	.product-grid {
		@apply mt-6 grid gap-5 md:grid-cols-2 xl:grid-cols-3;
	}

	.product-card {
		@apply block border border-espresso/10 bg-parchment-light p-5 transition-transform duration-150 hover:-translate-y-1 hover:shadow-lift;

		& h2 {
			@apply mt-4 font-display text-2xl tracking-tight text-espresso;
		}

		& p {
			@apply mt-2 text-lg leading-relaxed text-mahogany;
		}

		& span {
			@apply mt-5 inline-block font-mono text-xs uppercase tracking-[0.18em] text-crimson-bright;
		}
	}

	.product-card__image {
		@apply flex min-h-56 overflow-hidden border border-dashed border-parchment-dark bg-parchment;

		& img {
			@apply h-full w-full object-cover;
		}
	}
</style>