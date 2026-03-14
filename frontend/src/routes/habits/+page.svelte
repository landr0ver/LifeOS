<script lang="ts">
	import type { Habit } from '$lib/types';

	let habits: Habit[] = $state([]);
	let loading = $state(true);

	const daysOfWeek = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
</script>

<div class="habits-page">
	<div class="page-header">
		<h1>Habit Tracker</h1>
		<button class="btn btn-primary">+ New Habit</button>
	</div>

	{#if loading}
		<p class="loading">Loading habits...</p>
	{:else if habits.length === 0}
		<div class="empty-state card">
			<h2>No habits yet</h2>
			<p>Start building good habits by tracking them daily.</p>
			<button class="btn btn-primary">Add Habit</button>
		</div>
	{:else}
		<div class="habits-grid">
			{#each habits as habit}
				<div class="card habit-card">
					<div class="habit-header">
						<h3>{habit.name}</h3>
						<span class="badge">{habit.frequency}</span>
					</div>
					{#if habit.description}
						<p class="habit-desc">{habit.description}</p>
					{/if}
					<div class="day-grid">
						{#each daysOfWeek as day}
							<div class="day-cell">
								<span class="day-label">{day}</span>
								<button class="day-toggle"></button>
							</div>
						{/each}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
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

	.habits-grid {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		max-width: 600px;
	}

	.habit-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: var(--space-2);
	}

	.habit-desc {
		color: var(--color-text-muted);
		font-size: var(--text-sm);
		margin-bottom: var(--space-3);
	}

	.day-grid {
		display: flex;
		gap: var(--space-2);
	}

	.day-cell {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-1);
	}

	.day-label {
		font-size: var(--text-xs);
		color: var(--color-text-muted);
	}

	.day-toggle {
		width: 32px;
		height: 32px;
		border-radius: var(--radius-sm);
		border: 2px solid var(--color-border);
		background: var(--color-bg-card);
		cursor: pointer;
		transition: all var(--transition);
	}

	.day-toggle:hover {
		border-color: var(--color-accent);
		background: rgba(91, 140, 90, 0.1);
	}
</style>
