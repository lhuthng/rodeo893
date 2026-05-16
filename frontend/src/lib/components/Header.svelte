<script>
	import Logo from './svgs/Logo.svelte';
	import { getRoute, pathRoute, switchLanguage } from '$lib/navigation/index.js';
	import { allLanguages, language, t } from '$lib/localization';
	import Burger from './svgs/Burger.svelte';
	import Language from './svgs/Language.svelte';
	import { getContext } from 'svelte';
	import Portal from './Portal.svelte';
	import { fade, fly } from 'svelte/transition';

	const primaryItems = $derived($t('navigation.primary'));
	const accountLink = $derived($t('navigation.account'));
	const currentRoute = $derived($pathRoute || 'home');
	const isProductsRoute = $derived(
		currentRoute === 'products' ||
			currentRoute.startsWith('products') ||
			currentRoute.startsWith('product')
	);
	const isMembershipRoute = $derived(currentRoute === 'membership');
	const currentLanguageCode = $derived($language || 'en');
	const availableLanguages = $derived(allLanguages().length ? allLanguages() : ['en', 'vi']);

	const getNavPortalTarget = getContext('portal-1');

	//
	const BREAKPOINT_LG = 1024;
	let isBurgerOpen = $state(false);
	let isLanguageDropdownOpen = $state(false);
	let isMenuLanguageDropdownOpen = $state(false);
	let desktopLanguagePickerEl = $state(null);
	let menuLanguagePickerEl = $state(null);
	let innerWidth = $state();
	$effect(() => {
		if (innerWidth > BREAKPOINT_LG) {
			isBurgerOpen = false;
			isMenuLanguageDropdownOpen = false;
		}
	});

	$effect(() => {
		if (!isLanguageDropdownOpen && !isMenuLanguageDropdownOpen) return;

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
		};

		const closeOnEscape = (event) => {
			if (event.key !== 'Escape') return;
			isLanguageDropdownOpen = false;
			isMenuLanguageDropdownOpen = false;
		};

		document.addEventListener('pointerdown', closeOnOutside);
		document.addEventListener('keydown', closeOnEscape);

		return () => {
			document.removeEventListener('pointerdown', closeOnOutside);
			document.removeEventListener('keydown', closeOnEscape);
		};
	});

	const onSelectLanguage = (event, languageCode) => {
		event.preventDefault();
		isLanguageDropdownOpen = false;
		isMenuLanguageDropdownOpen = false;
		isBurgerOpen = false;
		if (languageCode === currentLanguageCode) return;
		$switchLanguage(event, languageCode);
	};

	const closeAllMenus = () => {
		isBurgerOpen = false;
		isLanguageDropdownOpen = false;
		isMenuLanguageDropdownOpen = false;
	};
</script>

<svelte:window bind:innerWidth />

<nav class="z-200" data-theme={isMembershipRoute ? 'dark' : undefined}>
	<div class="relative flex h-full justify-between">
		<div class="nav-links__burger">
			<button onclick={() => (isBurgerOpen = !isBurgerOpen)}
				><Burger class="h-8 w-8" toggled={isBurgerOpen} /></button
			>

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
			<li><a class="membership-link" href={$getRoute('membership')}>{$t('cta.membership')}</a></li>
			<li class="relative h-full w-14">
				<a class="ribbon" class:ribbon--open={isBurgerOpen} href={$getRoute('products')}>
					<span class="relative z-20 block w-full px-2 py-4 text-center uppercase not-lg:text-sm"
						>{$t('cta.order')}</span
					>
				</a>
			</li>
		</ol>
	</div>
