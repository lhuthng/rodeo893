import { defaultLanguage } from '$lib/localization/index.js';
import { get } from 'svelte/store';

export const load = async ({ params }) => {
	let { pathLanguage } = params;
	pathLanguage ||= get(defaultLanguage);

	return {
		pathRoute: 'home'
	};
};