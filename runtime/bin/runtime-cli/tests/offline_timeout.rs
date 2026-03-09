use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn simulate_times_out_when_runtime_is_offline() {
    let tmp = tempdir().expect("tmp");
    let config_path = tmp.path().join("config.dev.json");
    let bridge_dir = tmp.path().join("bridge");
    let replay_dir = tmp.path().join("replay");
    let cache_dir = tmp.path().join("cache");
    let log_file = tmp.path().join("runtime.log");
    let metrics_file = tmp.path().join("metrics.json");

    let config = serde_json::json!({
        "mode": "developer",
        "runtime_build_id": "test-build",
        "prompt_version": "recap-v1",
        "openai": {
            "model": "gpt-4.1-mini",
            "timeout_ms": 20000,
            "max_retries": 0,
            "mock_mode": true
        },
        "budgets": {
            "max_calls_per_hour": 120,
            "max_tokens_per_call": 1200,
            "daily_spend_cap_usd": 2.0
        },
        "caching": {
            "enabled": true,
            "ttl_seconds": 3600,
            "directory": cache_dir
        },
        "privacy": {
            "redaction_enabled": true,
            "retention_policy": "minimal"
        },
        "replay": {
            "export_enabled": true,
            "directory": replay_dir
        },
        "bridge": {
            "base_dir": bridge_dir,
            "requests_dir": bridge_dir.join("requests"),
            "responses_dir": bridge_dir.join("responses")
        },
        "observability": {
            "log_file": log_file,
            "metrics_file": metrics_file
        }
    });
    fs::write(
        &config_path,
        serde_json::to_string_pretty(&config).expect("config body"),
    )
    .expect("write config");

    let output = Command::new(env!("CARGO_BIN_EXE_runtime-cli"))
        .arg("simulate")
        .arg("--config")
        .arg(&config_path)
        .arg("--timeout-ms")
        .arg("200")
        .output()
        .expect("run simulate");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("timeout waiting for response"));
}
