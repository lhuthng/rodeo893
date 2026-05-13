export const trailingSlash = 'never';

import { defaultLanguage } from '$lib/localization/index.js';
import { error } from '@sveltejs/kit';
import { get } from 'svelte/store';

export const load = async ({ params }) => {
	
	let { pathLanguage } = params;
	pathLanguage ||= get(defaultLanguage);
	return {
			pathLanguage,
	}
}