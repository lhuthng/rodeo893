<script>
	import { goto } from '$app/navigation';
	import { navigating, page } from '$app/stores';
	import Logo from './svgs/Logo.svelte';
	import { getRoute, pathRoute, switchLanguage } from '$lib/navigation/index.js';
	import { allLanguages, language, t } from '$lib/localization';
	import Burger from './svgs/Burger.svelte';
	import Language from './svgs/Language.svelte';
	import Search from './svgs/Search.svelte';
	import { isAuthenticated } from '$lib/stores/auth.js';
	import { getContext, tick } from 'svelte';
	import { fly } from 'svelte/transition';
	import Portal from './Portal.svelte';
	import SearchPanel from './header/SearchPanel.svelte';
	import BurgerMenu from './header/BurgerMenu.svelte';
	import {
		animatePanelLeave,
		animatePanelEnter,
		animateBackdrop,
		Flip,
		PANEL_MS
	} from '$lib/animations/panel.js';

	const primaryItems = $derived($t('navigation.primary'));
	const currentRoute = $derived($pathRoute || 'home');
	const isProductsRoute = $derived(
		currentRoute === 'products' ||
			currentRoute.startsWith('products') ||
			currentRoute.startsWith('product')
	);
	// Clear dark theme as soon as navigation starts — same timing as the layout.
	const isMembershipRoute = $derived($page.data?.routeId === 'membership' && !$navigating);
	const currentLanguageCode = $derived($language || 'en');
	const availableLanguages = $derived(allLanguages().length ? allLanguages() : ['en', 'vi']);
	const searchQueryLink = $derived(
		`${$getRoute('products')}?search=${encodeURIComponent(debouncedSearchTerm.trim())}`
	);

	const searchResults = $derived.by(() => {
		const query = debouncedSearchTerm.trim().toLowerCase();
		if (!query) return [];
		const all = $page.data?.products ?? [];
		return all
			.filter(
				(p) => p.name.toLowerCase().includes(query) || p.description.toLowerCase().includes(query)
			)
			.slice(0, 4);
	});

	const totalSearchMatches = $derived.by(() => {
		const query = debouncedSearchTerm.trim().toLowerCase();
		if (!query) return 0;
		const all = $page.data?.products ?? [];
		return all.filter(
			(p) => p.name.toLowerCase().includes(query) || p.description.toLowerCase().includes(query)
		).length;
	});

	const getNavPortalTarget = getContext('portal-1');

	const SEARCH_DEBOUNCE_MS = 1000;
	const BREAKPOINT_LG = 1024;
	let currentPanel = $state(null);
	let panelTransitionInFlight = $state(false);
	let isSearchOpen = $derived(currentPanel === 'search');
	let isBurgerOpen = $derived(currentPanel === 'burger');
	let isLanguageDropdownOpen = $state(false);
	let isMenuLanguageDropdownOpen = $state(false);
	let searchTerm = $state('');
	let debouncedSearchTerm = $state('');
	let searchInputEl = $state(null);
	let panelStackEl = $state(null);
	let backdropEl = $state(null);
	let searchPanelEl = $state(null);
	let searchControlsEl = $state(null);
	let desktopLanguagePickerEl = $state(null);
	let menuLanguagePickerEl = $state(null);
	let searchDebounceTimer = null;
	let queuedPanelTarget = null;
	let innerWidth = $state();

	const runPanelTransition = async (targetPanel) => {
		if (!panelStackEl || currentPanel === targetPanel) return;

		const isOpening = currentPanel === null;
		const isClosing = targetPanel === null;

		const leavingPanel = panelStackEl.querySelector('[data-panel]');
		if (leavingPanel) {
			if (isClosing) animateBackdrop(backdropEl, false); // simultaneous with leave
			await animatePanelLeave(leavingPanel);
		}

		const prevState = Flip.getState('[data-panel]');
		currentPanel = null;
		await tick();

		if (isClosing) return;

		currentPanel = targetPanel;
		await tick();

		if (isOpening) animateBackdrop(backdropEl, true); // simultaneous with enter

		Flip.from(prevState, { duration: PANEL_MS / 1000, ease: 'power2.inOut', absolute: true });

		const enteringPanel = panelStackEl.querySelector('[data-panel]');
		if (!enteringPanel) return;

		await animatePanelEnter(enteringPanel);
		if (targetPanel === 'search') searchInputEl?.focus();
	};

	const transitionToPanel = async (targetPanel) => {
		if (panelTransitionInFlight) {
			queuedPanelTarget = targetPanel;
			return;
		}

		panelTransitionInFlight = true;
		let nextTarget = targetPanel;

		while (true) {
			queuedPanelTarget = null;
			await runPanelTransition(nextTarget);

			if (queuedPanelTarget === null) break;
			nextTarget = queuedPanelTarget;
		}

		panelTransitionInFlight = false;
	};

	$effect(() => {
		if (innerWidth > BREAKPOINT_LG && isBurgerOpen) {
			isMenuLanguageDropdownOpen = false;
			transitionToPanel(null);
		}
	});

	$effect(() => {
		if (!isLanguageDropdownOpen && !isMenuLanguageDropdownOpen && !isSearchOpen) return;

		const closeOnOutside = (event) => {
			const target = event.target;
			if (
				isLanguageDropdownOpen &&
				desktopLanguagePickerEl &&
				!desktopLanguagePickerEl.contains(target)
			) {
				isLanguageDropdownOpen = false;
			}
			if (
				isMenuLanguageDropdownOpen &&
				menuLanguagePickerEl &&
				!menuLanguagePickerEl.contains(target)
			) {
				isMenuLanguageDropdownOpen = false;
			}
			if (
				isSearchOpen &&
				searchPanelEl &&
				!searchPanelEl.contains(target) &&
				searchControlsEl &&
				!searchControlsEl.contains(target)
			) {
				transitionToPanel(null);
			}
		};

		const closeOnEscape = (event) => {
			if (event.key !== 'Escape') return;
			isLanguageDropdownOpen = false;
			isMenuLanguageDropdownOpen = false;
			if (isSearchOpen || isBurgerOpen) transitionToPanel(null);
		};

		document.addEventListener('pointerdown', closeOnOutside);
		document.addEventListener('keydown', closeOnEscape);

		return () => {
			document.removeEventListener('pointerdown', closeOnOutside);
			document.removeEventListener('keydown', closeOnEscape);
		};
	});

	$effect(() => {
		const query = searchTerm.trim();
		clearTimeout(searchDebounceTimer);
		searchDebounceTimer = setTimeout(() => {
			debouncedSearchTerm = query;
		}, SEARCH_DEBOUNCE_MS);

		return () => clearTimeout(searchDebounceTimer);
	});

	const onSelectLanguage = (event, languageCode) => {
		event.preventDefault();
		isLanguageDropdownOpen = false;
		if (isBurgerOpen) transitionToPanel(null);
		if (languageCode === currentLanguageCode) return;
		$switchLanguage(event, languageCode);
	};

	const closeAllMenus = () => {
		transitionToPanel(null);
		isLanguageDropdownOpen = false;
		isMenuLanguageDropdownOpen = false;
	};

	const toggleBurgerMenu = () => {
		if (isBurgerOpen) isMenuLanguageDropdownOpen = false;
		transitionToPanel(isBurgerOpen ? null : 'burger');
	};

	const toggleSearchPanel = () => transitionToPanel(isSearchOpen ? null : 'search');

	const submitSearch = async (event) => {
		event.preventDefault();
		if (!searchTerm.trim()) return;
		await goto(`${$getRoute('products')}?search=${encodeURIComponent(searchTerm.trim())}`);
		transitionToPanel(null);
	};
