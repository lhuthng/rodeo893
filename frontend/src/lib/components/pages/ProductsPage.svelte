<script>
	import { getRoute } from '$lib/navigation/index.js';
	import { t } from '$lib/localization';
	import PageShell from './PageShell.svelte';
	import {
		productCategoriesByRoute,
		products,
		productsByCategory
	} from '$lib/content/products/index.js';

	let { searchTerm = '' } = $props();

	const categories = ['productsCookies', 'productsIceCream', 'productsCoffee'];
	const normalizedSearchTerm = $derived(searchTerm.trim().toLowerCase());
	const filteredProducts = $derived(
		normalizedSearchTerm
			? products.filter((product) => {
					const name = $t(`productDetails.${product.route}.name`).toLowerCase();
					const description = $t(`productDetails.${product.route}.description`).toLowerCase();
					return name.includes(normalizedSearchTerm) || description.includes(normalizedSearchTerm);
				})
			: []
	);
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

	{#if normalizedSearchTerm}
		<section class="product-group product-group--search">
			<div class="product-group__heading">
				<p>{$t('search.submit')}: {searchTerm}</p>
			</div>
			{#if filteredProducts.length}
				<div class="product-grid">
					{#each filteredProducts as product}
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
			{:else}
				<p class="product-search-empty">{$t('search.empty')}</p>
			{/if}
		</section>
	{:else}
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
	{/if}
</PageShell>

<style lang="postcss">
	@reference '../../../app.css';

	.product-groups {
		@apply mt-10 grid gap-10;
	}

	.products-hero {
		@apply grid gap-6 border p-7 transition-all duration-500 md:grid-cols-[1.05fr_0.95fr] md:items-end;

		border-color: var(--theme-border);
		background-color: var(--theme-bg-elevated);

		& h2 {
			@apply mt-4 font-display text-[clamp(2.8rem,5vw,4.5rem)] leading-none tracking-tight;
			transition: color 500ms ease;
			color: var(--theme-fg);
		}

		& p {
			@apply mt-4 max-w-110 text-lg leading-relaxed;
			transition: color 500ms ease;
			color: var(--theme-fg-muted);
		}
	}

	.products-hero__eyebrow {
		@apply font-mono text-xs tracking-widest2 uppercase;
		transition: color 500ms ease;
		color: var(--theme-accent);
	}

	.products-hero__actions {
		@apply mt-8 flex flex-wrap gap-4;

		& a {
			@apply inline-flex items-center px-5 py-3 text-sm tracking-wider uppercase transition-colors duration-500;
		}

		& a:first-child {
			background-color: var(--theme-accent);
			color: var(--theme-accent-fg);

			&:hover {
				background-color: var(--theme-accent-alt);
				color: var(--theme-accent-alt-fg);
			}
		}

		& a:last-child {
			border: 1px solid var(--theme-border);
			color: var(--theme-fg);

			&:hover {
				border-color: var(--theme-accent);
				color: var(--theme-accent);
			}
		}
	}

	.products-hero__panel {
		@apply grid gap-3 border border-dashed p-5 text-lg leading-relaxed transition-all duration-500;
		border-color: var(--theme-border);
		background-color: var(--theme-bg);
		color: var(--theme-fg-muted);
	}

	.product-group__heading {
		@apply border-b pb-4 font-mono text-xs tracking-widest2 uppercase transition-colors duration-500;
		border-color: var(--theme-border);
		color: var(--theme-accent);
	}

	.product-search-empty {
		@apply mt-6 border px-4 py-3 text-lg;
		border-color: var(--theme-border);
		color: var(--theme-fg-muted);
	}

	.product-grid {
		@apply mt-6 grid gap-5 md:grid-cols-2 xl:grid-cols-3;
	}

	.product-card {
		@apply block border p-5 transition-all duration-500 hover:-translate-y-1 hover:shadow-lift;
		border-color: var(--theme-border);
		background-color: var(--theme-bg-elevated);

		& h2 {
			@apply mt-4 font-display text-2xl tracking-tight text-espresso;
			transition: color 500ms ease;
			color: var(--theme-fg);
		}

		& p {
			@apply mt-2 text-lg leading-relaxed;
			transition: color 500ms ease;
			color: var(--theme-fg-muted);
		}

		& span {
			@apply mt-5 inline-block font-mono text-xs tracking-wider uppercase;
			transition: color 500ms ease;
			color: var(--theme-accent);
		}
	}

	.product-card__image {
		@apply flex min-h-56 overflow-hidden border border-dashed transition-all duration-500;
		border-color: var(--theme-border);
		background-color: var(--theme-bg);

		& img {
			@apply h-full w-full object-cover;
		}
	}
</style>
