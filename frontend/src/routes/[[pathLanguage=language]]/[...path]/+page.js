import { redirect } from '@sveltejs/kit';
import { get } from 'svelte/store';

import { defaultLanguage } from '$lib/localization/index.js';
import { buildRoutePath, resolveRouteBySegments } from '$lib/routing/routes.js';

export const trailingSlash = 'never';

export const load = async ({ params, url }) => {
	const activeDefaultLanguage = get(defaultLanguage);
	const requestedLanguage = params.pathLanguage || activeDefaultLanguage;
	const segments = params.path.split('/').filter(Boolean);

	// Walk up the path until a route matches (longest match wins)
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
		// Nothing matched at all — go to language root
		const langPrefix = params.pathLanguage ? `/${requestedLanguage}` : '';
		throw redirect(307, langPrefix || '/');
	}

	// Partial match (some trailing segments were unrecognised) — redirect to resolved parent
	if (matchedLength < segments.length) {
		throw redirect(307, resolved.canonicalPath);
	}

	// Cross-language or non-canonical slug — redirect to canonical path
	if (url.pathname !== resolved.canonicalPath) {
		throw redirect(307, resolved.canonicalPath);
	}

	if (resolved.routeId === 'account') {
		throw redirect(307, buildRoutePath('profile', requestedLanguage, activeDefaultLanguage));
	}

	return {
		routeId: resolved.routeId,
		searchTerm: url.searchParams.get('search') ?? ''
	};
};
