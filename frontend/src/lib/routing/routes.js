import { getLocalizedRouteName } from '$lib/localization/index.js';
import { allLanguages, getRouteName } from '$lib/localization/index.js';

export const routeTree = {
	home: [],
	products: ['products'],
	productsCookies: ['products', 'productsCookies'],
	productsIceCream: ['products', 'productsIceCream'],
	productsCoffee: ['products', 'productsCoffee'],
	productVanilla: ['products', 'productsCookies', 'productVanilla'],
	productMocha: ['products', 'productsCookies', 'productMocha'],
	productChocoChip: ['products', 'productsCookies', 'productChocoChip'],
	productPeanutButter: ['products', 'productsCookies', 'productPeanutButter'],
	productAlmondCrescent: ['products', 'productsCookies', 'productAlmondCrescent'],
	productIceCreamChoco: ['products', 'productsIceCream', 'productIceCreamChoco'],
	productIceCreamCoffee: ['products', 'productsIceCream', 'productIceCreamCoffee'],
	productBlueCheeseCoffee: ['products', 'productsCoffee', 'productBlueCheeseCoffee'],
	membership: ['membership'],
	login: ['login'],
	profile: ['profile'],
	account: ['account'],
	orders: ['orders'],
	about: ['about'],
	contact: ['contact']
};

const orderedRoutes = Object.entries(routeTree).sort((left, right) => right[1].length - left[1].length);

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