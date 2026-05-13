<script>
    import { t } from "$lib/localization";

    let { skip, route } = $props();
</script>

<svelte:head>
    <title>{$t('titles.' + route)}</title>
    <meta property="og:title" content={$t(`titles.${route}`)} />

    <meta name="description" content={$t(`meta-descriptions.${route}`)}/>
    <meta name="og:description" content={$t(`meta-descriptions.${route}`)}/>
</svelte:head>