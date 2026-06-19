use std::collections::HashMap;
use std::path::Path;

use rusqlite::{params, Connection};

use crate::error::{Result, WcError};
use crate::models::{Command, Framework, PlaygroundSession, SimulateResult};

const SCHEMA_SQL: &str = r"
CREATE TABLE IF NOT EXISTS commands (
  id TEXT PRIMARY KEY,
  command TEXT NOT NULL,
  description TEXT,
  category TEXT,
  platform TEXT,
  danger_level INTEGER DEFAULT 0,
  source TEXT,
  updated_at TEXT
);

CREATE VIRTUAL TABLE IF NOT EXISTS commands_fts USING fts5(
  command, description, content='commands', content_rowid='rowid'
);

CREATE TABLE IF NOT EXISTS frameworks (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  icon TEXT
);

CREATE TABLE IF NOT EXISTS command_frameworks (
  command_id TEXT REFERENCES commands(id),
  framework_id TEXT REFERENCES frameworks(id),
  PRIMARY KEY (command_id, framework_id)
);

CREATE TABLE IF NOT EXISTS playground_sessions (
  id TEXT PRIMARY KEY,
  command TEXT NOT NULL,
  transcript TEXT,
  updated_at TEXT
);
";

pub struct CommandStore {
    conn: Connection,
}

