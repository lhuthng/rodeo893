<script>
	import { getRoute } from '$lib/navigation/index.js';
	import { t } from '$lib/localization';
	import PageShell from './PageShell.svelte';

	let { product } = $props();
	const details = $derived($t(`productDetails.${product.route}`));
</script>

<PageShell
	eyebrow={details('category')}
	title={details('name')}
	description={details('description')}
>
	<div class="detail-grid">
		<div class="detail-image">
			<img src={product.imageSrc} alt={product.imageAlt} loading="lazy" />
		</div>
		<div class="detail-copy">
			<div>
				<p class="detail-price">{product.priceFrom}</p>
				<ul>
					{#each details('notes') as note}
						<li>{note}</li>
					{/each}
				</ul>
			</div>
			<p class="detail-plating">{product.plating}</p>
			<div class="detail-actions">
				<a href={$getRoute('products')}>{$t('cta.exploreProducts')}</a>
				<a href={$getRoute('membership')}>{$t('cta.membership')}</a>
			</div>
		</div>
	</div>
</PageShell>

<style lang="postcss">
	@reference '../../../app.css';

	.detail-grid {
		@apply mt-10 grid gap-8 transition-all duration-500 md:grid-cols-[1.05fr_0.95fr];
	}

	.detail-image {
		@apply min-h-90 overflow-hidden border border-dashed transition-all duration-500;
		border-color: var(--theme-border);
		background-color: var(--theme-bg);

		& img {
			@apply h-full w-full object-cover;
		}
	}

	.detail-copy {
		@apply flex flex-col justify-between gap-8 border p-6 transition-all duration-500;
		border-color: var(--theme-border);
		background-color: var(--theme-bg-elevated);
		color: var(--theme-fg);

		& ul {
			@apply grid gap-3;
		}

		& li {
			@apply border-b pb-3 text-lg leading-relaxed transition-colors duration-500;
			border-color: var(--theme-border);
			color: var(--theme-fg-muted);
		}
	}

	.detail-price {
		@apply font-mono text-xs tracking-widest2 uppercase transition-colors duration-500;
		color: var(--theme-accent);
	}

	.detail-plating {
		@apply text-lg leading-relaxed transition-colors duration-500;
		color: var(--theme-fg-muted);
	}

	.detail-actions {
		@apply flex flex-wrap gap-4;

		& a {
			@apply inline-flex items-center px-5 py-3 text-sm tracking-wider uppercase transition-colors duration-500;
			border: 1px solid var(--theme-border);
			color: var(--theme-fg);

			&:hover {
				border-color: var(--theme-accent);
				color: var(--theme-accent);
			}
		}
	}
</style>
