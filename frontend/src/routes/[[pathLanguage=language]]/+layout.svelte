<script>
	import { page, navigating } from '$app/stores';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { language } from '$lib/localization/index.js';
	import { setContext } from 'svelte';

	let { children, data } = $props();
	let portal1 = $state();
	setContext('portal-1', () => portal1);

	$effect(() => {
		language.set(data.pathLanguage);
	});

	// Clear dark theme as soon as navigation starts (not after the new page commits).
	// This ensures the header transitions in sync with the page leaving, not after it loads.
	const isDarkPage = $derived($page.data?.routeId === 'membership' && !$navigating);
</script>

<Header />
<div bind:this={portal1} class="pointer-events-none fixed z-150 h-dvh w-screen"></div>
<main class="page-bg" data-theme={isDarkPage ? 'dark' : undefined}>
	{@render children()}
</main>
<Footer />

<style lang="postcss">
	@reference '../../app.css';

	.page-bg {
		@apply transition-[background-color,color] duration-500;
		background-color: var(--theme-bg);
		color: var(--theme-fg);
	}
</style>
