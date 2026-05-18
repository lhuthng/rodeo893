import { redirect } from '@sveltejs/kit';

import { buildRoutePath, resolveRouteBySegments } from '$lib/routing/routes.js';

export const resolveStaticRoute = ({
	segments,
	requestedLanguage,
	activeDefaultLanguage,
	searchTerm,
	params,
	url
}) => {
	let resolved = null;
	let matchedLength = segments.length;

	while (matchedLength > 0) {
		resolved = resolveRouteBySegments(
			segments.slice(0, matchedLength),
			requestedLanguage,
			activeDefaultLanguage
		);
		if (resolved) break;
		matchedLength--;
	}

	if (!resolved) {
		const langPrefix = params.pathLanguage ? `/${requestedLanguage}` : '';
		throw redirect(307, langPrefix || '/');
	}

	if (matchedLength < segments.length) {
		throw redirect(307, resolved.canonicalPath);
	}

	if (url.pathname !== resolved.canonicalPath) {
		throw redirect(307, resolved.canonicalPath);
	}

	if (resolved.routeId === 'account') {
		throw redirect(307, buildRoutePath('profile', requestedLanguage, activeDefaultLanguage));
	}

	return {
		pageType: 'static',
		routeId: resolved.routeId,
		searchTerm
	};
};
