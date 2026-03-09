use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeMode {
    Player,
    Developer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiConfig {
    pub model: String,
    pub timeout_ms: u64,
    pub max_retries: u32,
    pub mock_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConfig {
    pub max_calls_per_hour: u32,
    pub max_tokens_per_call: u32,
    pub daily_spend_cap_usd: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfig {
    pub enabled: bool,
    pub ttl_seconds: u64,
    pub directory: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    pub redaction_enabled: bool,
    pub retention_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayConfig {
    pub export_enabled: bool,
    pub directory: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    pub base_dir: String,
    pub requests_dir: String,
    pub responses_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    pub log_file: String,
    pub metrics_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    // Central runtime configuration loaded from JSON.
    pub mode: RuntimeMode,
    pub runtime_build_id: String,
    pub prompt_version: String,
    pub openai: OpenAiConfig,
    pub budgets: BudgetConfig,
    pub caching: CachingConfig,
    pub privacy: PrivacyConfig,
    pub replay: ReplayConfig,
    pub bridge: BridgeConfig,
    pub observability: ObservabilityConfig,
}

impl RuntimeConfig {
    pub fn from_path(path: &Path) -> anyhow::Result<Self> {
        let body = fs::read_to_string(path)?;
        let cfg = serde_json::from_str::<Self>(&body)?;
        Ok(cfg)
    }

    pub fn default_for_dev() -> Self {
        // Developer defaults favor debuggability: mock provider + replay export on.
        Self {
            mode: RuntimeMode::Developer,
            runtime_build_id: "dev-build".to_string(),
            prompt_version: "recap-v1".to_string(),
            openai: OpenAiConfig {
                model: "gpt-4.1-mini".to_string(),
                timeout_ms: 20_000,
                max_retries: 2,
                mock_mode: true,
            },
            budgets: BudgetConfig {
                max_calls_per_hour: 120,
                max_tokens_per_call: 1200,
                daily_spend_cap_usd: 2.0,
            },
            caching: CachingConfig {
                enabled: true,
                ttl_seconds: 3600,
                directory: ".cache".to_string(),
            },
            privacy: PrivacyConfig {
                redaction_enabled: true,
                retention_policy: "minimal".to_string(),
            },
            replay: ReplayConfig {
                export_enabled: true,
                directory: "replay-bundles".to_string(),
            },
            bridge: BridgeConfig {
                base_dir: "bridge".to_string(),
                requests_dir: "bridge/requests".to_string(),
                responses_dir: "bridge/responses".to_string(),
            },
            observability: ObservabilityConfig {
                log_file: "runtime.log".to_string(),
                metrics_file: "metrics.json".to_string(),
            },
        }
    }
}
