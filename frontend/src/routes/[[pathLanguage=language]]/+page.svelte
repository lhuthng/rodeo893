<script>
	import Metahead from '$lib/components/Metahead.svelte';
	import { getPageEntry } from '$lib/components/pages/index.js';
	import { pathRoute } from '$lib/navigation/index.js';

	const { data } = $props();
	const routeId = $derived(data.pathRoute);
	const routeEntry = $derived(getPageEntry(routeId));
	const Route = $derived(routeEntry?.component);
	const routeProps = $derived(routeEntry?.props ?? {});

	$effect(() => {
		pathRoute.set(routeId);
	});
</script>

<Metahead route={routeId} skip="true" />
{#if Route}
	<Route {...routeProps} />
{/if}
