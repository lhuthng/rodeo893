<script>
	import { getRoute } from '$lib/navigation/index.js';
	import { t } from '$lib/localization';
	import PageShell from './PageShell.svelte';
</script>

<PageShell
	eyebrow={$t('loginPage.eyebrow')}
	title={$t('loginPage.title')}
	description={$t('loginPage.description')}
>
	<div class="login-panel">
		<p>{$t('loginPage.note')}</p>
		<a href={$getRoute('membership')}>{$t('cta.membership')}</a>
	</div>
</PageShell>

<style lang="postcss">
	@reference '../../../app.css';

	.login-panel {
		@apply mt-10 grid gap-4 border p-6 text-lg leading-relaxed;
		border-color: var(--theme-border);
		background-color: var(--theme-bg-elevated);
		color: var(--theme-fg-muted);

		& a {
			@apply inline-flex w-fit items-center px-5 py-3 text-sm tracking-wider uppercase transition-colors duration-500;
			background-color: var(--theme-accent);
			color: var(--theme-accent-fg);

			&:hover {
				background-color: var(--theme-accent-alt);
				color: var(--theme-accent-alt-fg);
			}
		}
	}
</style>
