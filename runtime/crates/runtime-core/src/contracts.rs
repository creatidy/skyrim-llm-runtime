use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Request/response versions are part of the runtime public contract.
pub const RECAP_REQUEST_VERSION: &str = "v1";
pub const RECAP_RESPONSE_VERSION: &str = "v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecapRequestV1 {
    pub contract_version: String,
    pub request_id: String,
    pub feature: String,
    pub created_at_utc: DateTime<Utc>,
    pub spoiler_mode: SpoilerMode,
    pub game_context: GameContext,
    pub client: ClientInfo,
}

impl RecapRequestV1 {
    pub fn validate_basic(&self) -> Result<(), String> {
        // Fast preflight validation before any provider/caching work.
        if self.contract_version != RECAP_REQUEST_VERSION {
            return Err(format!(
                "unsupported request contract version: {}",
                self.contract_version
            ));
        }
        if self.feature != "recap" {
            return Err(format!("unsupported feature: {}", self.feature));
        }
        if self.request_id.trim().is_empty() {
            return Err("request_id is empty".to_string());
        }
        if self.game_context.event_log.is_empty() {
            return Err("event_log is empty".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SpoilerMode {
    Safe,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameContext {
    pub game_time: Option<String>,
    pub player_location: String,
    pub event_log: Vec<EventEntryV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventEntryV1 {
    pub t: String,
    pub kind: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientInfo {
    pub client_kind: String,
    pub client_version: String,
    pub profile: ClientProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ClientProfile {
    Player,
    Developer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecapResponseV1 {
    pub contract_version: String,
    pub request_id: String,
    pub ok: bool,
    pub recap: Option<RecapPayload>,
    pub meta: ResponseMeta,
    pub error: Option<RuntimeErrorPayload>,
}

impl RecapResponseV1 {
    // Helper constructor for successful runtime responses.
    pub fn success(request_id: &str, recap: RecapPayload, meta: ResponseMeta) -> Self {
        Self {
            contract_version: RECAP_RESPONSE_VERSION.to_string(),
            request_id: request_id.to_string(),
            ok: true,
            recap: Some(recap),
            meta,
            error: None,
        }
    }

    // Helper constructor for structured failures.
    pub fn failure(request_id: &str, code: RuntimeErrorCode, message: &str, meta: ResponseMeta) -> Self {
        Self {
            contract_version: RECAP_RESPONSE_VERSION.to_string(),
            request_id: request_id.to_string(),
            ok: false,
            recap: None,
            meta,
            error: Some(RuntimeErrorPayload {
                code,
                message: message.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecapPayload {
    pub summary: String,
    pub next_steps: Vec<String>,
    pub spoiler_risk: SpoilerRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SpoilerRisk {
    None,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponseMeta {
    pub runtime_build_id: String,
    pub prompt_version: String,
    pub provider: ProviderMeta,
    pub cache: CacheMeta,
    pub budgets: BudgetsMeta,
    pub created_at_utc: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderMeta {
    pub name: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheMeta {
    pub hit: bool,
    pub key: Option<String>,
    pub age_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BudgetsMeta {
    pub tokens_in: Option<u32>,
    pub tokens_out: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeErrorPayload {
    pub code: RuntimeErrorCode,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeErrorCode {
    RuntimeOffline,
    ProviderError,
    BudgetExceeded,
    ValidationFailed,
    TransportError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateRecap {
    // Provider output before safety validation/coercion.
    pub summary: String,
    pub next_steps: Vec<String>,
    pub provider_name: String,
    pub provider_model: String,
    pub tokens_in: Option<u32>,
    pub tokens_out: Option<u32>,
}

pub fn default_fallback_recap() -> RecapPayload {
    // Safe fallback used when provider fails or candidate validation fails.
    RecapPayload {
        summary: "Unable to generate a fresh recap. Here is a safe fallback summary based on recent activity.".to_string(),
        next_steps: vec![
            "Review your latest quest objective in the journal.".to_string(),
            "Open the map and confirm your intended destination.".to_string(),
            "Create a quick save before resuming exploration.".to_string(),
        ],
        spoiler_risk: SpoilerRisk::None,
    }
}
