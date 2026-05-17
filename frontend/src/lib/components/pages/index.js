import AboutPage from './AboutPage.svelte';
import AccountPage from './AccountPage.svelte';
import ContactPage from './ContactPage.svelte';
import HomePage from './HomePage.svelte';
import LoginPage from './LoginPage.svelte';
import MembershipPage from './MembershipPage.svelte';
import OrdersPage from './OrdersPage.svelte';
import ProfilePage from './ProfilePage.svelte';
import ProductDetailPage from './ProductDetailPage.svelte';
import ProductCategoryPage from './ProductCategoryPage.svelte';
import ProductsPage from './ProductsPage.svelte';
import { productCategoriesByRoute, productsByCategory, productsByRoute } from '$lib/content/products/index.js';

const baseRegistry = {
	home: { component: HomePage },
	products: { component: ProductsPage },
	productsCookies: {
		component: ProductCategoryPage,
		props: {
			...productCategoriesByRoute.productsCookies,
			products: productsByCategory.cookies ?? []
		}
	},
	productsIceCream: {
		component: ProductCategoryPage,
		props: {
			...productCategoriesByRoute.productsIceCream,
			products: productsByCategory.iceCream ?? []
		}
	},
	productsCoffee: {
		component: ProductCategoryPage,
		props: {
			...productCategoriesByRoute.productsCoffee,
			products: productsByCategory.coffee ?? []
		}
	},
	membership: { component: MembershipPage },
	login: { component: LoginPage },
	profile: { component: ProfilePage },
	account: { component: AccountPage },
	orders: { component: OrdersPage },
	about: { component: AboutPage },
	contact: { component: ContactPage }
};

const productRegistry = Object.fromEntries(
	Object.entries(productsByRoute).map(([route, product]) => [
		route,
		{ component: ProductDetailPage, props: { product } }
	])
);

const pageRegistry = {
	...baseRegistry,
	...productRegistry
};

export const getPageEntry = (route) => pageRegistry[route];