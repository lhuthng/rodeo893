<script>
	import { getRoute } from '$lib/navigation/index.js';
	import { t } from '$lib/localization';

	let {
		searchResults,
		totalSearchMatches,
		debouncedSearchTerm,
		searchQueryLink,
		searchTerm = $bindable(''),
		inputEl = $bindable(null),
		onSubmit,
		onClose
	} = $props();
</script>

<form class="search-panel__form" onsubmit={onSubmit}>
	<input
		bind:this={inputEl}
		bind:value={searchTerm}
		type="search"
		placeholder={$t('search.placeholder')}
		aria-label={$t('search.placeholder')}
	/>
	<button type="submit">{$t('search.submit')}</button>
</form>

{#if debouncedSearchTerm.trim()}
	<div class="search-panel__results">
		{#if searchResults.length}
			<ol>
				{#each searchResults as product}
					<li>
						<a href={product.path} onclick={onClose}>
							<strong>{product.name}</strong>
							<span>{product.description}</span>
						</a>
					</li>
				{/each}
			</ol>
			{#if totalSearchMatches > 4}
				<a class="search-panel__more" href={searchQueryLink} onclick={onClose}>
					{$t('search.viewMore')}
				</a>
			{/if}
		{:else}
			<p class="search-panel__empty">{$t('search.empty')}</p>
		{/if}
	</div>
{/if}

<style lang="postcss">
	@reference "../../../app.css";

	.search-panel__form {
		@apply mx-auto flex max-w-300 flex-col gap-3 md:flex-row;

		& input {
			@apply w-full border px-4 py-3 font-mono text-sm tracking-wider uppercase;
			border-color: var(--theme-border);
			background-color: var(--theme-bg);
			color: var(--theme-fg);
		}

		& button {
			@apply inline-flex cursor-pointer items-center justify-center px-5 py-3 font-mono text-sm tracking-wider uppercase transition-colors duration-500;
			background-color: var(--theme-accent);
			color: var(--theme-accent-fg);

			&:hover {
				background-color: var(--theme-accent-alt);
				color: var(--theme-accent-alt-fg);
			}
		}
	}

	.search-panel__results {
		@apply mt-4 grid gap-2;

		& ol {
			@apply grid gap-2;
		}

		& a {
			@apply grid gap-1 border px-4 py-3 transition-colors duration-300;
			border-color: var(--theme-border);
			color: var(--theme-fg);

			&:hover {
				border-color: var(--theme-accent);
			}

			& strong {
				@apply font-display text-xl tracking-tight;
			}

			& span {
				@apply text-sm leading-relaxed;
				color: var(--theme-fg-muted);
			}
		}
	}

	.search-panel__more {
		@apply inline-flex w-fit items-center px-4 py-2 font-mono text-xs tracking-widest uppercase transition-colors duration-300;
		background-color: var(--theme-accent);
		color: var(--theme-accent-fg);

		&:hover {
			background-color: var(--theme-accent-alt);
			color: var(--theme-accent-alt-fg);
		}
	}

	.search-panel__empty {
		@apply border px-4 py-3 text-sm;
		border-color: var(--theme-border);
		color: var(--theme-fg-muted);
	}
</style>
