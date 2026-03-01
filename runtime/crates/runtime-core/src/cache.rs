use crate::contracts::{BudgetsMeta, ProviderMeta, RecapPayload, RecapRequestV1};
use crate::traits::CacheStore;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedRecap {
    pub recap: RecapPayload,
    pub provider: ProviderMeta,
    pub budgets: BudgetsMeta,
    pub created_at_utc: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub created_at_utc: DateTime<Utc>,
    pub value: CachedRecap,
}

#[derive(Debug, Clone)]
pub struct FileCacheStore {
    directory: PathBuf,
    ttl_seconds: u64,
}

impl FileCacheStore {
    pub fn new(directory: impl Into<PathBuf>, ttl_seconds: u64) -> anyhow::Result<Self> {
        let dir = directory.into();
        fs::create_dir_all(&dir)?;
        Ok(Self {
            directory: dir,
            ttl_seconds,
        })
    }

    fn entry_path(&self, key: &str) -> PathBuf {
        self.directory.join(format!("{key}.json"))
    }

    fn read_entry(&self, key: &str) -> anyhow::Result<Option<CacheEntry>> {
        let path = self.entry_path(key);
        if !path.exists() {
            return Ok(None);
        }

        let body = fs::read_to_string(path)?;
        let entry = serde_json::from_str::<CacheEntry>(&body)?;
        Ok(Some(entry))
    }
}

impl CacheStore for FileCacheStore {
    fn stable_key_for(&self, request: &RecapRequestV1, prompt_version: &str, model: &str) -> String {
        let stable_payload = serde_json::json!({
            "contract_version": &request.contract_version,
            "feature": &request.feature,
            "spoiler_mode": &request.spoiler_mode,
            "game_context": &request.game_context,
            "client_kind": &request.client.client_kind,
            "client_profile": &request.client.profile,
            "prompt_version": prompt_version,
            "model": model
        });

        let serialized = serde_json::to_vec(&stable_payload).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        let digest = hasher.finalize();
        hex::encode(digest)
    }

    fn get_fresh(&mut self, key: &str) -> anyhow::Result<Option<(CachedRecap, u64)>> {
        let Some(entry) = self.read_entry(key)? else {
            return Ok(None);
        };

        let age = (Utc::now() - entry.created_at_utc)
            .num_seconds()
            .max(0) as u64;
        if age > self.ttl_seconds {
            return Ok(None);
        }

        Ok(Some((entry.value, age)))
    }

    fn get_stale(&mut self, key: &str) -> anyhow::Result<Option<CachedRecap>> {
        let Some(entry) = self.read_entry(key)? else {
            return Ok(None);
        };
        Ok(Some(entry.value))
    }

    fn set(&mut self, key: &str, value: &CachedRecap) -> anyhow::Result<()> {
        let entry = CacheEntry {
            key: key.to_string(),
            created_at_utc: Utc::now(),
            value: value.clone(),
        };

        let path = self.entry_path(key);
        let body = serde_json::to_string_pretty(&entry)?;
        fs::write(path, body)?;
        Ok(())
    }
}
