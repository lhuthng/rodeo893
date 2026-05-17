<script>
	import { getRoute, pathRoute, switchLanguage } from '$lib/navigation/index.js';
	import { allLanguages, language, t } from '$lib/localization';
	import { isAuthenticated } from '$lib/stores/auth.js';
	import Language from '../svgs/Language.svelte';

	let {
		isMenuLanguageDropdownOpen = $bindable(false),
		menuLanguagePickerEl = $bindable(null),
		onClose
	} = $props();

	const primaryItems = $derived($t('navigation.primary'));
	const currentRoute = $derived($pathRoute || 'home');
	const isProductsRoute = $derived(
		currentRoute === 'products' ||
			currentRoute.startsWith('products') ||
			currentRoute.startsWith('product')
	);
	const isMembershipRoute = $derived(currentRoute === 'membership');
	const currentLanguageCode = $derived($language || 'en');
	const availableLanguages = $derived(allLanguages().length ? allLanguages() : ['en', 'vi']);

	const onSelectLanguage = (event, languageCode) => {
		event.preventDefault();
		isMenuLanguageDropdownOpen = false;
		onClose();
		if (languageCode === currentLanguageCode) return;
		$switchLanguage(event, languageCode);
	};
</script>

<ol class="burger-menu__list">
	{#each primaryItems as item}
		<li>
			<a
				class:active={item.route === 'products' ? isProductsRoute : currentRoute === item.route}
				href={$getRoute(item.route)}
				onclick={onClose}
			>
				{item.label}
			</a>
		</li>
	{/each}
	<li class="border-t border-espresso/10 pt-0">
		{#if $isAuthenticated}
			<a class="auth-link" href={$getRoute('profile')} onclick={onClose}>
				{$t('navigation.profileLabel')}
			</a>
		{:else}
			<a class="auth-link" href={$getRoute('login')} onclick={onClose}>
				{$t('cta.membership')}
			</a>
		{/if}
	</li>
	<li bind:this={menuLanguagePickerEl}>
		<button
			type="button"
			class="lang-trigger"
			aria-haspopup="menu"
			aria-expanded={isMenuLanguageDropdownOpen}
			onclick={() => (isMenuLanguageDropdownOpen = !isMenuLanguageDropdownOpen)}
		>
			<Language class="h-4 w-4 fill-current" />
			<span>{currentLanguageCode.toUpperCase()}</span>
			<span class="lang-caret">{isMenuLanguageDropdownOpen ? '▴' : '▾'}</span>
		</button>
		{#if isMenuLanguageDropdownOpen}
			<div class="lang-dropdown" role="menu">
				{#each availableLanguages as langCode}
					<button
						type="button"
						role="menuitemradio"
						aria-checked={langCode === currentLanguageCode}
						class="lang-option"
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

<style lang="postcss">
	@reference "../../../app.css";

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
					@apply mr-2 inline-block transition-colors duration-150 content-['›'];
					color: var(--theme-accent);
				}

				&:hover::before {
					color: var(--theme-accent-fg);
				}
			}

			&.active:hover {
				color: var(--theme-accent-fg);
			}
		}

		& .auth-link {
			@apply inline-flex w-full items-center justify-center border-none px-6 py-4 font-mono text-sm tracking-[0.14em] whitespace-nowrap uppercase transition-colors duration-150;
			color: var(--theme-fg);

			&:hover {
				background-color: var(--theme-fg);
				color: var(--theme-bg);
			}
		}

		& .lang-trigger {
			@apply inline-flex w-full items-center justify-center gap-2 border-none px-6 py-4 font-mono text-sm tracking-[0.14em] uppercase transition-colors duration-150;
			color: var(--theme-fg);

			&:hover {
				background-color: var(--theme-accent);
				color: var(--theme-accent-fg);
			}
		}

		& .lang-caret {
			@apply text-xs;
		}

		& .lang-dropdown {
			@apply w-full;
			background-color: var(--theme-bg-elevated);
			border-top: 1px solid var(--theme-border);
		}

		& .lang-option {
			@apply inline-flex w-full cursor-pointer items-center justify-center px-6 py-3 font-mono text-xs tracking-[0.14em] uppercase transition-colors duration-150;
			color: var(--theme-fg);
			border-bottom: 1px solid var(--theme-border);

			&:hover {
				background-color: var(--theme-interactive);
				color: var(--theme-interactive-fg);
			}

			&.active {
				background-color: var(--theme-accent);
				color: var(--theme-accent-fg);
			}
		}
	}
</style>
