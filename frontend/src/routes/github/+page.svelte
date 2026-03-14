<script lang="ts">
	import type { GithubRepo, GithubIssue, GithubPullRequest } from '$lib/types';

	let repos: GithubRepo[] = $state([]);
	let issues: GithubIssue[] = $state([]);
	let prs: GithubPullRequest[] = $state([]);
	let loading = $state(true);
	let activeTab: 'issues' | 'prs' = $state('issues');
</script>

<div class="github-page">
	<div class="page-header">
		<h1>GitHub</h1>
		<div class="header-actions">
			<button class="btn btn-secondary">Sync Features</button>
			<button class="btn btn-primary">+ Connect Repo</button>
		</div>
	</div>

	{#if repos.length > 0}
		<div class="repos">
			{#each repos as repo}
				<span class="badge">{repo.owner}/{repo.name}</span>
			{/each}
		</div>
	{/if}

	<div class="tabs">
		<button
			class="tab"
			class:active={activeTab === 'issues'}
			onclick={() => activeTab = 'issues'}
		>Issues</button>
		<button
			class="tab"
			class:active={activeTab === 'prs'}
			onclick={() => activeTab = 'prs'}
		>Pull Requests</button>
	</div>

	{#if loading}
		<p class="loading">Loading...</p>
	{:else if activeTab === 'issues'}
		{#if issues.length === 0}
			<div class="empty-state card">
				<h2>No issues synced</h2>
				<p>Connect a GitHub repo and sync to see issues here.</p>
			</div>
		{:else}
			<div class="items-list">
				{#each issues as issue}
					<div class="card item-card">
						<div class="item-header">
							<span class="issue-number">#{issue.issue_number}</span>
							<h3>{issue.title}</h3>
						</div>
						<div class="item-meta">
							<span class="badge" class:open={issue.state === 'open'} class:closed={issue.state === 'closed'}>
								{issue.state}
							</span>
							{#if issue.labels?.length}
								{#each issue.labels as label}
									<span class="badge">{label}</span>
								{/each}
							{/if}
						</div>
					</div>
				{/each}
			</div>
		{/if}
	{:else}
		{#if prs.length === 0}
			<div class="empty-state card">
				<h2>No pull requests synced</h2>
				<p>Connect a GitHub repo and sync to see PRs here.</p>
			</div>
		{:else}
			<div class="items-list">
				{#each prs as pr}
					<div class="card item-card">
						<div class="item-header">
							<span class="issue-number">#{pr.pr_number}</span>
							<h3>{pr.title}</h3>
						</div>
						<span class="badge">{pr.state}</span>
					</div>
				{/each}
			</div>
		{/if}
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
		gap: var(--space-2);
	}

	.repos {
		display: flex;
		gap: var(--space-2);
		margin-bottom: var(--space-4);
	}

	.tabs {
		display: flex;
		gap: var(--space-1);
		margin-bottom: var(--space-4);
		border-bottom: 1px solid var(--color-border);
		padding-bottom: var(--space-1);
	}

	.tab {
		padding: var(--space-2) var(--space-4);
		border: none;
		background: none;
		font-family: var(--font-body);
		font-size: var(--text-sm);
		font-weight: 500;
		color: var(--color-text-muted);
		cursor: pointer;
		border-bottom: 2px solid transparent;
		transition: all var(--transition);
	}

	.tab.active {
		color: var(--color-primary);
		border-bottom-color: var(--color-primary);
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
	}

	.items-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.item-header {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		margin-bottom: var(--space-2);
	}

	.issue-number {
		color: var(--color-text-muted);
		font-size: var(--text-sm);
	}

	.item-meta {
		display: flex;
		gap: var(--space-2);
	}

	.badge.open {
		background: rgba(91, 140, 90, 0.15);
		color: var(--color-accent);
	}

	.badge.closed {
		background: rgba(107, 123, 141, 0.15);
		color: var(--color-accent-2);
	}
</style>
