<script>
	import { t } from '$lib/localization';
	import PageShell from './PageShell.svelte';
</script>

<PageShell
	eyebrow={$t('accountPage.eyebrow')}
	title={$t('accountPage.title')}
	description={$t('accountPage.description')}
>
	<div class="panel-grid">
		{#each $t('accountPage.panels') as panel}
			<div class="panel-card">{panel}</div>
		{/each}
	</div>
</PageShell>

<style lang="postcss">
	@reference '../../../app.css';

	.panel-grid {
		@apply mt-10 grid gap-5 md:grid-cols-3;
	}

	.panel-card {
		@apply border border-espresso/10 bg-parchment-light p-6 text-lg leading-relaxed text-mahogany;
	}
</style>