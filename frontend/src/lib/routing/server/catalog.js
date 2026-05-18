import { error, redirect } from '@sveltejs/kit';
import { resolveCatalogPath } from '$lib/api/products.js';

import {
	buildCatalogCategoryPath,
	buildCatalogProductPath,
	buildRoutePath,
	getLocalizedCatalogCategorySlug,
	getLocalizedCatalogProductSlug
} from '$lib/routing/routes.js';

export const tryResolveCatalogRoute = async ({
	segments,
	products,
	categories,
	requestedLanguage,
	activeDefaultLanguage,
	searchTerm,
	url,
	fetch,
	apiBase
}) => {
	if (segments.length === 0) return null;

	const rest = segments.slice(1);
	if (rest.length === 0) {
		const canonicalPath = buildRoutePath('products', requestedLanguage, activeDefaultLanguage);
		if (url.pathname !== canonicalPath) throw redirect(307, canonicalPath);

		return {
			pageType: 'products',
			routeId: 'products',
			searchTerm,
			products,
			categories
		};
	}

	if (rest.length === 1) {
		const categorySegment = rest[0];
		const resolvedPath = await resolveCatalogPath(
			{
				category: categorySegment,
				lang: requestedLanguage
			},
			fetch,
			apiBase
		);
		if (!resolvedPath) throw error(404, 'Category not found');

		const category = categories.find((entry) => entry.slug === resolvedPath.category_slug);
		if (!category) throw error(404, 'Category not found');

		const canonicalPath = buildCatalogCategoryPath(
			category.slug,
			requestedLanguage,
			activeDefaultLanguage,
			getLocalizedCatalogCategorySlug(category)
		);
		if (url.pathname !== canonicalPath) throw redirect(307, canonicalPath);

		return {
			pageType: 'category',
			routeId: 'products',
			searchTerm,
			category,
			products,
			categories
		};
	}

	if (rest.length === 2) {
		const [categorySegment, productSegment] = rest;
		const resolvedPath = await resolveCatalogPath(
			{
				category: categorySegment,
				product: productSegment,
				lang: requestedLanguage
			},
			fetch,
			apiBase
		);
		if (!resolvedPath || !resolvedPath.product_slug) throw error(404, 'Product not found');

		const category = categories.find((entry) => entry.slug === resolvedPath.category_slug);
		if (!category) throw error(404, 'Product not found');

		const product = products.find(
			(entry) =>
				entry.category_slug === resolvedPath.category_slug &&
				entry.slug === resolvedPath.product_slug
		);
		if (!product) throw error(404, 'Product not found');

		const canonicalPath = buildCatalogProductPath(
			product.category_slug,
			product.slug,
			requestedLanguage,
			activeDefaultLanguage,
			getLocalizedCatalogProductSlug(product),
			getLocalizedCatalogCategorySlug(product)
		);
		if (url.pathname !== canonicalPath) throw redirect(307, canonicalPath);

		return {
			pageType: 'product',
			routeId: 'products',
			searchTerm,
			category,
			product,
			products,
			categories
		};
	}

	throw error(404, 'Catalog route not found');
};
