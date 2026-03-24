use anyhow::Result;
use std::fmt;

use super::store::StateStore;
use super::{Session, SessionMetrics, SessionState};
use crate::config::Config;
use crate::observability::{log_tool_call, ToolCallEvent, ToolLogEntry, ToolLogPage, ToolLogger};
use crate::worktree;

pub async fn create_session(
    db: &StateStore,
    cfg: &Config,
    task: &str,
    agent_type: &str,
    use_worktree: bool,
) -> Result<String> {
    let id = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let now = chrono::Utc::now();

    let wt = if use_worktree {
        Some(worktree::create_for_session(&id, cfg)?)
    } else {
        None
    };

    let session = Session {
        id: id.clone(),
        task: task.to_string(),
        agent_type: agent_type.to_string(),
        state: SessionState::Pending,
        worktree: wt,
        created_at: now,
        updated_at: now,
        metrics: SessionMetrics::default(),
    };

    db.insert_session(&session)?;
    Ok(id)
}

pub fn list_sessions(db: &StateStore) -> Result<Vec<Session>> {
    db.list_sessions()
}

pub fn get_status(db: &StateStore, id: &str) -> Result<SessionStatus> {
    let session = db
        .get_session(id)?
        .ok_or_else(|| anyhow::anyhow!("Session not found: {id}"))?;
    Ok(SessionStatus(session))
}

pub async fn stop_session(db: &StateStore, id: &str) -> Result<()> {
    db.update_state(id, &SessionState::Stopped)?;
    Ok(())
}

pub fn record_tool_call(
    db: &StateStore,
    session_id: &str,
    tool_name: &str,
    input_summary: &str,
    output_summary: &str,
    duration_ms: u64,
) -> Result<ToolLogEntry> {
    let session = db
        .get_session(session_id)?
        .ok_or_else(|| anyhow::anyhow!("Session not found: {session_id}"))?;

    let event = ToolCallEvent::new(
        session.id.clone(),
        tool_name,
        input_summary,
        output_summary,
        duration_ms,
    );
    let entry = log_tool_call(db, &event)?;
    db.increment_tool_calls(&session.id)?;

    Ok(entry)
}

pub fn query_tool_calls(
    db: &StateStore,
    session_id: &str,
    page: u64,
    page_size: u64,
) -> Result<ToolLogPage> {
    let session = db
        .get_session(session_id)?
        .ok_or_else(|| anyhow::anyhow!("Session not found: {session_id}"))?;

    ToolLogger::new(db).query(&session.id, page, page_size)
}

pub struct SessionStatus(Session);

impl fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = &self.0;
        writeln!(f, "Session: {}", s.id)?;
        writeln!(f, "Task:    {}", s.task)?;
        writeln!(f, "Agent:   {}", s.agent_type)?;
        writeln!(f, "State:   {}", s.state)?;
        if let Some(ref wt) = s.worktree {
            writeln!(f, "Branch:  {}", wt.branch)?;
            writeln!(f, "Worktree: {}", wt.path.display())?;
        }
        writeln!(f, "Tokens:  {}", s.metrics.tokens_used)?;
        writeln!(f, "Tools:   {}", s.metrics.tool_calls)?;
        writeln!(f, "Files:   {}", s.metrics.files_changed)?;
        writeln!(f, "Cost:    ${:.4}", s.metrics.cost_usd)?;
        writeln!(f, "Created: {}", s.created_at)?;
        write!(f, "Updated: {}", s.updated_at)
    }
}

#[cfg(test)]
mod tests {
    use super::{create_session, query_tool_calls, record_tool_call};
    use crate::config::Config;
    use crate::session::store::StateStore;

    #[tokio::test]
    async fn record_tool_call_updates_session_metrics() -> anyhow::Result<()> {
        let db_path =
            std::env::temp_dir().join(format!("ecc2-session-manager-{}.db", uuid::Uuid::new_v4()));
        let db = StateStore::open(&db_path)?;

        let cfg = Config {
            db_path: db_path.clone(),
            ..Config::default()
        };

        let session_id =
            create_session(&db, &cfg, "implement tool logging", "claude", false).await?;

        let entry = record_tool_call(&db, &session_id, "Bash", "git status", "clean worktree", 18)?;

        assert_eq!(entry.session_id, session_id);
        assert_eq!(entry.tool_name, "Bash");

        let session = db.get_session(&session_id)?.expect("session should exist");
        assert_eq!(session.metrics.tool_calls, 1);

        let page = query_tool_calls(&db, &session_id[..4], 1, 10)?;
        assert_eq!(page.total, 1);
        assert_eq!(page.entries[0].output_summary, "clean worktree");

        std::fs::remove_file(&db_path).ok();

        Ok(())
    }
}
