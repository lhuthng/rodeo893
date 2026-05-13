<script>
	let { eyebrow, title, description, tone = 'light', children } = $props();
</script>

<section class:dark={tone === 'dark'} class="page-shell">
	<div class="page-shell__inner">
		<p class="page-shell__eyebrow">{eyebrow}</p>
		<h1 class="page-shell__title">{title}</h1>
		<p class="page-shell__description">{description}</p>
		{@render children?.()}
	</div>
</section>

<style lang="postcss">
	@reference '../../../app.css';

	.page-shell {
		@apply relative px-6 pt-36 pb-18 md:px-10;
	}

	.page-shell.dark {
		@apply bg-espresso text-parchment-light;
	}

	.page-shell__inner {
		@apply mx-auto max-w-300;
	}

	.page-shell__eyebrow {
		@apply mb-4 font-mono text-xs uppercase tracking-[0.24em] text-crimson-bright;
	}

	.page-shell__title {
		@apply max-w-180 font-display text-5xl leading-tight tracking-tight md:text-6xl;
	}

	.page-shell__description {
		@apply mt-5 max-w-110 text-lg leading-relaxed text-mahogany;
	}

	.dark .page-shell__description {
		@apply text-parchment;
	}
</style>