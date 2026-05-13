import { addTranslation, allLanguages, allRoutesOf, defaultLanguage } from "$lib/localization";

// import de from '$lib/translations/de.json';
import en from '$lib/translations/en.json';
import vi from '$lib/translations/vi.json';

addTranslation('en', en);
// addTranslation('de', de);
addTranslation('vi', vi);

defaultLanguage.set('en');