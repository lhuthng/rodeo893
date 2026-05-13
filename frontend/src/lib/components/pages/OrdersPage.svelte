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
		@apply border border-espresso/10 bg-parchment p-5;

		& span {
			@apply font-mono text-xs uppercase tracking-[0.22em] text-crimson-bright;
		}

		& p {
			@apply mt-4 text-lg leading-relaxed text-mahogany;
		}
	}

	.orders-link {
		@apply inline-flex w-fit items-center px-5 py-3 bg-espresso text-parchment-light text-sm uppercase tracking-[0.18em] transition-colors duration-150 hover:bg-crimson-bright;
	}
</style>