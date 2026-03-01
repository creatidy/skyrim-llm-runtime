use provider_openai::OpenAiProvider;
use runtime_core::cache::FileCacheStore;
use runtime_core::config::RuntimeConfig;
use runtime_core::contracts::{
    ClientInfo, ClientProfile, EventEntryV1, GameContext, RecapRequestV1, SpoilerMode,
    RECAP_REQUEST_VERSION,
};
use runtime_core::observability::StructuredLogger;
use runtime_core::replay::FileReplayStore;
use runtime_core::safety::DefaultSafetyPipeline;
use runtime_core::service::RecapService;
use runtime_core::traits::Transport;
use std::time::Duration;
use tempfile::tempdir;
use transport_file::FileTransport;

#[test]
fn simulator_loop_produces_response_and_replay() {
    let tmp = tempdir().expect("tmp");
    let bridge = tmp.path().join("bridge");
    let requests = bridge.join("requests");
    let responses = bridge.join("responses");
    let replay_dir = tmp.path().join("replay");
    let cache_dir = tmp.path().join("cache");
    let log_file = tmp.path().join("runtime.log");
    let metrics_file = tmp.path().join("metrics.json");

    let mut cfg = RuntimeConfig::default_for_dev();
    cfg.bridge.base_dir = bridge.to_string_lossy().to_string();
    cfg.bridge.requests_dir = requests.to_string_lossy().to_string();
    cfg.bridge.responses_dir = responses.to_string_lossy().to_string();
    cfg.replay.directory = replay_dir.to_string_lossy().to_string();
    cfg.caching.directory = cache_dir.to_string_lossy().to_string();
    cfg.observability.log_file = log_file.to_string_lossy().to_string();
    cfg.observability.metrics_file = metrics_file.to_string_lossy().to_string();
    cfg.openai.mock_mode = true;

    let provider = OpenAiProvider::new(cfg.openai.model.clone());
    let cache = FileCacheStore::new(&cfg.caching.directory, cfg.caching.ttl_seconds).expect("cache");
    let safety = DefaultSafetyPipeline::new(true);
    let replay = FileReplayStore::new(&cfg.replay.directory).expect("replay");
    let logger = StructuredLogger::new(&cfg.observability.log_file);

    let mut service = RecapService {
        config: cfg.clone(),
        provider,
        cache,
        safety,
        replay,
        metrics: Default::default(),
        logger,
    };

    let mut server_transport = FileTransport::new(&cfg.bridge.requests_dir, &cfg.bridge.responses_dir)
        .expect("server transport");
    let client_transport = FileTransport::new(&cfg.bridge.requests_dir, &cfg.bridge.responses_dir)
        .expect("client transport");

    let req = RecapRequestV1 {
        contract_version: RECAP_REQUEST_VERSION.to_string(),
        request_id: "e2e-1".to_string(),
        feature: "recap".to_string(),
        created_at_utc: chrono::Utc::now(),
        spoiler_mode: SpoilerMode::Safe,
        game_context: GameContext {
            game_time: Some("4E 201, 12:00".to_string()),
            player_location: "Whiterun".to_string(),
            event_log: vec![EventEntryV1 {
                t: "11:55".to_string(),
                kind: "quest".to_string(),
                text: "Visited Dragonsreach".to_string(),
            }],
        },
        client: ClientInfo {
            client_kind: "skyrim-mod".to_string(),
            client_version: "0.1.0".to_string(),
            profile: ClientProfile::Developer,
        },
    };

    client_transport.write_request_file(&req).expect("write req");

    let envelope = server_transport
        .pop_next_request(Duration::from_secs(2))
        .expect("pop req")
        .expect("some req");
    let response = service.process_request(envelope.request);
    server_transport
        .write_response(&envelope, &response)
        .expect("write response");

    let found = client_transport
        .wait_for_response(&req.request_id, Duration::from_secs(2))
        .expect("wait response")
        .expect("response file");

    assert!(found.ok);
    assert!(found.recap.is_some());
    assert!(found.meta.provider.name == "openai");

    let replay_path = std::path::Path::new(&cfg.replay.directory).join(&req.request_id);
    assert!(replay_path.join("request.json").exists());
    assert!(replay_path.join("response.json").exists());
}
