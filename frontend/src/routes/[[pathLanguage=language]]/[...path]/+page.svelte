<script>
	import Metahead from '$lib/components/Metahead.svelte';
	import { getPageEntry } from '$lib/components/pages/index.js';
	import { pathRoute } from '$lib/navigation/index.js';

	const { data } = $props();
	const routeId = $derived(data.routeId);
	const entry = $derived(getPageEntry(routeId));
	const Route = $derived(entry?.component);
	const routeProps = $derived({
		...(entry?.props ?? {}),
		searchTerm: data.searchTerm ?? '',
		products: data.products ?? [],
		product: (data.products ?? []).find((p) => p.frontend_key === routeId) ?? null
	});

	$effect(() => {
		pathRoute.set(routeId);
	});
</script>

<Metahead route={routeId} />
{#if Route}
	<Route {...routeProps} />
{/if}
