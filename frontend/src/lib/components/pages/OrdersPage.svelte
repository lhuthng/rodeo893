<script>
	import { t } from '$lib/localization';
	import { getRoute } from '$lib/navigation/index.js';
	import PageShell from './PageShell.svelte';
</script>

<PageShell
	eyebrow={$t('ordersPage.eyebrow')}
	title={$t('ordersPage.title')}
	description={$t('ordersPage.description')}
>
	<div class="orders-layout">
		<div class="orders-timeline">
			{#each $t('ordersPage.states') as state, index}
				<div class="orders-step">
					<span>{index + 1}</span>
					<p>{state}</p>
				</div>
			{/each}
		</div>
		<a class="orders-link" href={$getRoute('products')}>{$t('cta.order')}</a>
	</div>
</PageShell>

<style lang="postcss">
	@reference '../../../app.css';

	.orders-layout {
		@apply mt-10 grid gap-6;
	}

	.orders-timeline {
		@apply grid gap-4 md:grid-cols-3;
	}

	.orders-step {
		@apply border p-5 transition-all duration-500;
		border-color: var(--theme-border);
		background-color: var(--theme-bg-elevated);

		& span {
			@apply font-mono text-xs tracking-widest2 uppercase transition-colors duration-500;
			color: var(--theme-accent);
		}

		& p {
			@apply mt-4 text-lg leading-relaxed transition-colors duration-500;
			color: var(--theme-fg-muted);
		}
	}

	.orders-link {
		@apply inline-flex w-fit items-center px-5 py-3 text-sm tracking-wider uppercase transition-colors duration-500;
		background-color: var(--theme-accent);
		color: var(--theme-accent-fg);

		&:hover {
			background-color: var(--theme-accent-alt);
			color: var(--theme-accent-alt-fg);
		}
	}
</style>
