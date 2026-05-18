import AboutPage from './AboutPage.svelte';
import AccountPage from './AccountPage.svelte';
import ContactPage from './ContactPage.svelte';
import HomePage from './HomePage.svelte';
import LoginPage from './LoginPage.svelte';
import MembershipPage from './MembershipPage.svelte';
import OrdersPage from './OrdersPage.svelte';
import ProfilePage from './ProfilePage.svelte';

const pageRegistry = {
	home: { component: HomePage },
	membership: { component: MembershipPage },
	login:      { component: LoginPage },
	profile:    { component: ProfilePage },
	account:    { component: AccountPage },
	orders:     { component: OrdersPage },
	about:      { component: AboutPage },
	contact:    { component: ContactPage }
};

export const getPageEntry = (route) => pageRegistry[route];
