use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JournalError {
    #[error("could not determine a data directory for this system")]
    MissingDataDir,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub mood: u8,
    pub tags: Vec<String>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JournalData {
    pub next_id: u64,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone)]
pub struct MoodStats {
    pub count: usize,
    pub average: f32,
    pub best_mood: Option<u8>,
    pub worst_mood: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct Journal {
    path: PathBuf,
    data: JournalData,
}

impl Journal {
    pub fn load_default() -> Result<Self, JournalError> {
        let base = dirs::data_local_dir().ok_or(JournalError::MissingDataDir)?;
        let dir = base.join("mood-journal-cli");
        fs::create_dir_all(&dir)?;
        let path = dir.join("journal.json");
        Self::load_from(path)
    }

    pub fn load_from(path: impl Into<PathBuf>) -> Result<Self, JournalError> {
        let path = path.into();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let data = if path.exists() {
            let raw = fs::read_to_string(&path)?;
            if raw.trim().is_empty() {
                JournalData {
                    next_id: 1,
                    entries: Vec::new(),
                }
            } else {
                serde_json::from_str(&raw)?
            }
        } else {
            JournalData {
                next_id: 1,
                entries: Vec::new(),
            }
        };

        Ok(Self { path, data })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn entries(&self) -> &[Entry] {
        &self.data.entries
    }

    pub fn add_entry(
        &mut self,
        mood: u8,
        tags: Vec<String>,
        text: String,
    ) -> Result<&Entry, JournalError> {
        let entry = Entry {
            id: self.data.next_id,
            created_at: Local::now(),
            mood,
            tags,
            text,
        };

        self.data.next_id += 1;
        self.data.entries.push(entry);
        self.save()?;

        Ok(self.data.entries.last().expect("entry was just inserted"))
    }

    pub fn list_entries(&self, tag: Option<&str>, limit: Option<usize>) -> Vec<&Entry> {
        let mut items: Vec<&Entry> = self
            .data
            .entries
            .iter()
            .rev()
            .filter(|entry| match tag {
                Some(tag_value) => entry.tags.iter().any(|t| t == tag_value),
                None => true,
            })
            .collect();

        if let Some(max) = limit {
            items.truncate(max);
        }

        items
    }

    pub fn stats(&self) -> MoodStats {
        let count = self.data.entries.len();
        if count == 0 {
            return MoodStats {
                count: 0,
                average: 0.0,
                best_mood: None,
                worst_mood: None,
            };
        }

        let sum: u32 = self.data.entries.iter().map(|e| e.mood as u32).sum();
        let best = self.data.entries.iter().map(|e| e.mood).max();
        let worst = self.data.entries.iter().map(|e| e.mood).min();

        MoodStats {
            count,
            average: sum as f32 / count as f32,
            best_mood: best,
            worst_mood: worst,
        }
    }

    pub fn save(&self) -> Result<(), JournalError> {
        let raw = serde_json::to_string_pretty(&self.data)?;
        fs::write(&self.path, raw)?;
        Ok(())
    }
}

pub fn parse_tags(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
        .map(|tag| tag.to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn parses_tags_cleanly() {
        let tags = parse_tags("Work,  Rust , , CLI ");
        assert_eq!(tags, vec!["work", "rust", "cli"]);
    }

    #[test]
    fn saves_and_loads_entries() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("journal.json");

        let mut journal = Journal::load_from(&path).unwrap();
        journal
            .add_entry(4, vec!["test".into()], "hello world".into())
            .unwrap();

        let loaded = Journal::load_from(&path).unwrap();
        assert_eq!(loaded.entries().len(), 1);
        assert_eq!(loaded.entries()[0].mood, 4);
        assert_eq!(loaded.entries()[0].tags, vec!["test"]);
    }
}
