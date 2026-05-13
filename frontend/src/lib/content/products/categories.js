export const productCategories = [
	{
		route: 'productsCookies',
		category: 'cookies',
		featuredProduct: 'productChocoChip'
	},
	{
		route: 'productsIceCream',
		category: 'iceCream',
		featuredProduct: 'productIceCreamCoffee'
	},
	{
		route: 'productsCoffee',
		category: 'coffee',
		featuredProduct: 'productBlueCheeseCoffee'
	}
];

export const productCategoriesByRoute = Object.fromEntries(productCategories.map((category) => [category.route, category]));