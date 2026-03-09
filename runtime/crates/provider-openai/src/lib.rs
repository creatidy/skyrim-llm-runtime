use chrono::Utc;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use runtime_core::config::RuntimeConfig;
use runtime_core::contracts::{CandidateRecap, RecapRequestV1};
use runtime_core::traits::{Provider, ProviderError};
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::thread;
use std::time::Duration;

const RESPONSES_API_BASE_URL: &str = "https://api.openai.com/v1";

#[derive(Debug, Clone)]
pub struct OpenAiProvider {
    model: String,
}

#[derive(Debug)]
struct ProviderAttemptError {
    error: ProviderError,
    retryable: bool,
}

#[derive(Debug)]
struct RawProviderOutput {
    raw_json: String,
    provider_model: String,
    tokens_in: Option<u32>,
    tokens_out: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct ResponsesApiResponse {
    model: Option<String>,
    status: Option<String>,
    output: Option<Vec<ResponseOutputItem>>,
    usage: Option<ResponseUsage>,
}

#[derive(Debug, Deserialize)]
struct ResponseOutputItem {
    content: Option<Vec<ResponseContentItem>>,
}

#[derive(Debug, Deserialize)]
struct ResponseContentItem {
    #[serde(rename = "type")]
    item_type: String,
    text: Option<String>,
    refusal: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ResponseUsage {
    input_tokens: Option<u32>,
    output_tokens: Option<u32>,
}

impl OpenAiProvider {
    pub fn new(model: String) -> Self {
        Self { model }
    }

    fn build_prompt(request: &RecapRequestV1) -> String {
        // Prompt stays intentionally small/structured to fit recap contract.
        let events = request
            .game_context
            .event_log
            .iter()
            .map(|e| format!("- [{}] {}: {}", e.t, e.kind, e.text))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "You are a Skyrim recap assistant. Return strict JSON with fields summary and next_steps (3-5 items).\nLocation: {}\nSpoiler mode: {:?}\nEvents:\n{}",
            request.game_context.player_location, request.spoiler_mode, events
        )
    }

