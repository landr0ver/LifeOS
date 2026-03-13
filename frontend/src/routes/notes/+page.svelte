<script lang="ts">
	import type { Note } from '$lib/types';

	let notes: Note[] = $state([]);
	let searchQuery = $state('');
	let view: 'list' | 'graph' = $state('list');
	let loading = $state(true);
</script>

<div class="notes-page">
	<div class="page-header">
		<h1>Notes</h1>
		<div class="header-actions">
			<div class="view-toggle">
				<button
					class="btn"
					class:btn-primary={view === 'list'}
					class:btn-ghost={view !== 'list'}
					onclick={() => view = 'list'}
				>List</button>
				<button
					class="btn"
					class:btn-primary={view === 'graph'}
					class:btn-ghost={view !== 'graph'}
					onclick={() => view = 'graph'}
				>Graph</button>
			</div>
			<button class="btn btn-primary">+ New Note</button>
		</div>
	</div>

	<div class="search-bar">
		<input class="input" type="search" placeholder="Search notes..." bind:value={searchQuery} />
	</div>

	{#if view === 'list'}
		{#if loading}
			<p class="loading">Loading notes...</p>
		{:else if notes.length === 0}
			<div class="empty-state card">
				<h2>No notes yet</h2>
				<p>Create your first note to start building your knowledge graph.</p>
				<button class="btn btn-primary">Create Note</button>
			</div>
		{:else}
			<div class="notes-grid">
				{#each notes as note}
					<div class="card note-card">
						<h3>{note.title}</h3>
						<p class="note-preview">{note.content.slice(0, 150)}...</p>
						{#if note.tags?.length}
							<div class="tags">
								{#each note.tags as tag}
									<span class="badge">{tag}</span>
								{/each}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		{/if}
	{:else}
		<div class="graph-container card">
			<p class="graph-placeholder">Knowledge graph visualization will render here (d3-force)</p>
		</div>
	{/if}
</div>

<style>
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: var(--space-4);
	}

	.header-actions {
		display: flex;
		gap: var(--space-3);
		align-items: center;
	}

	.view-toggle {
		display: flex;
		gap: var(--space-1);
	}

	.search-bar {
		margin-bottom: var(--space-6);
		max-width: 400px;
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

	.notes-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: var(--space-4);
	}

	.note-card {
		cursor: pointer;
		transition: transform var(--transition);
	}

	.note-card:hover {
		transform: translateY(-1px);
	}

	.note-card h3 {
		margin-bottom: var(--space-2);
		color: var(--color-primary);
	}

	.note-preview {
		color: var(--color-text-muted);
		font-size: var(--text-sm);
		margin-bottom: var(--space-3);
		line-height: 1.5;
	}

	.tags {
		display: flex;
		gap: var(--space-1);
		flex-wrap: wrap;
	}

	.graph-container {
		height: 500px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.graph-placeholder {
		color: var(--color-text-muted);
	}
</style>
