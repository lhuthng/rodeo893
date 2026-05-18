<script>
	import { page } from '$app/stores';
	import { getRoute } from '$lib/navigation/index.js';
	import { t } from '$lib/localization';
	import PageShell from './PageShell.svelte';

	let { category = null, products = [] } = $props();
	const activeCategory = $derived(category ?? $page.data?.category ?? null);
	const categoryProducts = $derived(
		products.length
			? products
			: ($page.data?.products ?? []).filter(
					(product) => product.category_slug === activeCategory?.slug
				)
	);
	const details = $derived(
		activeCategory?.name ||
			categoryProducts[0]?.category_slug
				?.split('-')
				.map((part) => part.charAt(0).toUpperCase() + part.slice(1))
				.join(' ') ||
			$t('productsPage.title')
	);
	const featuredPath = $derived(
		activeCategory?.featured_path ?? categoryProducts[0]?.path ?? $getRoute('products')
	);

	const formatPrice = (amount, currency) => `${Number(amount).toLocaleString('vi-VN')} ${currency}`;
</script>

<PageShell
	eyebrow={$t('productsPage.eyebrow')}
	title={details}
	description={activeCategory?.description || $t('productsPage.description')}
>
	<div class="category-hero">
		<div class="category-hero__copy">
			<p>{$t('cta.exploreProducts')}</p>
			<a href={featuredPath}>{$t('cta.order')}</a>
		</div>
		<div class="category-hero__list">
			{#each categoryProducts as product}
				<a href={product.path}>
					{#if product.image_url}
						<img src={product.image_url} alt={product.image_alt ?? ''} loading="lazy" />
					{/if}
					<span>{product.name}</span>
					<small>{formatPrice(product.base_price, product.currency)}</small>
				</a>
			{/each}
		</div>
	</div>
</PageShell>

<style lang="postcss">
	@reference '../../../app.css';

	.category-hero {
		@apply mt-10 grid gap-6 transition-all duration-500 md:grid-cols-[0.85fr_1.15fr];
	}

	.category-hero__copy {
		@apply border px-6 py-8 transition-all duration-500;
		border-color: var(--theme-border);
		background-color: var(--theme-bg-elevated);
		color: var(--theme-fg);

		& p {
			@apply font-display text-4xl tracking-tight;
			transition: color 500ms ease;
		}

		& a {
			@apply mt-6 inline-flex w-fit items-center px-5 py-3 text-sm tracking-wider uppercase transition-colors duration-500;
			border: 1px solid var(--theme-border);
			color: var(--theme-fg);

			&:hover {
				border-color: var(--theme-accent);
				color: var(--theme-accent);
			}
		}
	}

	.category-hero__list {
		@apply grid gap-4;

		& a {
			@apply border p-5 transition-all duration-500 hover:-translate-y-1;
			border-color: var(--theme-border);
			background-color: var(--theme-bg-elevated);
		}

		& img {
			@apply mb-4 h-36 w-full border border-dashed object-cover transition-all duration-500;
			border-color: var(--theme-border);
			background-color: var(--theme-bg);
		}

		& span {
			@apply block font-display text-2xl;
			transition: color 500ms ease;
			color: var(--theme-fg);
		}

		& small {
			@apply mt-2 block text-lg leading-relaxed;
			transition: color 500ms ease;
			color: var(--theme-fg-muted);
		}
	}
</style>
