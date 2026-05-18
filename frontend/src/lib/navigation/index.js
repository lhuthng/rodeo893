import { goto } from "$app/navigation";
import { page } from "$app/stores";
import { derived, writable } from "svelte/store";

import { language, defaultLanguage } from "$lib/localization";
import { buildCatalogCategoryPath, buildCatalogProductPath, buildRoutePath } from '$lib/routing/routes.js';

export const pathRoute = writable("/");

export const getRoute = derived(
    [language, defaultLanguage],
    ([$language, $defaultLanguage]) => (route, newLanguage) => {
        const lang = newLanguage || $language || $defaultLanguage;
        const finalPath = buildRoutePath(route, lang, $defaultLanguage);
        return finalPath === '' ? '/' : finalPath;
    }
);

export const switchLanguage = derived(
    [getRoute, pathRoute, page, defaultLanguage],
    ([$getRoute, $pathRoute, $page, $defaultLanguage]) => (event, newLanguage) => {
        event.preventDefault();

        const activeDefaultLanguage = $defaultLanguage || 'en';
        const currentData = $page?.data ?? {};
        const currentPageType = currentData.pageType;
        const currentRouteId = currentData.routeId;

        let nextPath;
        if (currentRouteId === 'products') {
            if (currentPageType === 'category' && currentData.category?.slug) {
                nextPath = buildCatalogCategoryPath(
                    currentData.category.slug,
                    newLanguage,
                    activeDefaultLanguage,
                    currentData.category.localized_slug
                );
            } else if (currentPageType === 'product' && currentData.product?.slug) {
                nextPath = buildCatalogProductPath(
                    currentData.product.category_slug,
                    currentData.product.slug,
                    newLanguage,
                    activeDefaultLanguage,
                    currentData.product.localized_slug,
                    currentData.product.localized_category_slug
                );
            } else {
                nextPath = $getRoute('products', newLanguage);
            }
        } else {
            nextPath = $getRoute($pathRoute, newLanguage);
        }

        const search = $page?.url?.search ?? '';
        goto(`${nextPath}${search}`, { noScroll: true, keepFocus: true });
    }
)