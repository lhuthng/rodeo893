<script>
	import { getRoute } from '$lib/navigation/index.js';
	import { t } from '$lib/localization';

	let { products = [] } = $props();

	const featuredProducts = $derived(products.slice(0, 3));
	const hero = $derived($t('home.hero'));
	const featured = $derived($t('home.featured'));
	const membership = $derived($t('home.membership'));
	const process = $derived($t('home.process'));
	const story = $derived($t('home.story'));
	const finalCta = $derived($t('home.finalCta'));
	const marqueeItems = [
		'Rodeo',
		'Cookies & Cakes',
		'Pre-order Only',
		'Ice Cream',
		'Small Batch',
		'Coffee',
		'Real Butter',
		'Dark Chocolate',
		'No Shortcuts'
	];
</script>

{#snippet marquee(list, ariaHidden = false)}
	{#each list as element}
		<span>{element}</span><span aria-hidden={ariaHidden ? 'true' : 'false'}>&starf;</span>
	{/each}
{/snippet}

<section class="hero">
	<div class="hero__inner">
		<div class="hero__copy">
			<p class="hero__eyebrow">{hero('eyebrow')}</p>
			<h1 class="hero__title">
				<span>{hero('lineOne')}</span>
				<span>{hero('lineTwo')}</span>
				<em>{hero('lineThree')}</em>
			</h1>
			<p class="hero__description">{hero('description')}</p>
			<div class="hero__actions">
				<a class="hero__primary" href={$getRoute('membership')}>{$t('cta.membership')}</a>
				<a class="hero__secondary" href={$getRoute('products')}>{$t('cta.order')}</a>
			</div>
			<p class="hero__note">{hero('membershipNote')}</p>
		</div>
		<div class="hero__panel">
			<div class="hero__card">
				<p class="hero__panel-label">{$t('home.featured.eyebrow')}</p>
				<div class="hero__panel-grid">
					{#each featuredProducts as product}
						<a class="hero__product" href={$getRoute(product.frontend_key)}>
							<p class="hero__product-title">{product.name}</p>
							<p class="hero__product-copy">{product.description}</p>
						</a>
					{/each}
				</div>
			</div>
		</div>
	</div>
</section>

<section class="home-marquee">
	<div class="home-marquee__track">
		{@render marquee(marqueeItems)}
		{@render marquee(marqueeItems, true)}
	</div>
</section>

<section class="section-grid">
	<div class="section-card">
		<p class="section-card__eyebrow">{featured('eyebrow')}</p>
		<h2 class="section-card__title">{featured('title')}</h2>
		<p class="section-card__description">{featured('description')}</p>
		<a class="section-card__link" href={$getRoute('products')}>{$t('cta.viewAll')}</a>
	</div>
	<div class="section-card section-card--dark">
		<p class="section-card__eyebrow">{membership('eyebrow')}</p>
		<h2 class="section-card__title">{membership('title')}</h2>
		<p class="section-card__description">{membership('description')}</p>
		<a class="section-card__link" href={$getRoute('membership')}>{$t('cta.membership')}</a>
	</div>
</section>

<section class="process">
	<div class="process__header">
		<p class="section-card__eyebrow">{process('eyebrow')}</p>
		<h2 class="section-card__title">{process('title')}</h2>
	</div>
	<ol class="process__list">
		{#each process('steps') as step, index}
			<li>
				<span>{index + 1}</span>
				<p>{step}</p>
			</li>
		{/each}
	</ol>
</section>

<section class="story-band">
	<div>
		<p class="section-card__eyebrow">{story('eyebrow')}</p>
		<h2 class="section-card__title">{story('title')}</h2>
	</div>
	<p class="story-band__description">{story('description')}</p>
	<a class="section-card__link" href={$getRoute('about')}>{$t('cta.learnStory')}</a>
</section>

<section class="final-cta">
	<h2>{finalCta('title')}</h2>
	<p>{finalCta('description')}</p>
	<div class="hero__actions hero__actions--centered">
		<a class="hero__primary" href={$getRoute('membership')}>{$t('cta.membership')}</a>
		<a class="hero__secondary" href={$getRoute('products')}>{$t('cta.order')}</a>
	</div>
</section>

<style lang="postcss">
	@reference '../../../app.css';

	.hero {
		@apply relative flex min-h-208 w-full h-screen items-center px-6 pb-14 pt-32 md:px-10 md:pb-18;

		&::before {
			@apply pointer-events-none absolute inset-x-0 top-0 bottom-100 z-10 content-[""] bg-parchment/40;
		}

		&::after {
			@apply pointer-events-none absolute inset-x-0 bottom-0 z-10 h-100 content-[""] bg-parchment/40;
			min-width: 80rem;
			mask-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1440 40' preserveAspectRatio='none'%3E%3Cpath d='M0 0 L1440 0 L1440 30 C1200 30 960 40 720 30 C480 20 240 30 0 30 L0 40 Z' fill='black'/%3E%3C/svg%3E");
			mask-size: 100% 100%;
		}
	}

	.hero__inner,
	.section-grid,
	.process,
	.story-band,
	.final-cta {
		@apply mx-auto max-w-300;
	}

	.hero__inner {
		@apply relative z-20 grid w-full gap-10 md:grid-cols-2 md:items-end;
	}

	.hero__copy {
		@apply max-w-130;
	}

	.hero__eyebrow {
		@apply mb-5 font-mono text-xs uppercase tracking-[0.24em] text-crimson-bright;
	}

	.hero__title {
		@apply font-display text-[clamp(4.2rem,9vw,7.2rem)] leading-[0.93] tracking-tight text-espresso;

		& > span,
		& > em {
			@apply block;
		}

		& > em {
			@apply text-right italic text-crimson-bright md:text-left;
		}
	}

	.hero__description {
		@apply mt-7 max-w-90 text-xl leading-relaxed text-mahogany;
	}

	.hero__actions {
		@apply mt-8 flex flex-wrap gap-4;
	}

	.hero__actions--centered {
		@apply justify-center;
	}

	.hero__primary,
	.hero__secondary,
	.section-card__link {
		@apply inline-flex items-center px-5 py-3 text-sm uppercase tracking-wider transition-colors duration-150;
	}

	.hero__primary {
		@apply bg-espresso text-parchment-light hover:bg-crimson-bright;
	}

	.hero__secondary,
	.section-card__link {
		@apply border border-espresso/20 text-espresso hover:border-crimson-bright hover:text-crimson-bright;
	}

	.hero__note {
		@apply mt-6 max-w-80 border-l border-crimson-bright/40 pl-4 text-base leading-relaxed text-mahogany;
	}

	.hero__panel {
		@apply relative;
	}

	.hero__card {
		@apply rounded-none border border-espresso/10 bg-parchment px-5 py-6 shadow-card;
	}

	.hero__panel-label,
	.section-card__eyebrow {
		@apply mb-3 font-mono text-xs uppercase tracking-widest2 text-crimson-bright;
	}

	.hero__panel-grid {
		@apply grid gap-4;
	}

	.hero__product {
		@apply block border border-parchment-dark/60 bg-parchment-light p-4 transition-transform duration-150 hover:-translate-y-1;
	}

	.hero__product-title,
	.section-card__title,
	.final-cta h2 {
		@apply font-display text-3xl tracking-tight text-espresso;
	}

	.hero__product-copy,
	.section-card__description,
	.story-band__description,
	.final-cta p {
		@apply mt-2 text-lg leading-relaxed text-mahogany;
	}

	.home-marquee {
		@apply overflow-hidden bg-espresso px-4 py-8 text-lg text-parchment-dark/60;
	}

	.home-marquee__track {
		@apply flex gap-16 whitespace-nowrap animate-[marquee_20s_linear_infinite] select-none font-extrabold;

		& :global(span:nth-child(2n)) {
			@apply text-xs text-crimson-bright/60 translate-y-2;
		}
	}

	.section-grid {
		@apply grid gap-6 px-6 py-8 md:grid-cols-2 md:px-10;
	}

	.section-card {
		@apply border border-espresso/10 bg-parchment-light p-7;
	}

	.section-card--dark {
		@apply bg-espresso text-parchment-light;

		.section-card__title,
		.section-card__description,
		.section-card__link {
			@apply text-parchment-light;
		}

		.section-card__link {
			@apply border-parchment-dark hover:border-crimson-bright hover:text-crimson-bright;
		}
	}

	.process {
		@apply px-6 py-10 md:px-10;
	}

	.process__list {
		@apply mt-8 grid gap-4 md:grid-cols-3;

		& > li {
			@apply border border-espresso/10 bg-parchment p-5;
		}

		& span {
			@apply font-mono text-xs uppercase tracking-widest2 text-crimson-bright;
		}

		& p {
			@apply mt-4 text-lg leading-relaxed text-mahogany;
		}
	}

	.story-band {
		@apply grid gap-6 px-6 py-10 md:grid-cols-[0.8fr_1fr_auto] md:items-end md:px-10;
	}

	.final-cta {
		@apply px-6 py-16 text-center md:px-10;
	}
</style>