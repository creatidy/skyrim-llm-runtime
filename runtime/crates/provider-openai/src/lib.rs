use chrono::Utc;
use runtime_core::config::RuntimeConfig;
use runtime_core::contracts::{CandidateRecap, RecapRequestV1};
use runtime_core::traits::{Provider, ProviderError};
use serde::Deserialize;
use serde_json::json;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct OpenAiProvider {
    model: String,
}

impl OpenAiProvider {
    pub fn new(model: String) -> Self {
        Self { model }
    }

    fn build_prompt(request: &RecapRequestV1) -> String {
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

    fn generate_mock_json(request: &RecapRequestV1) -> Result<String, ProviderError> {
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
            return Ok(json!({
                "summary": "",
                "next_steps": ["Only one step"]
            })
            .to_string());
        }

        let location = &request.game_context.player_location;
        let most_recent = request
            .game_context
            .event_log
            .last()
            .map(|e| e.text.clone())
            .unwrap_or_else(|| "No recent events.".to_string());

        Ok(json!({
            "summary": format!("You last progressed near {}. Recent notable action: {}", location, most_recent),
            "next_steps": [
                "Check your active quest objective and map marker.",
                "Restock potions and repair gear before deeper exploration.",
                "Set a short-term goal for your next fast-travel stop."
            ]
        })
        .to_string())
    }

    fn parse_structured_output(
        model: &str,
        raw: &str,
        estimated_tokens_in: u32,
    ) -> Result<CandidateRecap, ProviderError> {
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
            provider_model: model.to_string(),
            tokens_in: Some(estimated_tokens_in),
            tokens_out: Some((raw.len() as u32 / 4).max(16)),
        })
    }

    fn estimate_tokens(prompt: &str) -> u32 {
        (prompt.len() as u32 / 4).max(32)
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
        let prompt = Self::build_prompt(request);
        let tokens_in = Self::estimate_tokens(&prompt);

        let mut last_err: Option<ProviderError> = None;
        for attempt in 0..=config.openai.max_retries {
            let raw = if config.openai.mock_mode || config.openai.api_key.is_none() {
                Self::generate_mock_json(request)
            } else {
                Err(ProviderError::Unavailable(
                    "real OpenAI HTTP path is disabled in this offline PoC build; use mock_mode or add HTTP adapter".to_string(),
                ))
            };

            match raw {
                Ok(body) => {
                    return Self::parse_structured_output(&self.model, &body, tokens_in);
                }
                Err(err) => {
                    last_err = Some(err);
                    if attempt < config.openai.max_retries {
                        thread::sleep(Duration::from_millis((attempt as u64 + 1) * 80));
                    }
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

#[cfg(test)]
mod tests {
    use super::OpenAiProvider;
    use runtime_core::config::RuntimeConfig;
    use runtime_core::contracts::{
        ClientInfo, ClientProfile, EventEntryV1, GameContext, RecapRequestV1, SpoilerMode,
        RECAP_REQUEST_VERSION,
    };
    use runtime_core::traits::Provider;

    #[test]
    fn provider_returns_structured_candidate() {
        let provider = OpenAiProvider::new("gpt-4.1-mini".to_string());
        let mut cfg = RuntimeConfig::default_for_dev();
        cfg.openai.mock_mode = true;

        let req = RecapRequestV1 {
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
        };

        let out = provider.generate_recap(&req, &cfg).expect("candidate");
        assert!(!out.summary.is_empty());
        assert!(out.next_steps.len() >= 3);
    }
}
