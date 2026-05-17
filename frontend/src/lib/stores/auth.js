import { browser } from '$app/environment';
import { writable } from 'svelte/store';

const AUTH_KEY = 'rodeo893.auth';

const readAuthState = () => {
	if (!browser) {
		return false;
	}

	return !!localStorage.getItem(AUTH_KEY);
};

export const isAuthenticated = writable(readAuthState());

if (browser) {
	window.addEventListener('storage', (event) => {
		if (event.key !== AUTH_KEY) {
			return;
		}

		isAuthenticated.set(!!event.newValue);
	});
}

// Temporary helpers for local development before backend auth is integrated.
export const setAuthenticated = (value) => {
	if (!browser) {
		return;
	}

	if (value) {
		localStorage.setItem(AUTH_KEY, '1');
	} else {
		localStorage.removeItem(AUTH_KEY);
	}

	isAuthenticated.set(!!value);
};
