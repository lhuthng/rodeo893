<script>
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
</script>

<Header />
<div bind:this={portal1} class="pointer-events-none fixed z-150 h-dvh w-screen"></div>
{@render children()}
<Footer />
