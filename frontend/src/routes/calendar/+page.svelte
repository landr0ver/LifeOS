<script lang="ts">
	let currentMonth = $state(new Date());
	let view: 'month' | 'week' = $state('month');

	function getDaysInMonth(date: Date): Date[] {
		const year = date.getFullYear();
		const month = date.getMonth();
		const firstDay = new Date(year, month, 1);
		const lastDay = new Date(year, month + 1, 0);

		// Start from Monday of the first week
		const startDate = new Date(firstDay);
		const dayOfWeek = startDate.getDay() || 7;
		startDate.setDate(startDate.getDate() - dayOfWeek + 1);

		const days: Date[] = [];
		const current = new Date(startDate);
		while (current <= lastDay || days.length % 7 !== 0) {
			days.push(new Date(current));
			current.setDate(current.getDate() + 1);
		}
		return days;
	}

	function formatMonth(date: Date): string {
		return date.toLocaleDateString('en-US', { month: 'long', year: 'numeric' });
	}

	function prevMonth() {
		const d = new Date(currentMonth);
		d.setMonth(d.getMonth() - 1);
		currentMonth = d;
	}

	function nextMonth() {
		const d = new Date(currentMonth);
		d.setMonth(d.getMonth() + 1);
		currentMonth = d;
	}

	const weekDays = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
</script>

<div class="calendar-page">
	<div class="page-header">
		<h1>Calendar</h1>
		<div class="header-actions">
			<div class="view-toggle">
				<button
					class="btn"
					class:btn-primary={view === 'month'}
					class:btn-ghost={view !== 'month'}
					onclick={() => view = 'month'}
				>Month</button>
				<button
					class="btn"
					class:btn-primary={view === 'week'}
					class:btn-ghost={view !== 'week'}
					onclick={() => view = 'week'}
				>Week</button>
			</div>
			<button class="btn btn-primary">+ New Event</button>
		</div>
	</div>

	<div class="month-nav">
		<button class="btn btn-ghost" onclick={prevMonth}>&larr;</button>
		<h2>{formatMonth(currentMonth)}</h2>
		<button class="btn btn-ghost" onclick={nextMonth}>&rarr;</button>
	</div>

	<div class="calendar-grid">
		{#each weekDays as day}
			<div class="day-header">{day}</div>
		{/each}
		{#each getDaysInMonth(currentMonth) as day}
			<div
				class="day-cell"
				class:other-month={day.getMonth() !== currentMonth.getMonth()}
				class:today={day.toDateString() === new Date().toDateString()}
			>
				<span class="day-number">{day.getDate()}</span>
			</div>
		{/each}
	</div>
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

	.month-nav {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-4);
		margin-bottom: var(--space-4);
	}

	.calendar-grid {
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		gap: 1px;
		background: var(--color-border);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		overflow: hidden;
	}

	.day-header {
		padding: var(--space-2);
		text-align: center;
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text-muted);
		background: var(--color-surface);
	}

	.day-cell {
		min-height: 80px;
		padding: var(--space-2);
		background: var(--color-bg-card);
		cursor: pointer;
		transition: background-color var(--transition);
	}

	.day-cell:hover {
		background: var(--color-surface);
	}

	.day-cell.other-month {
		opacity: 0.4;
	}

	.day-cell.today {
		background: rgba(199, 91, 57, 0.08);
	}

	.day-cell.today .day-number {
		background: var(--color-primary);
		color: white;
		border-radius: 50%;
		width: 24px;
		height: 24px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		font-size: var(--text-sm);
	}

	.day-number {
		font-size: var(--text-sm);
		font-weight: 500;
	}
</style>
