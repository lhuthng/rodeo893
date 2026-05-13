const supportedLanguages = ['', 'en', 'vi'];

export function match(lang) {
    return supportedLanguages.includes(lang);
}