</nav>
{#if isBurgerOpen}
	<Portal class="pointer-events-auto" target={getNavPortalTarget()}>
		<button
			class="menu-backdrop"
			aria-label="backdrop"
			in:fade={{ duration: 500 }}
			out:fade={{ duration: 500 }}
			onclick={closeAllMenus}
		></button>
		<div
			class="burger-menu-panel"
			data-theme={isMembershipRoute ? 'dark' : undefined}
			in:fly={{ y: '-100%', duration: 500 }}
			out:fly={{ y: '-100%', duration: 500 }}
		>
			<ol class="burger-menu__list">
				{#each primaryItems as item}
					<li>
						<a
							class:active={item.route === 'products'
								? isProductsRoute
								: currentRoute === item.route}
							href={$getRoute(item.route)}
							onclick={() => (isBurgerOpen = false)}
						>
							{item.label}
						</a>
					</li>
				{/each}
				<li class="mt-4 border-t border-espresso/10 pt-4">
					<a class="membership-link" href={$getRoute('membership')} onclick={closeAllMenus}>
						{$t('cta.membership')}
					</a>
				</li>
				<li bind:this={menuLanguagePickerEl}>
					<button
						type="button"
						class="language-trigger language-trigger--menu"
						aria-haspopup="menu"
						aria-expanded={isMenuLanguageDropdownOpen}
						onclick={() => (isMenuLanguageDropdownOpen = !isMenuLanguageDropdownOpen)}
					>
						<Language class="h-4 w-4 fill-current" />
						<span>{currentLanguageCode.toUpperCase()}</span>
						<span class="language-caret">{isMenuLanguageDropdownOpen ? '▴' : '▾'}</span>
					</button>
					{#if isMenuLanguageDropdownOpen}
						<div class="language-dropdown language-dropdown--menu" role="menu">
							{#each availableLanguages as langCode}
								<button
									type="button"
									role="menuitemradio"
									aria-checked={langCode === currentLanguageCode}
									class="language-option language-option--menu"
									class:active={langCode === currentLanguageCode}
									onclick={(event) => onSelectLanguage(event, langCode)}
								>
									{langCode.toUpperCase()}
								</button>
							{/each}
						</div>
					{/if}
				</li>
			</ol>
		</div>
	</Portal>
{/if}

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
	}

	.nav-links {
		@apply flex shrink grow items-center justify-center font-mono text-sm tracking-wider whitespace-nowrap uppercase not-lg:hidden;

		& > li > a {
			@apply relative block px-4 py-4 transition-colors duration-500;
			color: var(--theme-fg);

			&::before {
				@apply absolute right-1/2 bottom-2 left-1/2 h-0.5 opacity-0 transition-all duration-100 content-[''];
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
		@apply relative z-10 max-h-dvh overflow-y-auto pt-16 lg:pt-24;
		background-color: var(--theme-bg);
		color: var(--theme-fg);
	}

	.burger-menu__list {
		@apply flex flex-col font-mono text-sm tracking-wider uppercase;

		& > li > a {
			@apply block px-6 py-3 transition-colors duration-150;
			border-bottom: 1px solid var(--theme-border);

			&:hover {
				@apply font-bold;
				background-color: var(--theme-accent);
				color: var(--theme-accent-fg);
			}

			&.active {
				@apply font-bold;
				color: var(--theme-fg-active);

				&::before {
					content: '›';
					@apply mr-2 inline-block;
					color: var(--theme-accent);
				}
			}

			&.active:hover {
				color: var(--theme-accent-fg);
			}
		}

		& > li.membership-link {
			@apply px-0 py-0;
		}

		& .membership-link {
			@apply inline-flex w-full items-center justify-center border-none px-6 py-4 font-mono text-sm tracking-[0.14em] whitespace-nowrap uppercase transition-colors duration-150;
			color: var(--theme-fg);

			&:hover {
				border: none;
				background-color: var(--theme-fg);
				color: var(--theme-bg);
			}
		}

		& .language-trigger--menu {
			@apply inline-flex w-full items-center justify-center gap-2 border-none px-6 py-4 font-mono text-sm tracking-[0.14em] uppercase transition-colors duration-150;
			color: var(--theme-fg);

			&:hover {
				background-color: var(--theme-accent);
				color: var(--theme-accent-fg);
			}
		}

		& .language-dropdown--menu {
			@apply static mt-2 w-full p-0 shadow-none;
			background-color: transparent;
			border-color: var(--theme-border);
		}

		& .language-option--menu {
			@apply px-6 py-3;
			border-bottom: 1px solid var(--theme-border);
		}
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

		&.ribbon--open {
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
		@apply absolute z-9 h-full w-full cursor-not-allowed backdrop-blur-xl;
	}
</style>