    fn build_response_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["summary", "next_steps"],
            "properties": {
                "summary": {
                    "type": "string"
                },
                "next_steps": {
                    "type": "array",
                    "minItems": 3,
                    "maxItems": 5,
                    "items": {
                        "type": "string"
                    }
                }
            }
        })
    }

    fn build_live_request(&self, prompt: &str) -> serde_json::Value {
        json!({
            "model": self.model,
            "input": prompt,
            "text": {
                "format": {
                    "type": "json_schema",
                    "name": "skyrim_recap",
                    "strict": true,
                    "schema": Self::build_response_schema(),
                }
            }
        })
    }

    fn generate_mock_output(
        &self,
        request: &RecapRequestV1,
        estimated_tokens_in: u32,
    ) -> Result<RawProviderOutput, ProviderError> {
        // Test hooks to exercise provider-failure and invalid-output branches.
        if request
            .game_context
            .event_log
            .iter()
            .any(|e| e.text.contains("[force_provider_error]"))
        {
            return Err(ProviderError::Unavailable(
                "forced provider error for test scenario".to_string(),
            ));
        }

        if request
            .game_context
            .event_log
            .iter()
            .any(|e| e.text.contains("[force_invalid]"))
        {
            return Ok(RawProviderOutput {
                raw_json: json!({
                    "summary": "",
                    "next_steps": ["Only one step"]
                })
                .to_string(),
                provider_model: self.model.clone(),
                tokens_in: Some(estimated_tokens_in),
                tokens_out: Some(16),
            });
        }

        let location = &request.game_context.player_location;
        let most_recent = request
            .game_context
            .event_log
            .last()
            .map(|e| e.text.clone())
            .unwrap_or_else(|| "No recent events.".to_string());

        let raw_json = json!({
            "summary": format!("You last progressed near {}. Recent notable action: {}", location, most_recent),
            "next_steps": [
                "Check your active quest objective and map marker.",
                "Restock potions and repair gear before deeper exploration.",
                "Set a short-term goal for your next fast-travel stop."
            ]
        })
        .to_string();

        Ok(RawProviderOutput {
            raw_json: raw_json.clone(),
            provider_model: self.model.clone(),
            tokens_in: Some(estimated_tokens_in),
            tokens_out: Some((raw_json.len() as u32 / 4).max(16)),
        })
    }

    fn resolve_api_key() -> Result<String, ProviderError> {
        match env::var("OPENAI_API_KEY") {
            Ok(value) if !value.trim().is_empty() => Ok(value),
            _ => Err(ProviderError::Unavailable(
                "OPENAI_API_KEY is not set; set it in the runtime environment or enable mock_mode"
                    .to_string(),
            )),
        }
    }

    fn parse_structured_output(
        provider_model: &str,
        raw: &str,
        tokens_in: Option<u32>,
        tokens_out: Option<u32>,
    ) -> Result<CandidateRecap, ProviderError> {
        // Parse into strict shape before safety pipeline enforces final constraints.
        #[derive(Deserialize)]
        struct StructuredRecap {
            summary: String,
            next_steps: Vec<String>,
        }

        let parsed: StructuredRecap = serde_json::from_str(raw)
            .map_err(|e| ProviderError::InvalidOutput(format!("invalid json: {e}")))?;

        Ok(CandidateRecap {
            summary: parsed.summary,
            next_steps: parsed.next_steps,
            provider_name: "openai".to_string(),
            provider_model: provider_model.to_string(),
            tokens_in,
            tokens_out,
        })
    }

    fn estimate_tokens(prompt: &str) -> u32 {
        (prompt.len() as u32 / 4).max(32)
    }

    fn send_live_request(
        &self,
        base_url: &str,
        api_key: &str,
        prompt: &str,
        timeout_ms: u64,
    ) -> Result<RawProviderOutput, ProviderAttemptError> {
        let client = Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .build()
            .map_err(|err| ProviderAttemptError {
                error: ProviderError::Unavailable(format!("failed to build http client: {err}")),
                retryable: false,
            })?;

        let request_body = self.build_live_request(prompt);
        let endpoint = format!("{}/responses", base_url.trim_end_matches('/'));

        let response = client
            .post(&endpoint)
            .bearer_auth(api_key)
            .json(&request_body)
            .send()
            .map_err(|err| ProviderAttemptError {
                error: ProviderError::Unavailable(format!("live provider request failed: {err}")),
                retryable: true,
            })?;

        let status = response.status();
        let body = response.text().map_err(|err| ProviderAttemptError {
            error: ProviderError::Unavailable(format!("failed to read provider response: {err}")),
            retryable: status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error(),
        })?;

        if !status.is_success() {
            let retryable = status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error();
            let preview = body.chars().take(200).collect::<String>();
            return Err(ProviderAttemptError {
                error: ProviderError::Unavailable(format!(
                    "live provider returned status {} body={}",
                    status.as_u16(),
                    preview
                )),
                retryable,
            });
        }

        let parsed = serde_json::from_str::<ResponsesApiResponse>(&body).map_err(|err| {
            ProviderAttemptError {
                error: ProviderError::InvalidOutput(format!(
                    "invalid responses api payload: {err}"
                )),
                retryable: false,
            }
        })?;

        let raw_json = Self::extract_output_text(&parsed).map_err(|err| ProviderAttemptError {
            error: err,
            retryable: false,
        })?;

        Ok(RawProviderOutput {
            raw_json,
            provider_model: parsed.model.unwrap_or_else(|| self.model.clone()),
            tokens_in: parsed.usage.as_ref().and_then(|usage| usage.input_tokens),
            tokens_out: parsed.usage.as_ref().and_then(|usage| usage.output_tokens),
        })
    }

    fn extract_output_text(parsed: &ResponsesApiResponse) -> Result<String, ProviderError> {
        if let Some(items) = &parsed.output {
            for item in items {
                let Some(content) = &item.content else {
                    continue;
                };
                for content_item in content {
                    match content_item.item_type.as_str() {
                        "output_text" => {
                            if let Some(text) = &content_item.text {
                                return Ok(text.clone());
                            }
                        }
                        "refusal" => {
                            let reason = content_item
                                .refusal
                                .clone()
                                .unwrap_or_else(|| "model refusal".to_string());
                            return Err(ProviderError::InvalidOutput(reason));
                        }
                        _ => {}
                    }
                }
            }
        }

        Err(ProviderError::InvalidOutput(format!(
            "responses api payload had no output_text content (status={})",
            parsed.status.as_deref().unwrap_or("unknown")
        )))
    }

    fn generate_live_recap(
        &self,
        request: &RecapRequestV1,
        config: &RuntimeConfig,
        base_url: &str,
        api_key: &str,
    ) -> Result<CandidateRecap, ProviderError> {
        let prompt = Self::build_prompt(request);
        let estimated_tokens_in = Self::estimate_tokens(&prompt);
        let mut last_err: Option<ProviderError> = None;

        for attempt in 0..=config.openai.max_retries {
            match self.send_live_request(base_url, api_key, &prompt, config.openai.timeout_ms) {
                Ok(output) => {
                    return Self::parse_structured_output(
                        &output.provider_model,
                        &output.raw_json,
                        output.tokens_in.or(Some(estimated_tokens_in)),
                        output.tokens_out,
                    );
                }
                Err(err) => {
                    last_err = Some(err.error);
                    if err.retryable && attempt < config.openai.max_retries {
                        thread::sleep(Duration::from_millis((attempt as u64 + 1) * 80));
                        continue;
                    }
                    break;
                }
            }
        }

        Err(last_err.unwrap_or_else(|| {
            ProviderError::Unavailable(format!(
                "unknown provider failure at {}",
                Utc::now().to_rfc3339()
            ))
        }))
    }
}

