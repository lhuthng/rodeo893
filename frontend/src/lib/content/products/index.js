import { almondCrescentProduct } from './almond-crescent.js';
import { blueCheeseCoffeeProduct } from './blue-cheese-coffee.js';
import { chocoChipProduct } from './choco-chip.js';
import { iceCreamChocoProduct } from './ice-cream-choco.js';
import { iceCreamCoffeeProduct } from './ice-cream-coffee.js';
import { mochaProduct } from './mocha.js';
import { peanutButterProduct } from './peanut-butter.js';
import { vanillaProduct } from './vanilla.js';
import { productCategories, productCategoriesByRoute } from './categories.js';

export const products = [
	vanillaProduct,
	mochaProduct,
	chocoChipProduct,
	peanutButterProduct,
	almondCrescentProduct,
	iceCreamChocoProduct,
	iceCreamCoffeeProduct,
	blueCheeseCoffeeProduct
];

export const productsByRoute = Object.fromEntries(products.map((product) => [product.route, product]));

export const productsByCategory = products.reduce((categories, product) => {
	const list = categories[product.category] ?? [];
	list.push(product);
	categories[product.category] = list;
	return categories;
}, {});

export { productCategories, productCategoriesByRoute };