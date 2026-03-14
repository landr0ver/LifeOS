// Kanban
export interface Board {
  id: string;
  name: string;
  created_at: string;
  updated_at: string;
}

export interface Column {
  id: string;
  board_id: string;
  name: string;
  position: number;
  created_at: string;
}

export interface Card {
  id: string;
  column_id: string;
  title: string;
  description: string | null;
  position: number;
  due_date: string | null;
  labels: string[] | null;
  github_issue_id: number | null;
  github_pr_id: number | null;
  created_at: string;
  updated_at: string;
}

export interface BoardWithColumns extends Board {
  columns: ColumnWithCards[];
}

export interface ColumnWithCards extends Column {
  cards: Card[];
}

// Habits
export interface Habit {
  id: string;
  name: string;
  description: string | null;
  frequency: 'daily' | 'weekly' | 'custom';
  target_days: number[] | null;
  color: string | null;
  icon: string | null;
  created_at: string;
  archived_at: string | null;
}

export interface HabitEntry {
  id: string;
  habit_id: string;
  date: string;
  completed: boolean;
  note: string | null;
}

// Notes
export interface Note {
  id: string;
  title: string;
  content: string;
  tags: string[] | null;
  created_at: string;
  updated_at: string;
}

export interface NoteLink {
  id: string;
  source_id: string;
  target_id: string;
  label: string | null;
}

export interface GraphData {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

export interface GraphNode {
  id: string;
  title: string;
  tags: string[] | null;
}

export interface GraphEdge {
  id: string;
  source: string;
  target: string;
  label: string | null;
}

// Calendar
export interface CalendarEvent {
  id: string;
  title: string;
  description: string | null;
  start_time: string;
  end_time: string | null;
  all_day: boolean;
  recurrence: string | null;
  color: string | null;
  card_id: string | null;
  created_at: string;
}

// GitHub
export interface GithubRepo {
  id: string;
  owner: string;
  name: string;
  last_synced_at: string | null;
}

export interface GithubIssue {
  id: string;
  repo_id: string;
  issue_number: number;
  title: string;
  state: string;
  labels: string[] | null;
  body: string | null;
  card_id: string | null;
  synced_at: string;
}

export interface GithubPullRequest {
  id: string;
  repo_id: string;
  pr_number: number;
  title: string;
  state: string;
  linked_issue_number: number | null;
  card_id: string | null;
  synced_at: string;
}