impl CommandStore {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    pub fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(SCHEMA_SQL)?;
        Ok(())
    }

    pub fn seed_demo(&self) -> Result<()> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM commands", [], |r| r.get(0))?;
        if count > 0 {
            return Ok(());
        }

        let demos = [
            (
                "demo-ls",
                "ls -la",
                "List all files including hidden ones",
                "files",
                r#"["linux","osx"]"#,
                0,
                "tldr",
                "git",
                "Git",
                "Version control",
                "git-branch",
            ),
            (
                "demo-find",
                "find . -name '*.rs' -type f",
                "Find Rust source files recursively",
                "search",
                r#"["linux","osx"]"#,
                0,
                "tldr",
                "docker",
                "Docker",
                "Container runtime",
                "container",
            ),
            (
                "demo-rm",
                "rm -rf /",
                "Dangerous: remove root filesystem",
                "destructive",
                r#"["linux"]"#,
                3,
                "tldr",
                "bash",
                "Bash",
                "Shell scripting",
                "terminal",
            ),
        ];

        for (id, cmd, desc, cat, plat, danger, source, fw_id, fw_name, fw_desc, icon) in demos {
            self.conn.execute(
                "INSERT INTO commands (id, command, description, category, platform, danger_level, source, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'))",
                params![id, cmd, desc, cat, plat, danger, source],
            )?;
            self.conn.execute(
                "INSERT OR IGNORE INTO frameworks (id, name, description, icon) VALUES (?1, ?2, ?3, ?4)",
                params![fw_id, fw_name, fw_desc, icon],
            )?;
            self.conn.execute(
                "INSERT OR IGNORE INTO command_frameworks (command_id, framework_id) VALUES (?1, ?2)",
                params![id, fw_id],
            )?;
        }

        self.rebuild_fts()?;
        Ok(())
    }

    pub fn rebuild_fts(&self) -> Result<()> {
        self.conn.execute("DELETE FROM commands_fts", [])?;
        self.conn.execute(
            "INSERT INTO commands_fts(rowid, command, description)
             SELECT rowid, command, description FROM commands",
            [],
        )?;
        Ok(())
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<Command>> {
        let q = query.trim();
        if q.is_empty() {
            return self.list_recent(limit);
        }

        let pattern = format!("%{q}%");
        let mut stmt = self.conn.prepare(
            "SELECT id, command, description, category, platform, danger_level, source, updated_at
             FROM commands
             WHERE command LIKE ?1 OR description LIKE ?1 OR category LIKE ?1
             ORDER BY danger_level ASC, command ASC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![pattern, limit as i64], row_to_command)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(WcError::from)
    }

    fn list_recent(&self, limit: usize) -> Result<Vec<Command>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, command, description, category, platform, danger_level, source, updated_at
             FROM commands ORDER BY updated_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], row_to_command)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(WcError::from)
    }

    pub fn get_by_id(&self, id: &str) -> Result<Option<Command>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, command, description, category, platform, danger_level, source, updated_at
             FROM commands WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(row_to_command(row).map_err(WcError::from)?));
        }
        Ok(None)
    }

    pub fn categories(&self) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT DISTINCT category FROM commands ORDER BY category")?;
        let rows = stmt.query_map([], |r| r.get(0))?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(WcError::from)
    }

    pub fn list_frameworks(&self) -> Result<Vec<Framework>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description, icon FROM frameworks ORDER BY name")?;
        let rows = stmt.query_map([], |r| {
            Ok(Framework {
                id: r.get(0)?,
                name: r.get(1)?,
                description: r.get(2)?,
                icon: r.get(3)?,
            })
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(WcError::from)
    }

    pub fn commands_by_framework(&self, framework_id: &str, limit: usize) -> Result<Vec<Command>> {
        let mut stmt = self.conn.prepare(
            "SELECT c.id, c.command, c.description, c.category, c.platform, c.danger_level, c.source, c.updated_at
             FROM commands c
             INNER JOIN command_frameworks cf ON cf.command_id = c.id
             WHERE cf.framework_id = ?1
             ORDER BY c.command ASC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![framework_id, limit as i64], row_to_command)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(WcError::from)
    }

    pub fn related_commands(&self, command_id: &str, limit: usize) -> Result<Vec<Command>> {
        let Some(base) = self.get_by_id(command_id)? else {
            return Ok(vec![]);
        };
        let mut stmt = self.conn.prepare(
            "SELECT id, command, description, category, platform, danger_level, source, updated_at
             FROM commands
             WHERE category = ?1 AND id != ?2
             LIMIT ?3",
        )?;
        let rows = stmt.query_map(params![base.category, command_id, limit as i64], row_to_command)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(WcError::from)
    }

    pub fn validate_command(&self, cmd: &str) -> Result<(u8, String)> {
        let lower = cmd.to_lowercase();
        let danger = if lower.contains("rm -rf") || lower.contains("mkfs") || lower.contains(":(){") {
            3u8
        } else if lower.contains("rm ") || lower.contains("chmod") {
            2u8
        } else {
            0u8
        };
        let hint = match danger {
            3 => "Destructive command — simulation blocked".into(),
            2 => "Potentially destructive — review carefully".into(),
            _ => "Looks safe to simulate".into(),
        };
        Ok((danger, hint))
    }

    pub fn save_playground_session(&self, session: &PlaygroundSession) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO playground_sessions (id, command, transcript, updated_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                session.id,
                session.command,
                session.transcript,
                session.updated_at
            ],
        )?;
        Ok(())
    }

    pub fn list_playground_sessions(&self, limit: usize) -> Result<Vec<PlaygroundSession>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, command, transcript, updated_at FROM playground_sessions
             ORDER BY updated_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], |r| {
            Ok(PlaygroundSession {
                id: r.get(0)?,
                command: r.get(1)?,
                transcript: r.get(2)?,
                updated_at: r.get(3)?,
            })
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(WcError::from)
    }
}

fn row_to_command(row: &rusqlite::Row) -> rusqlite::Result<Command> {
    let platform_raw: String = row.get(4)?;
    let platform: Vec<String> = serde_json::from_str(&platform_raw).unwrap_or_else(|_| {
        platform_raw
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    });
    Ok(Command {
        id: row.get(0)?,
        command: row.get(1)?,
        description: row.get(2)?,
        category: row.get(3)?,
        platform,
        danger_level: row.get(5)?,
        source: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

pub fn simulate_with_store(store: &CommandStore, cmd: &str, vars: &HashMap<String, String>) -> SimulateResult {
    let mut resolved = cmd.to_string();
    for (k, v) in vars {
        resolved = resolved.replace(&format!("{{{{{k}}}}}"), v);
        resolved = resolved.replace(&format!("${k}"), v);
    }
    let (danger, _) = store.validate_command(&resolved).unwrap_or((0, String::new()));
    crate::simulator::simulate_command_inner(&resolved, danger)
}
