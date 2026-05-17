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

// Category slug (from DB) → translation key used by ProductCategoryPage
export const CATEGORY_SLUG_TO_KEY = {
	cookies: 'cookies',
	'ice-cream': 'iceCream',
	coffee: 'coffee'
};

const pageRegistry = {
	home:       { component: HomePage },
	products:   { component: ProductsPage },
	productsCookies: {
		component: ProductCategoryPage,
		props: { category: 'cookies', category_slug: 'cookies', featuredProduct: 'productChocoChip' }
	},
	productsIceCream: {
		component: ProductCategoryPage,
		props: { category: 'iceCream', category_slug: 'ice-cream', featuredProduct: 'productIceCreamCoffee' }
	},
	productsCoffee: {
		component: ProductCategoryPage,
		props: { category: 'coffee', category_slug: 'coffee', featuredProduct: 'productBlueCheeseCoffee' }
	},
	// Individual product pages – data comes from `product` prop via +page.svelte
	productVanilla:          { component: ProductDetailPage },
	productMocha:            { component: ProductDetailPage },
	productChocoChip:        { component: ProductDetailPage },
	productPeanutButter:     { component: ProductDetailPage },
	productAlmondCrescent:   { component: ProductDetailPage },
	productIceCreamChoco:    { component: ProductDetailPage },
	productIceCreamCoffee:   { component: ProductDetailPage },
	productBlueCheeseCoffee: { component: ProductDetailPage },
	membership: { component: MembershipPage },
	login:      { component: LoginPage },
	profile:    { component: ProfilePage },
	account:    { component: AccountPage },
	orders:     { component: OrdersPage },
	about:      { component: AboutPage },
	contact:    { component: ContactPage }
};

export const getPageEntry = (route) => pageRegistry[route];
