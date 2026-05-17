import { fetchProducts } from '$lib/api/products.js';

/**
 * Server-only layout load: fetches the product catalogue once per page request.
 * `BACKEND_URL` is set in docker-compose; defaults to localhost for local dev.
 * Data is merged with +layout.js data and available as $page.data.products everywhere.
 */
export const load = async ({ fetch, params }) => {
	const lang = params.pathLanguage || 'en';
	const apiBase = process.env.BACKEND_URL ?? 'http://localhost:3000';
	const products = await fetchProducts(lang, fetch, apiBase);
	return { products };
};
