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
	theme="dark"
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
			<p class="membership-note">
				Membership status, activation, and future billing details align directly with the backend
				membership endpoints.
			</p>
			<div class="membership-actions">
				<a class="membership-primary" href={$getRoute('profile')}>{$t('cta.membership')}</a>
				<a class="membership-secondary" href={$getRoute('products')}>{$t('cta.order')}</a>
			</div>
		</div>
	</div>
</PageShell>

<style lang="postcss">
	@reference '../../../app.css';

	.membership-topline {
		@apply mt-8 flex flex-wrap items-center gap-4 transition-all duration-500;
		color: var(--theme-fg);

		& p {
			@apply text-lg;
		}
	}

	.membership-pill {
		@apply inline-flex items-center px-3 py-2 font-mono text-xs tracking-widest uppercase transition-all duration-500;
		border: 1px solid var(--theme-border);
		background-color: var(--theme-bg-elevated);
		color: var(--theme-fg);
	}

	.membership-stats {
		@apply mt-6 grid gap-4 md:grid-cols-3;

		& > div {
			@apply p-4 transition-all duration-500;
			border: 1px solid var(--theme-border);
			background-color: var(--theme-bg-elevated);
		}

		& p {
			@apply font-mono text-xs tracking-widest uppercase transition-colors duration-500;
			color: var(--theme-accent);
		}

		& strong {
			@apply mt-2 block font-display text-2xl tracking-tight transition-colors duration-500;
			color: var(--theme-fg);
		}
	}

	.membership-grid {
		@apply mt-10 grid gap-6 md:grid-cols-2;
	}

	.membership-panel {
		@apply grid gap-4 p-6 text-lg leading-relaxed transition-all duration-500;
		border: 1px solid var(--theme-border);
		background-color: var(--theme-bg-elevated);
		color: var(--theme-fg);
	}

	.membership-panel--rich {
		& h2 {
			@apply font-display text-3xl transition-colors duration-500;
			color: var(--theme-fg);
		}

		& ol {
			@apply grid gap-3;
		}

		& li {
			@apply flex gap-3 pb-3 transition-colors duration-500;
			border-bottom: 1px solid var(--theme-border);
		}

		& li span {
			@apply font-mono text-xs tracking-widest uppercase transition-colors duration-500;
			color: var(--theme-accent);
		}

		& li p {
			@apply text-lg transition-colors duration-500;
		}
	}

	.membership-benefits {
		@apply mt-2 grid gap-2;

		& p {
			@apply px-4 py-3 transition-all duration-500;
			border: 1px solid var(--theme-border);
			background-color: var(--theme-bg);
		}
	}

	.membership-panel--accent {
		background-color: var(--theme-bg);
		color: var(--theme-fg);

		& h2 {
			@apply font-display text-4xl leading-tight tracking-tight transition-colors duration-500;
		}

		& p {
			@apply text-xl leading-relaxed transition-colors duration-500;
		}
	}

	.membership-note {
		@apply border-l pl-3 text-lg transition-colors duration-500;
		border-color: var(--theme-accent);
		color: var(--theme-fg-muted);
	}

	.membership-actions {
		@apply mt-3 flex flex-wrap gap-3;
	}

	.membership-primary,
	.membership-secondary {
		@apply inline-flex w-fit items-center px-5 py-3 text-sm tracking-wider uppercase transition-colors duration-500;
	}

	.membership-primary {
		background-color: var(--theme-accent);
		color: var(--theme-accent-fg);

		&:hover {
			background-color: var(--theme-accent-alt);
			color: var(--theme-accent-alt-fg);
		}
	}

	.membership-secondary {
		border: 1px solid var(--theme-border);
		color: var(--theme-fg);

		&:hover {
			border-color: var(--theme-accent);
			color: var(--theme-accent);
		}
	}
</style>
