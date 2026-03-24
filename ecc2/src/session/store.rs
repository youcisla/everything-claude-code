use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;

use super::{Session, SessionMetrics, SessionState};
use crate::observability::{ToolLogEntry, ToolLogPage};

pub struct StateStore {
    conn: Connection,
}

impl StateStore {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        let store = Self { conn };
        store.init_schema()?;
        Ok(store)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                task TEXT NOT NULL,
                agent_type TEXT NOT NULL,
                state TEXT NOT NULL DEFAULT 'pending',
                worktree_path TEXT,
                worktree_branch TEXT,
                worktree_base TEXT,
                tokens_used INTEGER DEFAULT 0,
                tool_calls INTEGER DEFAULT 0,
                files_changed INTEGER DEFAULT 0,
                duration_secs INTEGER DEFAULT 0,
                cost_usd REAL DEFAULT 0.0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tool_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL REFERENCES sessions(id),
                tool_name TEXT NOT NULL,
                input_summary TEXT,
                output_summary TEXT,
                duration_ms INTEGER,
                risk_score REAL DEFAULT 0.0,
                timestamp TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                from_session TEXT NOT NULL,
                to_session TEXT NOT NULL,
                content TEXT NOT NULL,
                msg_type TEXT NOT NULL DEFAULT 'info',
                read INTEGER DEFAULT 0,
                timestamp TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_sessions_state ON sessions(state);
            CREATE INDEX IF NOT EXISTS idx_tool_log_session ON tool_log(session_id);
            CREATE INDEX IF NOT EXISTS idx_messages_to ON messages(to_session, read);
            ",
        )?;
        Ok(())
    }

    pub fn insert_session(&self, session: &Session) -> Result<()> {
        self.conn.execute(
            "INSERT INTO sessions (id, task, agent_type, state, worktree_path, worktree_branch, worktree_base, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                session.id,
                session.task,
                session.agent_type,
                session.state.to_string(),
                session.worktree.as_ref().map(|w| w.path.to_string_lossy().to_string()),
                session.worktree.as_ref().map(|w| w.branch.clone()),
                session.worktree.as_ref().map(|w| w.base_branch.clone()),
                session.created_at.to_rfc3339(),
                session.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn update_state(&self, session_id: &str, state: &SessionState) -> Result<()> {
        self.conn.execute(
            "UPDATE sessions SET state = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![
                state.to_string(),
                chrono::Utc::now().to_rfc3339(),
                session_id,
            ],
        )?;
        Ok(())
    }

    pub fn update_metrics(&self, session_id: &str, metrics: &SessionMetrics) -> Result<()> {
        self.conn.execute(
            "UPDATE sessions SET tokens_used = ?1, tool_calls = ?2, files_changed = ?3, duration_secs = ?4, cost_usd = ?5, updated_at = ?6 WHERE id = ?7",
            rusqlite::params![
                metrics.tokens_used,
                metrics.tool_calls,
                metrics.files_changed,
                metrics.duration_secs,
                metrics.cost_usd,
                chrono::Utc::now().to_rfc3339(),
                session_id,
            ],
        )?;
        Ok(())
    }

    pub fn increment_tool_calls(&self, session_id: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE sessions SET tool_calls = tool_calls + 1, updated_at = ?1 WHERE id = ?2",
            rusqlite::params![chrono::Utc::now().to_rfc3339(), session_id],
        )?;
        Ok(())
    }

    pub fn list_sessions(&self) -> Result<Vec<Session>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, task, agent_type, state, worktree_path, worktree_branch, worktree_base,
                    tokens_used, tool_calls, files_changed, duration_secs, cost_usd,
                    created_at, updated_at
             FROM sessions ORDER BY updated_at DESC",
        )?;

        let sessions = stmt
            .query_map([], |row| {
                let state_str: String = row.get(3)?;
                let state = match state_str.as_str() {
                    "running" => SessionState::Running,
                    "idle" => SessionState::Idle,
                    "completed" => SessionState::Completed,
                    "failed" => SessionState::Failed,
                    "stopped" => SessionState::Stopped,
                    _ => SessionState::Pending,
                };

                let worktree_path: Option<String> = row.get(4)?;
                let worktree = worktree_path.map(|p| super::WorktreeInfo {
                    path: std::path::PathBuf::from(p),
                    branch: row.get::<_, String>(5).unwrap_or_default(),
                    base_branch: row.get::<_, String>(6).unwrap_or_default(),
                });

                let created_str: String = row.get(12)?;
                let updated_str: String = row.get(13)?;

                Ok(Session {
                    id: row.get(0)?,
                    task: row.get(1)?,
                    agent_type: row.get(2)?,
                    state,
                    worktree,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                        .unwrap_or_default()
                        .with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
                        .unwrap_or_default()
                        .with_timezone(&chrono::Utc),
                    metrics: SessionMetrics {
                        tokens_used: row.get(7)?,
                        tool_calls: row.get(8)?,
                        files_changed: row.get(9)?,
                        duration_secs: row.get(10)?,
                        cost_usd: row.get(11)?,
                    },
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(sessions)
    }

    pub fn get_session(&self, id: &str) -> Result<Option<Session>> {
        let sessions = self.list_sessions()?;
        Ok(sessions
            .into_iter()
            .find(|s| s.id == id || s.id.starts_with(id)))
    }

    pub fn send_message(&self, from: &str, to: &str, content: &str, msg_type: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO messages (from_session, to_session, content, msg_type, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![from, to, content, msg_type, chrono::Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn insert_tool_log(
        &self,
        session_id: &str,
        tool_name: &str,
        input_summary: &str,
        output_summary: &str,
        duration_ms: u64,
        risk_score: f64,
        timestamp: &str,
    ) -> Result<ToolLogEntry> {
        self.conn.execute(
            "INSERT INTO tool_log (session_id, tool_name, input_summary, output_summary, duration_ms, risk_score, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                session_id,
                tool_name,
                input_summary,
                output_summary,
                duration_ms,
                risk_score,
                timestamp,
            ],
        )?;

        Ok(ToolLogEntry {
            id: self.conn.last_insert_rowid(),
            session_id: session_id.to_string(),
            tool_name: tool_name.to_string(),
            input_summary: input_summary.to_string(),
            output_summary: output_summary.to_string(),
            duration_ms,
            risk_score,
            timestamp: timestamp.to_string(),
        })
    }

    pub fn query_tool_logs(
        &self,
        session_id: &str,
        page: u64,
        page_size: u64,
    ) -> Result<ToolLogPage> {
        let page = page.max(1);
        let offset = (page - 1) * page_size;

        let total: u64 = self.conn.query_row(
            "SELECT COUNT(*) FROM tool_log WHERE session_id = ?1",
            rusqlite::params![session_id],
            |row| row.get(0),
        )?;

        let mut stmt = self.conn.prepare(
            "SELECT id, session_id, tool_name, input_summary, output_summary, duration_ms, risk_score, timestamp
             FROM tool_log
             WHERE session_id = ?1
             ORDER BY timestamp DESC, id DESC
             LIMIT ?2 OFFSET ?3",
        )?;

        let entries = stmt
            .query_map(rusqlite::params![session_id, page_size, offset], |row| {
                Ok(ToolLogEntry {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    tool_name: row.get(2)?,
                    input_summary: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                    output_summary: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
                    duration_ms: row.get::<_, Option<u64>>(5)?.unwrap_or_default(),
                    risk_score: row.get::<_, Option<f64>>(6)?.unwrap_or_default(),
                    timestamp: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ToolLogPage {
            entries,
            page,
            page_size,
            total,
        })
    }
}
