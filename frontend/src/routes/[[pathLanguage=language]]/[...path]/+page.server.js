import { get } from 'svelte/store';
import { redirect } from '@sveltejs/kit';

import { allLanguages, defaultLanguage, getRouteName } from '$lib/localization/index.js';
import { tryResolveCatalogRoute } from '$lib/routing/server/catalog.js';
import { buildIncomingPath, buildRoutePath } from '$lib/routing/routes.js';
import { resolveStaticRoute } from '$lib/routing/server/static.js';

export const trailingSlash = 'never';

const resolveRouteKeyForSegment = (segment, languagePriority) => {
	for (const language of languagePriority) {
		const routeKey = getRouteName(`${language}.${segment}`);
		if (routeKey) return routeKey;
	}

	return null;
};

export const load = async ({ parent, params, url, fetch }) => {
	const activeDefaultLanguage = get(defaultLanguage);
	const requestedLanguage = params.pathLanguage || activeDefaultLanguage;
	const segments = params.path.split('/').filter(Boolean);
	const apiBase = process.env.BACKEND_URL ?? 'http://localhost:3000';
	const languagePriority = [
		requestedLanguage,
		activeDefaultLanguage,
		...allLanguages().filter(
			(language) => language !== requestedLanguage && language !== activeDefaultLanguage
		)
	];
	const searchTerm = url.searchParams.get('search') ?? '';

	if (segments.length > 0 && resolveRouteKeyForSegment(segments[0], languagePriority) === 'products') {
		const canonicalProductsRoot = buildRoutePath(
			'products',
			requestedLanguage,
			activeDefaultLanguage
		);
		const canonicalProductsSegment =
			canonicalProductsRoot.split('/').filter(Boolean).at(-1) ?? segments[0];

		if (segments[0] !== canonicalProductsSegment) {
			const canonicalIncomingPath = buildIncomingPath(
				requestedLanguage,
				[canonicalProductsSegment, ...segments.slice(1)],
				activeDefaultLanguage
			);
			if (url.pathname !== canonicalIncomingPath) {
				throw redirect(307, canonicalIncomingPath);
			}
		}

		const {
			products = [],
			categories = [],
			featuredProducts = [],
			catalogError = null
		} = await parent();

		const resolvedCatalogRoute = await tryResolveCatalogRoute({
			segments,
			products,
			categories,
			requestedLanguage,
			activeDefaultLanguage,
			searchTerm,
			url,
			fetch,
			apiBase
		});

		if (resolvedCatalogRoute) {
			return {
				...resolvedCatalogRoute,
				featuredProducts,
				catalogError
			};
		}
	}

	return resolveStaticRoute({
		segments,
		requestedLanguage,
		activeDefaultLanguage,
		searchTerm,
		params,
		url
	});
};
