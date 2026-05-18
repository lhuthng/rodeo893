<script>
	import { page } from '$app/stores';
	import { getRoute } from '$lib/navigation/index.js';
	import { t } from '$lib/localization';
	import PageShell from './PageShell.svelte';

	let {
		searchTerm = '',
		products = [],
		categories = [],
		featuredProducts = [],
		catalogError = null
	} = $props();
	const allProducts = $derived(products.length ? products : ($page.data?.products ?? []));
	const allCategories = $derived(categories.length ? categories : ($page.data?.categories ?? []));
	const allFeaturedProducts = $derived(
		featuredProducts.length ? featuredProducts : ($page.data?.featuredProducts ?? [])
	);
	const activeCatalogError = $derived(catalogError ?? $page.data?.catalogError ?? null);

	const normalizedSearchTerm = $derived(searchTerm.trim().toLowerCase());
	const filteredProducts = $derived(
		normalizedSearchTerm
			? allProducts.filter((p) => {
					return (
						p.name.toLowerCase().includes(normalizedSearchTerm) ||
						p.description.toLowerCase().includes(normalizedSearchTerm)
					);
				})
			: []
	);
	const visibleCategories = $derived(
		allCategories.length
			? allCategories
			: Array.from(
					new Map(
						allProducts.map((product) => [
							product.category_slug,
							{
								slug: product.category_slug,
								name: product.category_slug
									.split('-')
									.map((part) => part.charAt(0).toUpperCase() + part.slice(1))
									.join(' '),
								path: product.category_path
							}
						])
					).values()
				)
	);

	const formatPrice = (amount, currency) => `${Number(amount).toLocaleString('vi-VN')} ${currency}`;
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
				<a href={visibleCategories[0]?.path ?? $getRoute('products')}>{$t('cta.exploreProducts')}</a
				>
				<a href={$getRoute('membership')}>{$t('cta.order')}</a>
			</div>
		</div>
		<div class="products-hero__panel">
			{#each visibleCategories as category}
				<p>{category.name}</p>
			{/each}
		</div>
	</div>

	{#if activeCatalogError}
		<p class="product-search-empty">Catalog unavailable: {activeCatalogError}</p>
	{:else if allFeaturedProducts.length && !normalizedSearchTerm}
		<section class="product-group product-group--featured">
			<div class="product-group__heading">
				<p>Signature picks</p>
			</div>
			<div class="product-grid">
				{#each allFeaturedProducts as product}
					<a class="product-card" href={product.path}>
						<div class="product-card__image">
							{#if product.image_url}
								<img src={product.image_url} alt={product.image_alt ?? ''} loading="lazy" />
							{/if}
						</div>
						<h2>{product.name}</h2>
						<p>{product.description}</p>
						<span>{formatPrice(product.base_price, product.currency)}</span>
					</a>
				{/each}
			</div>
		</section>
	{/if}

	{#if !activeCatalogError && normalizedSearchTerm}
		<section class="product-group product-group--search">
			<div class="product-group__heading">
				<p>{$t('search.submit')}: {searchTerm}</p>
			</div>
			{#if filteredProducts.length}
				<div class="product-grid">
					{#each filteredProducts as product}
						<a class="product-card" href={product.path}>
							<div class="product-card__image">
								{#if product.image_url}
									<img src={product.image_url} alt={product.image_alt ?? ''} loading="lazy" />
								{/if}
							</div>
							<h2>{product.name}</h2>
							<p>{product.description}</p>
							<span>{formatPrice(product.base_price, product.currency)}</span>
						</a>
					{/each}
				</div>
			{:else}
				<p class="product-search-empty">{$t('search.empty')}</p>
			{/if}
		</section>
	{:else if !activeCatalogError && allProducts.length}
		<div class="product-groups">
			{#each visibleCategories as category}
				{@const categoryProducts = allProducts.filter((p) => p.category_slug === category.slug)}
				{#if categoryProducts.length}
					<section class="product-group">
						<div class="product-group__heading">
							<p>{category.name}</p>
						</div>
						<div class="product-grid">
							{#each categoryProducts as product}
								<a class="product-card" href={product.path}>
									<div class="product-card__image">
										{#if product.image_url}
											<img src={product.image_url} alt={product.image_alt ?? ''} loading="lazy" />
										{/if}
									</div>
									<h2>{product.name}</h2>
									<p>{product.description}</p>
									<span>{formatPrice(product.base_price, product.currency)}</span>
								</a>
							{/each}
						</div>
					</section>
				{/if}
			{/each}
		</div>
	{:else if !activeCatalogError}
		<p class="product-search-empty">No products available yet.</p>
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
