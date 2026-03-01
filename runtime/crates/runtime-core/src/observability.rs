use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metrics {
    pub request_count: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub provider_errors: u64,
    pub fallback_count: u64,
    pub validation_failures: u64,
}

impl Metrics {
    pub fn flush_to_file(&self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
        let path = path.into();
        let payload = json!({
            "updated_at_utc": Utc::now(),
            "metrics": self,
        });
        fs::write(path, serde_json::to_string_pretty(&payload)?)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StructuredLogger {
    path: PathBuf,
}

impl StructuredLogger {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn info(&self, event: &str, request_id: &str, extra: serde_json::Value) -> anyhow::Result<()> {
        self.write("info", event, request_id, extra)
    }

    pub fn error(&self, event: &str, request_id: &str, extra: serde_json::Value) -> anyhow::Result<()> {
        self.write("error", event, request_id, extra)
    }

    fn write(
        &self,
        level: &str,
        event: &str,
        request_id: &str,
        extra: serde_json::Value,
    ) -> anyhow::Result<()> {
        let line = serde_json::to_string(&json!({
            "ts": Utc::now(),
            "level": level,
            "event": event,
            "request_id": request_id,
            "extra": extra,
        }))?;
        let mut existing = if self.path.exists() {
            fs::read_to_string(&self.path)?
        } else {
            String::new()
        };
        existing.push_str(&line);
        existing.push('\n');
        fs::write(&self.path, existing)?;
        Ok(())
    }
}
