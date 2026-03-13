<script lang="ts">
	import type { BoardWithColumns } from '$lib/types';

	let boards: BoardWithColumns[] = $state([]);
	let loading = $state(true);
</script>

<div class="kanban-page">
	<div class="page-header">
		<h1>Kanban Board</h1>
		<button class="btn btn-primary">+ New Board</button>
	</div>

	{#if loading}
		<p class="loading">Loading boards...</p>
	{:else if boards.length === 0}
		<div class="empty-state card">
			<h2>No boards yet</h2>
			<p>Create your first board to start organizing tasks.</p>
			<button class="btn btn-primary">Create Board</button>
		</div>
	{:else}
		<div class="board">
			{#each boards as board}
				<div class="columns-row">
					{#each board.columns as col}
						<div class="column">
							<h3 class="column-title">{col.name}</h3>
							<div class="cards-list">
								{#each col.cards as card}
									<div class="card task-card">
										<p class="card-title">{card.title}</p>
										{#if card.labels?.length}
											<div class="labels">
												{#each card.labels as label}
													<span class="badge">{label}</span>
												{/each}
											</div>
										{/if}
									</div>
								{/each}
							</div>
							<button class="btn btn-ghost add-card">+ Add Card</button>
						</div>
					{/each}
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.kanban-page {
		height: calc(100dvh - var(--space-12));
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: var(--space-6);
	}

	.loading {
		color: var(--color-text-muted);
	}

	.empty-state {
		text-align: center;
		padding: var(--space-10);
	}

	.empty-state h2 {
		margin-bottom: var(--space-2);
	}

	.empty-state p {
		color: var(--color-text-muted);
		margin-bottom: var(--space-4);
	}

	.columns-row {
		display: flex;
		gap: var(--space-4);
		overflow-x: auto;
		padding-bottom: var(--space-4);
	}

	.column {
		min-width: 280px;
		max-width: 320px;
		background: var(--color-surface);
		border-radius: var(--radius);
		padding: var(--space-3);
	}

	.column-title {
		font-size: var(--text-sm);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-muted);
		margin-bottom: var(--space-3);
	}

	.cards-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		min-height: var(--space-8);
	}

	.task-card {
		cursor: grab;
	}

	.card-title {
		font-weight: 500;
		margin-bottom: var(--space-2);
	}

	.labels {
		display: flex;
		gap: var(--space-1);
		flex-wrap: wrap;
	}

	.add-card {
		width: 100%;
		margin-top: var(--space-2);
		justify-content: center;
	}
</style>