impl Provider for OpenAiProvider {
    fn name(&self) -> &str {
        "openai"
    }

    fn model(&self) -> &str {
        &self.model
    }

    fn generate_recap(
        &self,
        request: &RecapRequestV1,
        config: &RuntimeConfig,
    ) -> Result<CandidateRecap, ProviderError> {
        if config.openai.mock_mode {
            let prompt = Self::build_prompt(request);
            let estimated_tokens_in = Self::estimate_tokens(&prompt);
            let output = self.generate_mock_output(request, estimated_tokens_in)?;
            return Self::parse_structured_output(
                &output.provider_model,
                &output.raw_json,
                output.tokens_in,
                output.tokens_out,
            );
        }

        let api_key = Self::resolve_api_key()?;
        self.generate_live_recap(request, config, RESPONSES_API_BASE_URL, &api_key)
    }
}

#[cfg(test)]
mod tests {
    use super::OpenAiProvider;
    use runtime_core::config::RuntimeConfig;
    use runtime_core::contracts::{
        ClientInfo, ClientProfile, EventEntryV1, GameContext, RecapRequestV1, SpoilerMode,
        RECAP_REQUEST_VERSION,
    };
    use runtime_core::traits::{Provider, ProviderError};
    use serde_json::json;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::mpsc;
    use std::sync::Mutex;
    use std::thread;
    use std::time::Duration;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct TestResponse {
        status_line: &'static str,
        body: String,
        delay_ms: u64,
    }

    fn sample_request() -> RecapRequestV1 {
        RecapRequestV1 {
            contract_version: RECAP_REQUEST_VERSION.to_string(),
            request_id: "req-1".to_string(),
            feature: "recap".to_string(),
            created_at_utc: chrono::Utc::now(),
            spoiler_mode: SpoilerMode::Safe,
            game_context: GameContext {
                game_time: Some("4E 201, 17:20".to_string()),
                player_location: "Whiterun".to_string(),
                event_log: vec![EventEntryV1 {
                    t: "17:00".to_string(),
                    kind: "quest".to_string(),
                    text: "Spoke to Jarl Balgruuf".to_string(),
                }],
            },
            client: ClientInfo {
                client_kind: "skyrim-mod".to_string(),
                client_version: "0.1.0".to_string(),
                profile: ClientProfile::Developer,
            },
        }
    }

    fn response_payload(summary: &str) -> String {
        json!({
            "id": "resp_test",
            "status": "completed",
            "model": "gpt-4.1-mini-2026-03-01",
            "output": [{
                "content": [{
                    "type": "output_text",
                    "text": json!({
                        "summary": summary,
                        "next_steps": [
                            "Check your quest journal.",
                            "Restock before travel.",
                            "Pick the next nearby objective."
                        ]
                    }).to_string()
                }]
            }],
            "usage": {
                "input_tokens": 91,
                "output_tokens": 53
            }
        })
        .to_string()
    }

