/**
 * Fetch all active products (localized) from the backend API.
 * @param {string} lang - Language code ('en' | 'vi')
 * @param {typeof fetch} fetchFn - The SvelteKit fetch (SSR-safe)
 * @param {string} apiBase - Base URL (empty for relative, full for SSR)
 * @returns {Promise<import('./types').LocalizedProduct[]>}
 */
export async function fetchProducts(lang, fetchFn, apiBase = '') {
	try {
		const res = await fetchFn(`${apiBase}/api/products?lang=${encodeURIComponent(lang)}`);
		if (!res.ok) return [];
		return res.json();
	} catch {
		return [];
	}
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
	try {
		const res = await fetchFn(
			`${apiBase}/api/products/${encodeURIComponent(slug)}?lang=${encodeURIComponent(lang)}`
		);
		if (!res.ok) return null;
		return res.json();
	} catch {
		return null;
	}
}
