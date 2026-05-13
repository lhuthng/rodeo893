<script>
	import { t } from '$lib/localization';
	import { getRoute } from '$lib/navigation/index.js';
	import PageShell from './PageShell.svelte';

	const stats = $derived($t('membershipPage.stats'));
	const steps = $derived($t('membershipPage.steps'));
</script>

<PageShell
	eyebrow={$t('membershipPage.eyebrow')}
	title={$t('membershipPage.title')}
	description={$t('membershipPage.description')}
	tone="dark"
>
	<div class="membership-topline">
		<div class="membership-pill">{$t('membershipPage.badge')}</div>
		<p>{$t('membershipPage.reassurance')}</p>
	</div>

	<div class="membership-stats">
		{#each stats as stat}
			<div>
				<p>{stat.label}</p>
				<strong>{stat.value}</strong>
			</div>
		{/each}
	</div>

	<div class="membership-grid">
		<div class="membership-panel membership-panel--rich">
			<h2>{$t('membershipPage.stepsTitle')}</h2>
			<ol>
				{#each steps as step, index}
					<li>
						<span>{index + 1}</span>
						<p>{step}</p>
					</li>
				{/each}
			</ol>
			<div class="membership-benefits">
				{#each $t('membershipPage.benefits') as benefit}
					<p>{benefit}</p>
				{/each}
			</div>
		</div>
		<div class="membership-panel membership-panel--accent">
			<h2>{$t('membershipPage.conciergeTitle')}</h2>
			<p>{$t('membershipPage.conciergeText')}</p>
			<p class="membership-note">Membership status, activation, and future billing details align directly with the backend membership endpoints.</p>
			<div class="membership-actions">
				<a class="membership-primary" href={$getRoute('account')}>{$t('cta.membership')}</a>
				<a class="membership-secondary" href={$getRoute('products')}>{$t('cta.order')}</a>
			</div>
		</div>
	</div>
</PageShell>

<style lang="postcss">
	@reference '../../../app.css';

	.membership-topline {
		@apply mt-8 flex flex-wrap items-center gap-4 text-parchment;

		& p {
			@apply text-lg;
		}
	}

	.membership-pill {
		@apply inline-flex items-center border border-parchment-dark/50 bg-parchment/10 px-3 py-2 font-mono text-xs uppercase tracking-[0.2em] text-parchment-light;
	}

	.membership-stats {
		@apply mt-6 grid gap-4 md:grid-cols-3;

		& > div {
			@apply border border-parchment-dark/35 bg-espresso-light/75 p-4;
		}

		& p {
			@apply font-mono text-xs uppercase tracking-[0.2em] text-crimson-bright;
		}

		& strong {
			@apply mt-2 block font-display text-2xl tracking-tight text-parchment-light;
		}
	}

	.membership-grid {
		@apply mt-10 grid gap-6 md:grid-cols-2;
	}

	.membership-panel {
		@apply grid gap-4 border border-parchment-dark/40 bg-espresso-light p-6 text-lg leading-relaxed text-parchment;
	}

	.membership-panel--rich {
		& h2 {
			@apply font-display text-3xl text-parchment-light;
		}

		& ol {
			@apply grid gap-3;
		}

		& li {
			@apply flex gap-3 border-b border-parchment-dark/30 pb-3;
		}

		& li span {
			@apply font-mono text-xs uppercase tracking-[0.2em] text-crimson-bright;
		}

		& li p {
			@apply text-lg;
		}
	}

	.membership-benefits {
		@apply mt-2 grid gap-2;

		& p {
			@apply border border-parchment-dark/30 bg-espresso px-4 py-3;
		}
	}

	.membership-panel--accent {
		@apply bg-parchment text-espresso;

		& h2 {
			@apply font-display text-4xl leading-tight tracking-tight;
		}

		& p {
			@apply text-xl leading-relaxed;
		}
	}

	.membership-note {
		@apply border-l border-crimson-bright/55 pl-3 text-lg text-mahogany;
	}

	.membership-actions {
		@apply mt-3 flex flex-wrap gap-3;
}

	.membership-primary,
	.membership-secondary {
		@apply inline-flex w-fit items-center px-5 py-3 text-sm uppercase tracking-[0.18em] transition-colors duration-150;
	}

	.membership-primary {
		@apply bg-espresso text-parchment-light hover:bg-crimson-bright;
	}

	.membership-secondary {
		@apply border border-espresso/30 text-espresso hover:border-crimson-bright hover:text-crimson-bright;
	}
</style>