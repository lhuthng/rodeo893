<script>
	import '../app.css';
	import { onMount } from "svelte";
	import { language } from "$lib/localization";

	let { children, data } = $props();

	onMount(() => {
		document.documentElement.lang = $language;
	});

	$effect(() => {
			document.documentElement.lang = $language;
	});
</script>

{@render children?.()}
