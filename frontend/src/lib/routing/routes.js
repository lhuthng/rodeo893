import { getLocalizedRouteName } from '$lib/localization/index.js';
import { allLanguages, getRouteName } from '$lib/localization/index.js';

export const routeTree = {
	home: [],
	products: ['products'],
	membership: ['membership'],
	login: ['login'],
	profile: ['profile'],
	account: ['account'],
	orders: ['orders'],
	about: ['about'],
	contact: ['contact']
};

const orderedRoutes = Object.entries(routeTree).sort((left, right) => right[1].length - left[1].length);

export const slugifyCatalogSegment = (value) => {
	if (!value) return '';

	return value
		.toString()
		.normalize('NFD')
		.replace(/[\u0300-\u036f]/g, '')
		.toLowerCase()
		.replace(/[^a-z0-9]+/g, '-')
		.replace(/^-+|-+$/g, '')
		.replace(/-{2,}/g, '-');
};

export const getLocalizedCatalogCategorySlug = (category) => {
	if (typeof category === 'string') {
		return slugifyCatalogSegment(category);
	}

	if (category?.category_localized_slug) return category.category_localized_slug;
	if (category?.localized_slug) return category.localized_slug;
	if (category?.slug) return slugifyCatalogSegment(category.slug);

	return '';
};

export const getLocalizedCatalogCategoryName = (category) => {
	if (typeof category === 'string') {
		return category
			.split('-')
			.map((part) => part.charAt(0).toUpperCase() + part.slice(1))
			.join(' ');
	}

	if (category?.category_name) return category.category_name;
	if (category?.name) return category.name;

	return getLocalizedCatalogCategoryName(category?.slug ?? '');
};

export const getLocalizedCatalogProductSlug = (product) => {
	if (product?.localized_slug) return product.localized_slug;
	if (product?.name) {
		const generated = slugifyCatalogSegment(product.name);
		if (generated) return generated;
	}

	return product?.slug ?? '';
};

export const buildProductsRootPath = (language, defaultLanguage) =>
	buildRoutePath('products', language, defaultLanguage);

export const buildCatalogCategoryPath = (
	categorySlug,
	language,
	defaultLanguage,
	localizedCategorySlug = categorySlug
) => {
	const rootPath = buildProductsRootPath(language, defaultLanguage);
	const safeCategorySlug = localizedCategorySlug || categorySlug;

	return `${rootPath}/${safeCategorySlug}`.replace(/\/+/g, '/');
};

export const buildCatalogProductPath = (
	categorySlug,
	productSlug,
	language,
	defaultLanguage,
	localizedProductSlug = productSlug,
	localizedCategorySlug = categorySlug
) => {
	const categoryPath = buildCatalogCategoryPath(
		categorySlug,
		language,
		defaultLanguage,
		localizedCategorySlug
	);
	const safeProductSlug = localizedProductSlug || productSlug;
	return `${categoryPath}/${safeProductSlug}`.replace(/\/+/g, '/');
};

export const buildRoutePath = (routeId, language, defaultLanguage) => {
	const segments = routeTree[routeId];

	if (!segments) {
		return '/';
	}

	const localizedSegments = segments
		.map((segment) => getLocalizedRouteName(`${language}.${segment}`))
		.filter(Boolean);

	if (localizedSegments.length === 0) {
		return '/';
	}

	const languagePrefix = language && language !== defaultLanguage ? `/${language}` : '';
	return `${languagePrefix}/${localizedSegments.join('/')}`.replace(/\/+/g, '/');
};

export const buildIncomingPath = (requestedLanguage, segments, defaultLanguage) => {
	const languagePrefix = requestedLanguage && requestedLanguage !== defaultLanguage ? `/${requestedLanguage}` : '';
	return `${languagePrefix}/${segments.filter(Boolean).join('/')}`.replace(/\/+/g, '/');
};

export const matchRoutePath = (segments, languageCandidates) => {
	const normalizedSegments = segments.filter(Boolean);

	for (const language of languageCandidates) {
		for (const [routeId, routeSegments] of orderedRoutes) {
			const localizedSegments = routeSegments.map((segment) => getLocalizedRouteName(`${language}.${segment}`));
			if (localizedSegments.length === normalizedSegments.length && localizedSegments.every((segment, index) => segment === normalizedSegments[index])) {
				return { routeId, language };
			}
		}
	}

	return null;
};

export const pathToSegments = (...parts) => parts.filter(Boolean);

export const resolveRouteMatch = (segments, languageCandidates) => {
	const matched = matchRoutePath(segments, languageCandidates);

	if (!matched) {
		return null;
	}

	return {
		...matched,
		canonicalPath: buildRoutePath(matched.routeId, matched.language)
	};
};

export const resolveRouteBySegments = (segments, requestedLanguage, defaultLanguage) => {
	const normalizedSegments = segments.filter(Boolean);
	const languagePriority = [requestedLanguage, defaultLanguage, ...allLanguages().filter((language) => language !== requestedLanguage && language !== defaultLanguage)];
	const routeKeySegments = normalizedSegments.map((segment) => {
		for (const language of languagePriority) {
			const routeKey = getRouteName(`${language}.${segment}`);
			if (routeKey) {
				return routeKey;
			}
		}

		return null;
	});

	if (routeKeySegments.some((segment) => !segment)) {
		return null;
	}

	for (const [routeId, routeSegments] of orderedRoutes) {
		if (routeSegments.length !== routeKeySegments.length) {
			continue;
		}

		if (routeSegments.every((segment, index) => segment === routeKeySegments[index])) {
			return {
				routeId,
				canonicalPath: buildRoutePath(routeId, requestedLanguage, defaultLanguage)
			};
		}
	}

	return null;
};