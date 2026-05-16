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
		@apply border p-6 text-lg leading-relaxed transition-all duration-500;
		border-color: var(--theme-border);
		background-color: var(--theme-bg-elevated);
		color: var(--theme-fg-muted);
	}
</style>
