/**
 * Fetch all active products (localized) from the backend API.
 * @param {string} lang - Language code ('en' | 'vi')
 * @param {typeof fetch} fetchFn - The SvelteKit fetch (SSR-safe)
 * @param {string} apiBase - Base URL (empty for relative, full for SSR)
 * @returns {Promise<import('./types').LocalizedProduct[]>}
 */
export async function fetchProducts(lang, fetchFn, apiBase = '') {
	const res = await fetchFn(`${apiBase}/api/products?lang=${encodeURIComponent(lang)}`);
	if (!res.ok) {
		throw new Error(`Failed to fetch products: ${res.status}`);
	}
	return res.json();
}

/**
 * Fetch a single product by slug.
 * @param {string} slug
 * @param {string} lang
 * @param {typeof fetch} fetchFn
 * @param {string} apiBase
 * @returns {Promise<import('./types').LocalizedProduct | null>}
 */
export async function fetchProduct(slug, lang, fetchFn, apiBase = '') {
	const res = await fetchFn(
		`${apiBase}/api/products/${encodeURIComponent(slug)}?lang=${encodeURIComponent(lang)}`
	);
	if (!res.ok) return null;
	return res.json();
}

/**
 * Resolve incoming category/product slugs from any supported language to canonical slugs.
 * @param {{ category: string; product?: string; lang: string }} input
 * @param {typeof fetch} fetchFn
 * @param {string} apiBase
 * @returns {Promise<{
 *   category_slug: string;
 *   category_localized_slug: string;
 *   product_slug?: string | null;
 *   product_localized_slug?: string | null;
 * } | null>}
 */
export async function resolveCatalogPath(input, fetchFn, apiBase = '') {
	const params = new URLSearchParams();
	params.set('category', input.category);
	params.set('lang', input.lang);

	if (input.product) {
		params.set('product', input.product);
	}

	const res = await fetchFn(`${apiBase}/api/products/resolve-path?${params.toString()}`);
	if (res.status === 404) {
		return null;
	}
	if (!res.ok) {
		throw new Error(`Failed to resolve catalog path: ${res.status}`);
	}

	return res.json();
}
