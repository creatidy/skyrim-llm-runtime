use runtime_core::cache::{CachedRecap, FileCacheStore};
use runtime_core::contracts::{
    BudgetsMeta, ClientInfo, ClientProfile, EventEntryV1, GameContext, ProviderMeta, RecapRequestV1,
    SpoilerMode, RECAP_REQUEST_VERSION,
};
use runtime_core::replay::FileReplayStore;
use runtime_core::safety::DefaultSafetyPipeline;
use runtime_core::traits::{CacheStore, ReplayStore, SafetyPipeline};
use tempfile::tempdir;

fn request_with_text(text: &str) -> RecapRequestV1 {
    RecapRequestV1 {
        contract_version: RECAP_REQUEST_VERSION.to_string(),
        request_id: "req-safety".to_string(),
        feature: "recap".to_string(),
        created_at_utc: chrono::Utc::now(),
        spoiler_mode: SpoilerMode::Safe,
        game_context: GameContext {
            game_time: None,
            player_location: "C:\\Users\\Player\\Documents".to_string(),
            event_log: vec![EventEntryV1 {
                t: "t".to_string(),
                kind: "note".to_string(),
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

#[test]
fn safety_redacts_and_enforces_output_constraints() {
    let safety = DefaultSafetyPipeline::new(true);
    let req = request_with_text("Met Alduin near the mountain");
    let sanitized = safety.sanitize_request(&req);
    assert!(sanitized.game_context.player_location.contains("redacted"));

    let payload = safety
        .validate_candidate(
            &sanitized,
            runtime_core::contracts::CandidateRecap {
                summary: "Alduin encountered near peak".to_string(),
                next_steps: vec![
                    "Review inventory".to_string(),
                    "Check map".to_string(),
                    "Prepare before travel".to_string(),
                ],
                provider_name: "openai".to_string(),
                provider_model: "gpt-4.1-mini".to_string(),
                tokens_in: Some(100),
                tokens_out: Some(50),
            },
        )
        .expect("payload");

    assert!(payload.summary.contains("[spoiler-redacted]") || payload.spoiler_risk == runtime_core::contracts::SpoilerRisk::None);
    assert_eq!(payload.next_steps.len(), 3);
}

#[test]
fn file_cache_returns_hit_for_identical_request() {
    let tmp = tempdir().expect("tmp");
    let mut cache = FileCacheStore::new(tmp.path(), 3600).expect("cache");
    let req = request_with_text("Fetched an objective");

    let key = cache.stable_key_for(&req, "prompt-v1", "gpt-4.1-mini");
    let value = CachedRecap {
        recap: runtime_core::contracts::default_fallback_recap(),
        provider: ProviderMeta {
            name: "openai".to_string(),
            model: "gpt-4.1-mini".to_string(),
        },
        budgets: BudgetsMeta {
            tokens_in: Some(10),
            tokens_out: Some(20),
        },
        created_at_utc: chrono::Utc::now(),
    };

    cache.set(&key, &value).expect("set");
    let hit = cache.get_fresh(&key).expect("get");
    assert!(hit.is_some());
}

#[test]
fn replay_bundle_contains_required_files() {
    let tmp = tempdir().expect("tmp");
    let replay = FileReplayStore::new(tmp.path()).expect("replay");
    let req = request_with_text("Completed side quest");

    let resp = runtime_core::contracts::RecapResponseV1::success(
        &req.request_id,
        runtime_core::contracts::default_fallback_recap(),
        runtime_core::contracts::ResponseMeta {
            runtime_build_id: "dev".to_string(),
            prompt_version: "v1".to_string(),
            provider: ProviderMeta {
                name: "openai".to_string(),
                model: "gpt-4.1-mini".to_string(),
            },
            cache: runtime_core::contracts::CacheMeta {
                hit: false,
                key: None,
                age_seconds: None,
            },
            budgets: BudgetsMeta {
                tokens_in: Some(10),
                tokens_out: Some(20),
            },
            created_at_utc: chrono::Utc::now(),
        },
    );

    replay
        .export_bundle(&req, &resp, Some("{\"redacted\": true}"))
        .expect("export");

    let dir = tmp.path().join(&req.request_id);
    assert!(dir.join("request.json").exists());
    assert!(dir.join("response.json").exists());
    assert!(dir.join("runtime_build_id.txt").exists());
    assert!(dir.join("prompt_version.txt").exists());
    assert!(dir.join("provider.json").exists());
    assert!(dir.join("timestamps.json").exists());
}
