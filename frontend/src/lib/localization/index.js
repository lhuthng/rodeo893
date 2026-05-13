import { derived, get, writable } from "svelte/store";
import { translate } from "./translator";

const translations = {};

const routeMap = {};
const reversedRouteMap = {};

export const addTranslation = (languageCode, translation) => {
    translations[languageCode] = translation;
    for (let routeName in translation.routes) {
        routeMap[`${languageCode}.${translation.routes[routeName]}`] = routeName;
        reversedRouteMap[`${languageCode}.${routeName}`] = translation.routes[routeName];
    }
}

export const defaultLanguage = writable(null);
export const language = writable(null);

const currentTranslation = derived(
    [defaultLanguage, language], ([$default, $current]) => ({
        ...translations[$default],
        ...translations[$current]
    })
);

export const t = derived(
    currentTranslation, ($current) => (keyword, modifier) => translate($current, keyword, modifier)
);

export const extract = (t, ...keys) => {
    let result = {}
    keys.forEach(key => result[key] = t(key));
    return result;
}

export const getRouteName = (routePath) => routeMap[routePath];
export const getLocalizedRouteName = (routeNamePath) => reversedRouteMap[routeNamePath];
export const allLanguages = () => Object.keys(translations);
export const allRoutesOf = (language) => Object.values(translations[language]["routes"]);