    fn spawn_test_server(
        responses: Vec<TestResponse>,
    ) -> (String, mpsc::Receiver<String>, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let addr = listener.local_addr().expect("addr");
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            for reply in responses {
                let (mut stream, _) = listener.accept().expect("accept");
                stream
                    .set_read_timeout(Some(Duration::from_millis(200)))
                    .expect("read timeout");

                let mut buffer = Vec::new();
                let mut chunk = [0_u8; 1024];
                let mut body_len = None;

                loop {
                    match stream.read(&mut chunk) {
                        Ok(0) => break,
                        Ok(n) => {
                            buffer.extend_from_slice(&chunk[..n]);
                            if body_len.is_none() {
                                if let Some(idx) = find_subsequence(&buffer, b"\r\n\r\n") {
                                    let headers = String::from_utf8_lossy(&buffer[..idx]);
                                    for line in headers.lines() {
                                        let lower = line.to_ascii_lowercase();
                                        if let Some(value) = lower.strip_prefix("content-length:") {
                                            body_len = value.trim().parse::<usize>().ok();
                                        }
                                    }
                                }
                            }

                            if let (Some(idx), Some(len)) =
                                (find_subsequence(&buffer, b"\r\n\r\n"), body_len)
                            {
                                let start = idx + 4;
                                if buffer.len() >= start + len {
                                    break;
                                }
                            }
                        }
                        Err(_) => {
                            if !buffer.is_empty() {
                                break;
                            }
                        }
                    }
                }

                tx.send(String::from_utf8_lossy(&buffer).into_owned())
                    .expect("send body");

                if reply.delay_ms > 0 {
                    thread::sleep(Duration::from_millis(reply.delay_ms));
                }

                let http = format!(
                    "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    reply.status_line,
                    reply.body.len(),
                    reply.body
                );
                stream.write_all(http.as_bytes()).expect("write response");
            }
        });

        (format!("http://{}", addr), rx, handle)
    }

    fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack
            .windows(needle.len())
            .position(|window| window == needle)
    }

    #[test]
    fn provider_returns_structured_candidate() {
        let provider = OpenAiProvider::new("gpt-4.1-mini".to_string());
        let mut cfg = RuntimeConfig::default_for_dev();
        cfg.openai.mock_mode = true;

        let out = provider
            .generate_recap(&sample_request(), &cfg)
            .expect("candidate");
        assert!(!out.summary.is_empty());
        assert!(out.next_steps.len() >= 3);
    }

    #[test]
    fn live_mode_requires_env_api_key() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        let provider = OpenAiProvider::new("gpt-4.1-mini".to_string());
        let mut cfg = RuntimeConfig::default_for_dev();
        cfg.openai.mock_mode = false;

        unsafe {
            std::env::remove_var("OPENAI_API_KEY");
        }

        let err = provider
            .generate_recap(&sample_request(), &cfg)
            .expect_err("missing key");
        match err {
            ProviderError::Unavailable(message) => {
                assert!(message.contains("OPENAI_API_KEY"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn live_request_uses_responses_api_and_parses_success() {
        let provider = OpenAiProvider::new("gpt-4.1-mini".to_string());
        let response = TestResponse {
            status_line: "200 OK",
            body: response_payload("You returned to Whiterun after meeting the jarl."),
            delay_ms: 0,
        };
        let (base_url, rx, handle) = spawn_test_server(vec![response]);

        let output = provider
            .send_live_request(&base_url, "test-key", "Prompt body", 2_000)
            .expect("live output");
        let request = rx.recv().expect("captured request");
        handle.join().expect("server join");

        assert!(request.starts_with("POST /responses HTTP/1.1"));
        assert!(request.contains("\"model\":\"gpt-4.1-mini\""));
        assert!(request.contains("\"type\":\"json_schema\""));
        assert!(request.contains("\"name\":\"skyrim_recap\""));
        assert_eq!(output.provider_model, "gpt-4.1-mini-2026-03-01");
        assert_eq!(output.tokens_in, Some(91));
        assert_eq!(output.tokens_out, Some(53));
        assert!(output.raw_json.contains("\"summary\""));
    }

    #[test]
    fn live_path_retries_retryable_failures() {
        let provider = OpenAiProvider::new("gpt-4.1-mini".to_string());
        let mut cfg = RuntimeConfig::default_for_dev();
        cfg.openai.mock_mode = false;
        cfg.openai.max_retries = 1;
        cfg.openai.timeout_ms = 2_000;

        let responses = vec![
            TestResponse {
                status_line: "500 Internal Server Error",
                body: "{\"error\":\"temporary\"}".to_string(),
                delay_ms: 0,
            },
            TestResponse {
                status_line: "200 OK",
                body: response_payload("The retry produced a valid recap."),
                delay_ms: 0,
            },
        ];
        let (base_url, rx, handle) = spawn_test_server(responses);

        let candidate = provider
            .generate_live_recap(&sample_request(), &cfg, &base_url, "test-key")
            .expect("retry success");

        let first = rx.recv().expect("first request");
        let second = rx.recv().expect("second request");
        handle.join().expect("server join");

        assert!(first.starts_with("POST /responses HTTP/1.1"));
        assert!(second.starts_with("POST /responses HTTP/1.1"));
        assert!(candidate.summary.contains("retry produced"));
    }

    #[test]
    fn live_path_surfaces_timeouts_as_provider_errors() {
        let provider = OpenAiProvider::new("gpt-4.1-mini".to_string());
        let mut cfg = RuntimeConfig::default_for_dev();
        cfg.openai.mock_mode = false;
        cfg.openai.max_retries = 0;
        cfg.openai.timeout_ms = 50;

        let responses = vec![TestResponse {
            status_line: "200 OK",
            body: response_payload("This response arrives too late."),
            delay_ms: 250,
        }];
        let (base_url, _rx, handle) = spawn_test_server(responses);

        let err = provider
            .generate_live_recap(&sample_request(), &cfg, &base_url, "test-key")
            .expect_err("timeout");
        handle.join().expect("server join");

        match err {
            ProviderError::Unavailable(message) => {
                assert!(message.contains("live provider request failed"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
