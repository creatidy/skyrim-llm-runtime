use crate::{
    cache::CachedRecap,
    config::RuntimeConfig,
    contracts::{CandidateRecap, RecapRequestV1, RecapResponseV1},
};
use std::{path::PathBuf, time::Duration};

#[derive(Debug, Clone)]
pub struct TransportEnvelope {
    pub request_path: PathBuf,
    pub response_path: PathBuf,
    pub request: RecapRequestV1,
}

#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    #[error("io error: {0}")]
    Io(String),
    #[error("decode error: {0}")]
    Decode(String),
    #[error("encode error: {0}")]
    Encode(String),
}

pub trait Transport {
    fn pop_next_request(&mut self, wait_for: Duration) -> Result<Option<TransportEnvelope>, TransportError>;
    fn write_response(
        &mut self,
        envelope: &TransportEnvelope,
        response: &RecapResponseV1,
    ) -> Result<(), TransportError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("provider unavailable: {0}")]
    Unavailable(String),
    #[error("provider invalid output: {0}")]
    InvalidOutput(String),
}

pub trait Provider {
    fn name(&self) -> &str;
    fn model(&self) -> &str;
    fn generate_recap(
        &self,
        request: &RecapRequestV1,
        config: &RuntimeConfig,
    ) -> Result<CandidateRecap, ProviderError>;
}

pub trait SafetyPipeline {
    fn sanitize_request(&self, request: &RecapRequestV1) -> RecapRequestV1;
    fn validate_candidate(
        &self,
        request: &RecapRequestV1,
        candidate: CandidateRecap,
    ) -> Result<crate::contracts::RecapPayload, String>;
    fn fallback_recap(&self) -> crate::contracts::RecapPayload;
}

pub trait CacheStore {
    fn stable_key_for(&self, request: &RecapRequestV1, prompt_version: &str, model: &str) -> String;
    fn get_fresh(&mut self, key: &str) -> anyhow::Result<Option<(CachedRecap, u64)>>;
    fn get_stale(&mut self, key: &str) -> anyhow::Result<Option<CachedRecap>>;
    fn set(&mut self, key: &str, value: &CachedRecap) -> anyhow::Result<()>;
}

pub trait ReplayStore {
    fn export_bundle(
        &self,
        request: &RecapRequestV1,
        response: &RecapResponseV1,
        redaction_report: Option<&str>,
    ) -> anyhow::Result<()>;
}
