<script lang="ts">
	// Dashboard — aggregated view of all modules
</script>

<div class="dashboard">
	<h1>Welcome to LifeOS</h1>
	<p class="subtitle">Your personal organizer & planning assistant</p>

	<div class="grid">
		<a href="/kanban" class="dashboard-card">
			<span class="card-icon">▦</span>
			<h2>Kanban</h2>
			<p>Manage tasks and projects</p>
		</a>

		<a href="/habits" class="dashboard-card">
			<span class="card-icon">✓</span>
			<h2>Habits</h2>
			<p>Track daily routines</p>
		</a>

		<a href="/notes" class="dashboard-card">
			<span class="card-icon">✎</span>
			<h2>Notes</h2>
			<p>Knowledge graph & notes</p>
		</a>

		<a href="/calendar" class="dashboard-card">
			<span class="card-icon">▣</span>
			<h2>Calendar</h2>
			<p>Schedule & events</p>
		</a>

		<a href="/github" class="dashboard-card">
			<span class="card-icon">⑂</span>
			<h2>GitHub</h2>
			<p>Issues, PRs & project tracking</p>
		</a>
	</div>
</div>

<style>
	.dashboard {
		max-width: 900px;
	}

	.subtitle {
		color: var(--color-text-muted);
		margin-top: var(--space-2);
		margin-bottom: var(--space-8);
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
		gap: var(--space-4);
	}

	.dashboard-card {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		padding: var(--space-6);
		background: var(--color-bg-card);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-lg);
		box-shadow: var(--shadow);
		transition: transform var(--transition), box-shadow var(--transition);
		color: var(--color-text);
	}

	.dashboard-card:hover {
		transform: translateY(-2px);
		box-shadow: var(--shadow-lg);
		color: var(--color-text);
	}

	.card-icon {
		font-size: 2rem;
	}

	.dashboard-card h2 {
		font-size: var(--text-xl);
		color: var(--color-primary);
	}

	.dashboard-card p {
		color: var(--color-text-muted);
		font-size: var(--text-sm);
	}
</style>
