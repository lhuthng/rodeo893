<script>
	import Metahead from '$lib/components/Metahead.svelte';
	import { getPageEntry } from '$lib/components/pages/index.js';
	import ProductCategoryPage from '$lib/components/pages/ProductCategoryPage.svelte';
	import ProductDetailPage from '$lib/components/pages/ProductDetailPage.svelte';
	import ProductsPage from '$lib/components/pages/ProductsPage.svelte';
	import { pathRoute } from '$lib/navigation/index.js';

	const { data } = $props();
	const routeId = $derived(data.routeId);
	const entry = $derived(data.pageType === 'static' ? getPageEntry(routeId) : null);
	const Route = $derived.by(() => {
		switch (data.pageType) {
			case 'products':
				return ProductsPage;
			case 'category':
				return ProductCategoryPage;
			case 'product':
				return ProductDetailPage;
			default:
				return entry?.component;
		}
	});
	const routeProps = $derived.by(() => {
		switch (data.pageType) {
			case 'products':
				return {
					searchTerm: data.searchTerm ?? '',
					products: data.products ?? [],
					categories: data.categories ?? [],
					featuredProducts: data.featuredProducts ?? [],
					catalogError: data.catalogError ?? null
				};
			case 'category':
				return {
					category: data.category ?? null,
					catalogError: data.catalogError ?? null,
					products: (data.products ?? []).filter(
						(product) => product.category_slug === data.category?.slug
					)
				};
			case 'product':
				return {
					category: data.category ?? null,
					catalogError: data.catalogError ?? null,
					product: data.product ?? null
				};
			default:
				return entry?.props ?? {};
		}
	});

	$effect(() => {
		pathRoute.set(data.pageType === 'static' ? routeId : 'products');
	});
</script>

<Metahead route={data.pageType === 'static' ? routeId : 'products'} />
{#if Route}
	<Route {...routeProps} />
{/if}
