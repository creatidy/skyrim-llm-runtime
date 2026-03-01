use anyhow::{anyhow, Context};
use chrono::Utc;
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
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use transport_file::FileTransport;
use uuid::Uuid;

fn main() -> anyhow::Result<()> {
    // Lightweight manual CLI parsing keeps dependencies minimal for PoC.
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        print_usage();
        return Ok(());
    }

    match args[0].as_str() {
        "serve" => cmd_serve(&args[1..]),
        "replay" => cmd_replay(&args[1..]),
        "simulate" => cmd_simulate(&args[1..]),
        "init-config" => cmd_init_config(&args[1..]),
        _ => {
            print_usage();
            Err(anyhow!("unsupported command: {}", args[0]))
        }
    }
}

fn cmd_serve(args: &[String]) -> anyhow::Result<()> {
    let transport = get_option(args, "--transport").unwrap_or_else(|| "file".to_string());
    if transport != "file" {
        return Err(anyhow!("only --transport file is supported in PoC"));
    }
    let config_path = get_option(args, "--config");
    let once = has_flag(args, "--once");

    let config = load_config(config_path.as_deref())?;
    ensure_runtime_paths(&config)?;

    let provider = OpenAiProvider::new(config.openai.model.clone());
    let cache = FileCacheStore::new(&config.caching.directory, config.caching.ttl_seconds)?;
    let safety = DefaultSafetyPipeline::new(config.privacy.redaction_enabled);
    let replay = FileReplayStore::new(&config.replay.directory)?;
    let logger = StructuredLogger::new(&config.observability.log_file);

    // Compose the runtime pipeline from swappable components.
    let mut service = RecapService {
        config: config.clone(),
        provider,
        cache,
        safety,
        replay,
        metrics: Default::default(),
        logger,
    };

    let mut file_transport =
        FileTransport::new(&config.bridge.requests_dir, &config.bridge.responses_dir)?;

    // Main worker loop: poll for request files, process, emit response files.
    loop {
        let maybe = file_transport
            .pop_next_request(Duration::from_secs(2))
            .map_err(|e| anyhow!(e.to_string()))?;
        let Some(envelope) = maybe else {
            if once {
                break;
            }
            continue;
        };

        let request_id = envelope.request.request_id.clone();
        let response = service.process_request(envelope.request);
        file_transport
            .write_response(&envelope, &response)
            .map_err(|e| anyhow!(e.to_string()))?;

        service
            .metrics
            .flush_to_file(&config.observability.metrics_file)
            .ok();

        println!(
            "processed request_id={} ok={} cache_hit={}",
            request_id, response.ok, response.meta.cache.hit
        );

        if once {
            break;
        }
    }

    Ok(())
}

fn cmd_replay(args: &[String]) -> anyhow::Result<()> {
    // Replay reads an exported bundle and prints a quick human-readable summary.
    let bundle = get_option(args, "--bundle").ok_or_else(|| anyhow!("missing --bundle <path>"))?;
    let summary = FileReplayStore::replay_bundle_summary(bundle)?;
    println!("{summary}");
    Ok(())
}

fn cmd_simulate(args: &[String]) -> anyhow::Result<()> {
    let config_path = get_option(args, "--config");
    let config = load_config(config_path.as_deref())?;

    ensure_runtime_paths(&config)?;

    let spoiler_mode = match get_option(args, "--spoiler-mode").as_deref() {
        Some("full") => SpoilerMode::Full,
        _ => SpoilerMode::Safe,
    };

    // Simulator acts like a Skyrim client by writing request JSON to the bridge.
    let request = build_sample_request(spoiler_mode);
    let transport = FileTransport::new(&config.bridge.requests_dir, &config.bridge.responses_dir)?;
    transport
        .write_request_file(&request)
        .map_err(|e| anyhow!(e.to_string()))?;

    let response = transport
        .wait_for_response(&request.request_id, Duration::from_secs(15))
        .map_err(|e| anyhow!(e.to_string()))?
        .ok_or_else(|| anyhow!("timeout waiting for response"))?;

    println!("request_id={} ok={}", request.request_id, response.ok);
    if let Some(recap) = response.recap {
        println!("summary={}", recap.summary);
        for (i, step) in recap.next_steps.iter().enumerate() {
            println!("next_step_{}={}", i + 1, step);
        }
    }
    if let Some(err) = response.error {
        println!("error_code={:?} message={}", err.code, err.message);
    }

    Ok(())
}

fn cmd_init_config(args: &[String]) -> anyhow::Result<()> {
    let out = get_option(args, "--out").unwrap_or_else(|| "config.dev.json".to_string());
    let cfg = RuntimeConfig::default_for_dev();
    let body = serde_json::to_string_pretty(&cfg)?;
    let path = PathBuf::from(out);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, body)?;
    println!("wrote {}", path.display());
    Ok(())
}

fn build_sample_request(spoiler_mode: SpoilerMode) -> RecapRequestV1 {
    // Minimal synthetic payload used for local, Skyrim-free E2E checks.
    RecapRequestV1 {
        contract_version: RECAP_REQUEST_VERSION.to_string(),
        request_id: Uuid::new_v4().to_string(),
        feature: "recap".to_string(),
        created_at_utc: Utc::now(),
        spoiler_mode,
        game_context: GameContext {
            game_time: Some("4E 201, 17:20".to_string()),
            player_location: "Whiterun".to_string(),
            event_log: vec![
                EventEntryV1 {
                    t: "17:03".to_string(),
                    kind: "quest".to_string(),
                    text: "Spoke to Farengar and received a dragonstone objective.".to_string(),
                },
                EventEntryV1 {
                    t: "17:10".to_string(),
                    kind: "location".to_string(),
                    text: "Left Dragonsreach and prepared to travel.".to_string(),
                },
            ],
        },
        client: ClientInfo {
            client_kind: "skyrim-mod".to_string(),
            client_version: "0.1.0".to_string(),
            profile: ClientProfile::Developer,
        },
    }
}

fn load_config(path: Option<&str>) -> anyhow::Result<RuntimeConfig> {
    if let Some(p) = path {
        RuntimeConfig::from_path(Path::new(p)).with_context(|| format!("loading config from {p}"))
    } else {
        Ok(RuntimeConfig::default_for_dev())
    }
}

fn ensure_runtime_paths(config: &RuntimeConfig) -> anyhow::Result<()> {
    // Ensure directories exist before polling/writing to avoid noisy runtime failures.
    fs::create_dir_all(&config.bridge.base_dir)?;
    fs::create_dir_all(&config.bridge.requests_dir)?;
    fs::create_dir_all(&config.bridge.responses_dir)?;
    fs::create_dir_all(&config.caching.directory)?;
    fs::create_dir_all(&config.replay.directory)?;

    if let Some(parent) = Path::new(&config.observability.log_file).parent() {
        fs::create_dir_all(parent)?;
    }
    if let Some(parent) = Path::new(&config.observability.metrics_file).parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn get_option(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|idx| args.get(idx + 1))
        .map(ToString::to_string)
}

fn has_flag(args: &[String], key: &str) -> bool {
    args.iter().any(|a| a == key)
}

fn print_usage() {
    println!(
        "runtime-cli commands:\n  serve --transport file [--config <path>] [--once]\n  simulate [--config <path>] [--spoiler-mode safe|full]\n  replay --bundle <path>\n  init-config [--out <path>]"
    );
}