</script>

<svelte:window bind:innerWidth />

<nav class="z-200" data-theme={isMembershipRoute ? 'dark' : undefined}>
	<div class="relative flex h-full justify-between">
		<div class="nav-links__burger">
			<button class:open={isBurgerOpen} onclick={toggleBurgerMenu}
				><Burger class="h-8 w-8" />
			</button>

			<ol class="hidden">
				{#each primaryItems as item}
					<li>
						<a
							class:active={item.route === 'products'
								? isProductsRoute
								: currentRoute === item.route}
							href={$getRoute(item.route)}>{item.label}</a
						>
					</li>
				{/each}
			</ol>
		</div>
		<div class="nav-brand">
			<a href={$getRoute('home')}
				><Logo class="h-14 w-14 overflow-hidden rounded-xs  lg:h-24 lg:w-24" /></a
			>
			<div class="nav-brand__text">
				<a class="nav-brand__main" href={$getRoute('home')}>{$t('brand.name')}</a>
				<span class="nav-brand__sub">{$t('brand.subtitle')}</span>
			</div>
		</div>
		<ol class="nav-links">
			{#each primaryItems as item}
				<li>
					<a
						class:active={item.route === 'products' ? isProductsRoute : currentRoute === item.route}
						href={$getRoute(item.route)}>{item.label}</a
					>
				</li>
			{/each}
		</ol>
		<ol class="flex items-center gap-8 pr-4">
			<li bind:this={searchControlsEl}>
				<button
					type="button"
					class="search-trigger"
					class:open={isSearchOpen}
					aria-haspopup="dialog"
					aria-expanded={isSearchOpen}
					onclick={toggleSearchPanel}
				>
					<Search class="h-4 w-4" />
				</button>
			</li>
			<li class="language-picker not-lg:hidden" bind:this={desktopLanguagePickerEl}>
				<button
					type="button"
					class="language-trigger"
					aria-haspopup="menu"
					aria-expanded={isLanguageDropdownOpen}
					onclick={() => (isLanguageDropdownOpen = !isLanguageDropdownOpen)}
				>
					<Language class="h-4 w-4 fill-current" />
					<span>{currentLanguageCode.toUpperCase()}</span>
					<span class="language-caret">{isLanguageDropdownOpen ? '▴' : '▾'}</span>
				</button>
				{#if isLanguageDropdownOpen}
					<div in:fly={{ y: -10 }} out:fly={{ y: 10 }} class="language-dropdown" role="menu">
						{#each availableLanguages as langCode}
							<button
								type="button"
								role="menuitemradio"
								aria-checked={langCode === currentLanguageCode}
								class="language-option"
								class:active={langCode === currentLanguageCode}
								onclick={(event) => onSelectLanguage(event, langCode)}
							>
								{langCode.toUpperCase()}
							</button>
						{/each}
					</div>
				{/if}
			</li>
			{#if $isAuthenticated}
				<li>
					<a
						class="profile-trigger"
						href={$getRoute('profile')}
						aria-label={$t('navigation.profileLabel')}
					>
						<span>◉</span>
					</a>
				</li>
			{:else}
				<li><a class="membership-link" href={$getRoute('login')}>{$t('cta.membership')}</a></li>
			{/if}
			<li class="relative h-full w-14">
				<a class="ribbon" class:open={currentPanel !== null} href={$getRoute('products')}>
					<span class="relative z-20 block w-full px-2 py-4 text-center uppercase not-lg:text-sm"
						>{$t('cta.order')}</span
					>
				</a>
			</li>
		</ol>
	</div>
</nav>
<Portal class="pointer-events-auto" target={getNavPortalTarget()}>
	<button
		class="menu-backdrop"
		aria-label="close panels"
		bind:this={backdropEl}
		onclick={closeAllMenus}
	></button>
	<div class="panel-stack" bind:this={panelStackEl}>
		{#if currentPanel === 'search'}
			<div
				class="search-panel"
				data-panel="search"
				bind:this={searchPanelEl}
				data-theme={isMembershipRoute ? 'dark' : undefined}
			>
				<SearchPanel
					bind:inputEl={searchInputEl}
					bind:searchTerm
					{searchResults}
					{totalSearchMatches}
					{debouncedSearchTerm}
					{searchQueryLink}
					onSubmit={submitSearch}
					onClose={closeAllMenus}
				/>
			</div>
		{:else if currentPanel === 'burger'}
			<div
				class="burger-menu-panel"
				data-panel="burger"
				data-theme={isMembershipRoute ? 'dark' : undefined}
			>
				<BurgerMenu
					bind:isMenuLanguageDropdownOpen
					bind:menuLanguagePickerEl
					onClose={closeAllMenus}
				/>
			</div>
		{/if}
	</div>
</Portal>

<style lang="postcss">
	@reference "../../app.css";

	nav {
		@apply fixed inset-x-0 top-0 flex h-16 items-center justify-center font-display backdrop-blur-md transition-[background-color,border-color,color] duration-500 lg:h-24;
		background-color: var(--theme-bg-nav);
		color: var(--theme-fg);
		border-bottom: 1px solid var(--theme-border);

		& > div {
			@apply m-auto w-full max-w-300;
		}
	}

	.nav-brand {
		@apply absolute top-1/2 left-1/2 gap-2 not-lg:-translate-1/2 lg:static lg:flex;

		.nav-brand__text {
			@apply my-auto flex flex-col whitespace-nowrap not-lg:hidden;
		}

		.nav-brand__main {
			@apply text-3xl font-semibold tracking-tight transition-colors duration-500;
			color: var(--theme-fg);
		}

		.nav-brand__sub {
			@apply font-mono text-base tracking-wider transition-colors duration-500;
			color: var(--theme-fg);
		}
	}

	.membership-link {
		@apply inline-flex h-10 items-center px-3 py-2 font-mono text-xs tracking-[0.14em] whitespace-nowrap uppercase transition-colors duration-500 not-lg:hidden;
		border: 1px solid var(--theme-border);
		color: var(--theme-fg);

		&:hover {
			border-color: var(--theme-fg);
			background-color: var(--theme-interactive);
			color: var(--theme-interactive-fg);
		}
	}

	.profile-trigger,
	.search-trigger {
		@apply inline-flex h-10 w-10 cursor-pointer items-center justify-center transition-colors duration-150;
		border: 1px solid var(--theme-border);
		color: var(--theme-fg);

		&:hover {
			border-color: var(--theme-fg);
			background-color: var(--theme-interactive);
			color: var(--theme-interactive-fg);
		}
	}

	.search-trigger.open {
		border-color: var(--theme-fg);
		background-color: var(--theme-interactive);
		color: var(--theme-interactive-fg);
	}

	.profile-trigger {
		@apply font-mono text-xs;
	}

	.language-picker {
		@apply relative;
	}

	.language-trigger {
		@apply inline-flex h-10 cursor-pointer items-center gap-2 px-3 py-2 font-mono text-xs tracking-[0.14em] uppercase transition-colors duration-500;
		border: 1px solid var(--theme-border);
		color: var(--theme-fg);

		& span {
			@apply leading-none;
		}

		&:hover {
			border-color: var(--theme-fg);
			background-color: var(--theme-interactive);
			color: var(--theme-interactive-fg);
		}
	}

	.language-caret {
		@apply text-xs;
	}

	.language-dropdown {
		@apply absolute top-[calc(100%+0.35rem)] right-0 z-20 min-w-18 p-1 shadow-lift;
		border: 1px solid var(--theme-border);
		background-color: var(--theme-bg-elevated);
	}

	.language-option {
		@apply inline-flex w-full cursor-pointer items-center justify-center px-3 py-2 font-mono text-xs tracking-[0.14em] uppercase transition-colors duration-150 not-lg:text-sm;
		color: var(--theme-fg);

		&:hover {
			background-color: var(--theme-interactive);
			color: var(--theme-interactive-fg);
		}

		&.active {
			background-color: var(--theme-accent);
			color: var(--theme-accent-fg);
		}
	}

	.nav-links__burger {
		@apply relative flex items-center lg:hidden;

		button {
			@apply my-2 ml-2 cursor-pointer transition-colors duration-150;

			border: 1px solid var(--theme-border);

			&:hover {
				@apply bg-(--theme-interactive) text-(--theme-interactive-fg) [&>svg>path]:stroke-(--theme-interactive-fg);
			}
		}

		button.open {
			@apply bg-(--theme-interactive) text-(--theme-interactive-fg) [&>svg>path]:stroke-(--theme-interactive-fg);
		}
	}

	.nav-links {
		@apply flex shrink grow items-center justify-center font-mono text-sm tracking-wider whitespace-nowrap uppercase not-lg:hidden;

		& > li > a {
			@apply relative block px-4 py-4 transition-colors duration-500;
			color: var(--theme-fg);

			&::before {
				@apply absolute right-1/2 bottom-2 left-1/2 h-0.5 opacity-0 transition-all duration-500 content-[''];
				background-color: var(--theme-accent);
			}

			&:hover {
				@apply font-bold;
				background-color: var(--theme-accent);
				color: var(--theme-accent-fg);

				&::before {
					@apply right-2 left-2 opacity-100;
					background-color: var(--theme-accent-fg) !important;
				}
			}

			&.active {
				color: var(--theme-fg-active);

				&::before {
					@apply right-3 left-3 opacity-100;
					background-color: var(--theme-accent);
				}
			}

			&.active:hover {
				color: var(--theme-accent-fg);
			}
		}
	}

	.burger-menu-panel {
		@apply absolute z-10 w-full overflow-y-auto pt-16 transition-[background-color,color,border-color] duration-500 lg:pt-24;
		background-color: var(--theme-bg);
		color: var(--theme-fg);
	}

	.panel-stack {
		@apply fixed inset-x-0 top-0 z-10;
	}

	.search-panel {
		@apply absolute z-10 mx-auto mt-16 w-full border p-4 transition-[background-color,color,border-color] duration-500 lg:mt-24;
		border-color: var(--theme-border);
		background-color: var(--theme-bg-elevated);
	}

	.ribbon {
		@apply absolute -inset-x-2 -top-3 flex h-[calc(100%+0.75rem)] items-center justify-center transition-transform duration-150 not-lg:translate-y-1 hover:translate-y-3;

		span {
			@apply relative z-20 block w-full px-2 py-4 text-center uppercase transition-colors duration-500 not-lg:text-sm;
			color: var(--theme-accent-fg);
		}

		&::before,
		&::after {
			@apply absolute transition-colors duration-500 content-[""];
			background-color: var(--theme-accent);
		}

		&::before {
			@apply inset-x-0 top-0 bottom-2;
		}
		&::after {
			@apply inset-x-0 top-full h-6 -translate-y-2;
			clip-path: polygon(0% 0%, 100% 0%, 100% 100%, 50% 0%, 0% 100%);
		}

		&.open {
			span {
				color: var(--theme-accent-alt-fg);
			}

			&::after,
			&::before {
				background-color: var(--theme-accent-alt);
			}
		}
	}

	.menu-backdrop {
		@apply fixed inset-0 z-8 h-full w-full cursor-pointer backdrop-blur-xl;
		opacity: 0;
		visibility: hidden;
		display: none;
		pointer-events: none;
	}
</style>
