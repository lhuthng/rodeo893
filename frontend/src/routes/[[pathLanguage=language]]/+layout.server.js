import { fetchProducts } from '$lib/api/products.js';
import {
	buildCatalogCategoryPath,
	buildCatalogProductPath,
	getLocalizedCatalogCategoryName,
	getLocalizedCatalogCategorySlug,
	getLocalizedCatalogProductSlug
} from '$lib/routing/routes.js';

/**
 * Server-only layout load: fetches the product catalogue once per page request.
 * `BACKEND_URL` is set in docker-compose; defaults to localhost for local dev.
 * Data is merged with +layout.js data and available as $page.data.products everywhere.
 */
export const load = async ({ fetch, params }) => {
	const lang = params.pathLanguage || 'en';
	const apiBase = process.env.BACKEND_URL ?? 'http://localhost:3000';

	try {
		const products = await fetchProducts(lang, fetch, apiBase);

		const localizedProducts = products.map((product) => {
			const localizedCategorySlug = getLocalizedCatalogCategorySlug(product);
			const localizedProductSlug = getLocalizedCatalogProductSlug(product);

			return {
				...product,
				localized_slug: localizedProductSlug,
				localized_category_slug: localizedCategorySlug,
				category_path: buildCatalogCategoryPath(
					product.category_slug,
					lang,
					'en',
					localizedCategorySlug
				),
				path: buildCatalogProductPath(
					product.category_slug,
					product.slug,
					lang,
					'en',
					localizedProductSlug,
					localizedCategorySlug
				)
			};
		});

		const categoryMap = new Map();
		for (const product of localizedProducts) {
			if (!categoryMap.has(product.category_slug)) {
				categoryMap.set(product.category_slug, {
					slug: product.category_slug,
					localized_slug: product.localized_category_slug,
					name: getLocalizedCatalogCategoryName(product),
					description: '',
					path: buildCatalogCategoryPath(
						product.category_slug,
						lang,
						'en',
						product.localized_category_slug
					),
					featured_path: product.path
				});
			}
		}

		return {
			products: localizedProducts,
			categories: Array.from(categoryMap.values()),
			featuredProducts: localizedProducts.slice(0, 3),
			catalogError: null
		};
	} catch (err) {
		return {
			products: [],
			categories: [],
			featuredProducts: [],
			catalogError: err instanceof Error ? err.message : 'Catalog unavailable'
		};
	}
};
