use runtime_core::cache::CachedRecap;
use runtime_core::config::RuntimeConfig;
use runtime_core::contracts::{
    default_fallback_recap, BudgetsMeta, CandidateRecap, ClientInfo, ClientProfile, EventEntryV1,
    GameContext, ProviderMeta, RecapPayload, RecapRequestV1, SpoilerMode, SpoilerRisk,
    RECAP_REQUEST_VERSION,
};
use runtime_core::observability::StructuredLogger;
use runtime_core::replay::FileReplayStore;
use runtime_core::safety::DefaultSafetyPipeline;
use runtime_core::service::RecapService;
use runtime_core::traits::{CacheStore, Provider, ProviderError};

struct FakeProvider {
    candidate: Option<CandidateRecap>,
    error: Option<String>,
    model: String,
}

impl Provider for FakeProvider {
    fn name(&self) -> &str {
        "openai"
    }

    fn model(&self) -> &str {
        &self.model
    }

    fn generate_recap(
        &self,
        _request: &RecapRequestV1,
        _config: &RuntimeConfig,
    ) -> Result<CandidateRecap, ProviderError> {
        match (&self.candidate, &self.error) {
            (Some(candidate), _) => Ok(candidate.clone()),
            (None, Some(error)) => Err(ProviderError::Unavailable(error.clone())),
            (None, None) => Err(ProviderError::Unavailable("no fake result".to_string())),
        }
    }
}

#[derive(Default)]
struct FakeCache {
    fresh: Option<(CachedRecap, u64)>,
    stale: Option<CachedRecap>,
}

impl CacheStore for FakeCache {
    fn stable_key_for(
        &self,
        _request: &RecapRequestV1,
        _prompt_version: &str,
        _model: &str,
    ) -> String {
        "cache-key".to_string()
    }

    fn get_fresh(&mut self, _key: &str) -> anyhow::Result<Option<(CachedRecap, u64)>> {
        Ok(self.fresh.clone())
    }

    fn get_stale(&mut self, _key: &str) -> anyhow::Result<Option<CachedRecap>> {
        Ok(self.stale.clone())
    }

    fn set(&mut self, _key: &str, _value: &CachedRecap) -> anyhow::Result<()> {
        Ok(())
    }
}

fn request_with_text(text: &str) -> RecapRequestV1 {
    RecapRequestV1 {
        contract_version: RECAP_REQUEST_VERSION.to_string(),
        request_id: "req-service".to_string(),
        feature: "recap".to_string(),
        created_at_utc: chrono::Utc::now(),
        spoiler_mode: SpoilerMode::Safe,
        game_context: GameContext {
            game_time: None,
            player_location: "Whiterun".to_string(),
            event_log: vec![EventEntryV1 {
                t: "17:00".to_string(),
                kind: "quest".to_string(),
                text: text.to_string(),
            }],
        },
        client: ClientInfo {
            client_kind: "skyrim-mod".to_string(),
            client_version: "0.1.0".to_string(),
            profile: ClientProfile::Developer,
        },
    }
}

fn cached_recap(summary: &str) -> CachedRecap {
    CachedRecap {
        recap: RecapPayload {
            summary: summary.to_string(),
            next_steps: vec![
                "Check your journal.".to_string(),
                "Review your map.".to_string(),
                "Pick the next objective.".to_string(),
            ],
            spoiler_risk: SpoilerRisk::None,
        },
        provider: ProviderMeta {
            name: "openai".to_string(),
            model: "gpt-4.1-mini".to_string(),
        },
        budgets: BudgetsMeta {
            tokens_in: Some(20),
            tokens_out: Some(30),
        },
        created_at_utc: chrono::Utc::now(),
    }
}

fn build_service(
    provider: FakeProvider,
    cache: FakeCache,
) -> RecapService<FakeProvider, FakeCache, DefaultSafetyPipeline, FileReplayStore> {
    let root = std::env::temp_dir().join(format!(
        "skyrim-llm-runtime-service-test-{}",
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ));
    std::fs::create_dir_all(&root).expect("create test root");
    let log_file = root.join("runtime.log");
    let replay_dir = root.join("replay");

    let mut config = RuntimeConfig::default_for_dev();
    config.openai.mock_mode = true;
    config.replay.directory = replay_dir.to_string_lossy().to_string();
    config.observability.log_file = log_file.to_string_lossy().to_string();

    RecapService {
        config,
        provider,
        cache,
        safety: DefaultSafetyPipeline::new(true),
        replay: FileReplayStore::new(replay_dir).expect("replay"),
        metrics: Default::default(),
        logger: StructuredLogger::new(log_file),
    }
}

#[test]
fn provider_error_returns_stale_cache_when_available() {
    let provider = FakeProvider {
        candidate: None,
        error: Some("forced failure".to_string()),
        model: "gpt-4.1-mini".to_string(),
    };
    let stale = cached_recap("Stale recap from cache.");
    let cache = FakeCache {
        fresh: None,
        stale: Some(stale.clone()),
    };

    let mut service = build_service(provider, cache);
    let response = service.process_request(request_with_text("Some request"));

    assert!(response.ok);
    assert!(response.meta.cache.hit);
    assert_eq!(response.recap.expect("recap").summary, stale.recap.summary);
}

#[test]
fn provider_error_returns_safe_fallback_without_stale_cache() {
    let provider = FakeProvider {
        candidate: None,
        error: Some("forced failure".to_string()),
        model: "gpt-4.1-mini".to_string(),
    };

    let mut service = build_service(provider, FakeCache::default());
    let response = service.process_request(request_with_text("Fresh request"));

    assert!(response.ok);
    assert!(!response.meta.cache.hit);
    assert_eq!(
        response.recap.expect("recap").summary,
        default_fallback_recap().summary
    );
}

#[test]
fn invalid_provider_output_returns_safe_fallback_without_stale_cache() {
    let provider = FakeProvider {
        candidate: Some(CandidateRecap {
            summary: "".to_string(),
            next_steps: vec!["Only one".to_string()],
            provider_name: "openai".to_string(),
            provider_model: "gpt-4.1-mini".to_string(),
            tokens_in: Some(11),
            tokens_out: Some(12),
        }),
        error: None,
        model: "gpt-4.1-mini".to_string(),
    };

    let mut service = build_service(provider, FakeCache::default());
    let response = service.process_request(request_with_text("Needs validation fallback"));

    assert!(response.ok);
    assert_eq!(
        response.recap.expect("recap").summary,
        default_fallback_recap().summary
    );
}

#[test]
fn invalid_provider_output_returns_stale_cache_when_available() {
    let provider = FakeProvider {
        candidate: Some(CandidateRecap {
            summary: "".to_string(),
            next_steps: vec!["Only one".to_string()],
            provider_name: "openai".to_string(),
            provider_model: "gpt-4.1-mini".to_string(),
            tokens_in: Some(11),
            tokens_out: Some(12),
        }),
        error: None,
        model: "gpt-4.1-mini".to_string(),
    };
    let stale = cached_recap("Cached recap after invalid output.");
    let cache = FakeCache {
        fresh: None,
        stale: Some(stale.clone()),
    };

    let mut service = build_service(provider, cache);
    let response = service.process_request(request_with_text("Needs stale fallback"));

    assert!(response.ok);
    assert!(response.meta.cache.hit);
    assert_eq!(response.recap.expect("recap").summary, stale.recap.summary);
}
