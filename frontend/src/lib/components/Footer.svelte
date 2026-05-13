<script>
	import { getRoute } from '$lib/navigation/index.js';
	import { t } from '$lib/localization';

	const primaryItems = $derived($t('navigation.primary'));
</script>

<footer class="w-full bg-espresso text-parchment-light p-12">
  <div class="grid gap-8 max-w-300 mx-auto md:grid-cols-[1.2fr_1fr_1fr] md:items-center">
    <div class="space-y-3">
	      <div class="font-display font-bold text-xl [&>em]:italic [&>em]:text-crimson-bright">{$t('brand.name')}</div>
      <p class="max-w-90 text-lg text-parchment">{$t('home.story.description')}</p>
    </div>
    <ul class="grid gap-3 list-none text-parchment font-mono text-xs tracking-wider uppercase">
      {#each primaryItems as item}
        <li><a class="transition-colors duration-150 hover:text-crimson-bright" href={$getRoute(item.route)}>{item.label}</a></li>
      {/each}
    </ul>
    <div class="space-y-3 text-right max-md:text-left">
      <a class="inline-flex px-4 py-2 border border-parchment-dark text-sm uppercase tracking-wider transition-colors duration-150 hover:bg-crimson-bright hover:text-parchment-light" href={$getRoute('membership')}>{$t('cta.membership')}</a>
      <div class="font-mono text-xs text-parchment tracking-wider">&copy; 2026 · {$t('brand.footerNote')}</div>
    </div>
  </div>
</footer>