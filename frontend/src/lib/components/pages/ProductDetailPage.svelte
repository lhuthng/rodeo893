<script>
	import { getRoute } from '$lib/navigation/index.js';
	import { t } from '$lib/localization';
	import PageShell from './PageShell.svelte';

	let { product = null } = $props();

	const formatPrice = (amount, currency) =>
		`${Number(amount).toLocaleString('vi-VN')} ${currency}`;
</script>

{#if product}
<PageShell
	eyebrow={product.category_slug}
	title={product.name}
	description={product.description}
>
	<div class="detail-grid">
		<div class="detail-image">
			{#if product.image_url}
				<img src={product.image_url} alt={product.image_alt ?? ''} loading="lazy" />
			{/if}
		</div>
		<div class="detail-copy">
			<div>
				<p class="detail-price">{formatPrice(product.base_price, product.currency)}</p>
				{#if product.notes?.length}
					<ul>
						{#each product.notes as note}
							<li>{note}</li>
						{/each}
					</ul>
				{/if}
			</div>
			{#if product.plating}
				<p class="detail-plating">{product.plating}</p>
			{/if}
			<div class="detail-actions">
				<a href={$getRoute('products')}>{$t('cta.exploreProducts')}</a>
				<a href={$getRoute('membership')}>{$t('cta.membership')}</a>
			</div>
		</div>
	</div>
</PageShell>
{/if}

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
