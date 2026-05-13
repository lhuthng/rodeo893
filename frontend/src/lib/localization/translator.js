export const translate = (root, keyword, modifier) => {
    let current = root;
    if (keyword) {
        for (const part of keyword.split('.')) {
            if (typeof current === 'object' && current !== null && !Array.isArray(current) && part in current) {
                current = current[part];
            }
            else {
                return keyword;
            }
        }
    }

    if (Array.isArray(current)) {
        if (modifier) {
            return current.map(item => modifier(item));
        }
        return current;
    }

    if (typeof current === 'object') {
        const scopedTranslator = (keyword, newModifier) => translate(current, keyword, newModifier ?? modifier);
        return scopedTranslator;
    }

    if (modifier) {
        return modifier(current);
    }

    return current;
}