import { goto } from "$app/navigation";
import { derived, writable } from "svelte/store";

import { language, defaultLanguage } from "$lib/localization";
import { buildRoutePath } from '$lib/routing/routes.js';

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
    [getRoute, pathRoute],
    ([$getRoute, $pathRoute]) => (event, newLanguage) => {
        event.preventDefault();
        goto($getRoute($pathRoute, newLanguage));
    }
)