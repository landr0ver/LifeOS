<script lang="ts">
	import '$lib/theme/variables.css';

	let { children } = $props();

	const navItems = [
		{ href: '/', label: 'Dashboard', icon: '⌂' },
		{ href: '/kanban', label: 'Kanban', icon: '▦' },
		{ href: '/habits', label: 'Habits', icon: '✓' },
		{ href: '/notes', label: 'Notes', icon: '✎' },
		{ href: '/calendar', label: 'Calendar', icon: '▣' },
		{ href: '/github', label: 'GitHub', icon: '⑂' },
	];
</script>

<svelte:head>
	<title>LifeOS</title>
</svelte:head>

<div class="app-layout">
	<nav class="sidebar">
		<div class="sidebar-header">
			<h1 class="logo">LifeOS</h1>
		</div>
		<ul class="nav-list">
			{#each navItems as item}
				<li>
					<a href={item.href} class="nav-link">
						<span class="nav-icon">{item.icon}</span>
						<span class="nav-label">{item.label}</span>
					</a>
				</li>
			{/each}
		</ul>
	</nav>
	<main class="main-content">
		{@render children()}
	</main>
</div>

<style>
	.app-layout {
		display: flex;
		min-height: 100dvh;
	}

	.sidebar {
		width: var(--nav-width);
		background: var(--color-nav-bg);
		color: var(--color-nav-text);
		display: flex;
		flex-direction: column;
		padding: var(--space-4);
		position: fixed;
		top: 0;
		left: 0;
		bottom: 0;
		z-index: 100;
	}

	.sidebar-header {
		padding: var(--space-4) var(--space-2);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		margin-bottom: var(--space-4);
	}

	.logo {
		font-family: var(--font-heading);
		font-size: var(--text-2xl);
		color: var(--color-secondary);
		letter-spacing: 0.05em;
	}

	.nav-list {
		list-style: none;
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.nav-link {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-2) var(--space-3);
		border-radius: var(--radius);
		color: var(--color-nav-text);
		font-size: var(--text-sm);
		transition: background-color var(--transition);
	}

	.nav-link:hover {
		background: rgba(255, 255, 255, 0.1);
		color: var(--color-secondary);
	}

	.nav-icon {
		font-size: var(--text-lg);
		width: 1.5rem;
		text-align: center;
	}

	.main-content {
		flex: 1;
		margin-left: var(--nav-width);
		padding: var(--space-6);
		max-width: 100%;
		overflow-x: hidden;
	}

	@media (max-width: 768px) {
		.sidebar {
			position: fixed;
			bottom: 0;
			top: auto;
			left: 0;
			right: 0;
			width: 100%;
			flex-direction: row;
			padding: var(--space-2);
			z-index: 100;
		}

		.sidebar-header {
			display: none;
		}

		.nav-list {
			flex-direction: row;
			justify-content: space-around;
			width: 100%;
		}

		.nav-label {
			display: none;
		}

		.nav-icon {
			font-size: var(--text-xl);
		}

		.main-content {
			margin-left: 0;
			margin-bottom: 56px;
			padding: var(--space-4);
		}
	}
</style>
