use crate::cache::CachedRecap;
use crate::config::RuntimeConfig;
use crate::contracts::{
    BudgetsMeta, CacheMeta, ProviderMeta, RecapRequestV1, RecapResponseV1, ResponseMeta,
    RuntimeErrorCode,
};
use crate::observability::{Metrics, StructuredLogger};
use crate::traits::{CacheStore, Provider, ReplayStore, SafetyPipeline};
use chrono::Utc;
use serde_json::json;

pub struct RecapService<P, C, S, R>
where
    P: Provider,
    C: CacheStore,
    S: SafetyPipeline,
    R: ReplayStore,
{
    // Concrete dependencies are generic traits to keep provider/transport replaceable.
    pub config: RuntimeConfig,
    pub provider: P,
    pub cache: C,
    pub safety: S,
    pub replay: R,
    pub metrics: Metrics,
    pub logger: StructuredLogger,
}

impl<P, C, S, R> RecapService<P, C, S, R>
where
    P: Provider,
    C: CacheStore,
    S: SafetyPipeline,
    R: ReplayStore,
{
    pub fn process_request(&mut self, request: RecapRequestV1) -> RecapResponseV1 {
        self.metrics.request_count += 1;
        let provider_mode = if self.config.openai.mock_mode {
            "mock"
        } else {
            "live"
        };

        // 1) Contract sanity checks. Fail fast on invalid requests.
        if let Err(err) = request.validate_basic() {
            self.metrics.validation_failures += 1;
            return self.failure_response(
                &request.request_id,
                RuntimeErrorCode::ValidationFailed,
                &err,
                CacheMeta {
                    hit: false,
                    key: None,
                    age_seconds: None,
                },
                None,
            );
        }

        // 2) Normalize/redact before hashing/calling provider.
        let sanitized = self.safety.sanitize_request(&request);
        let cache_key = self.cache.stable_key_for(
            &sanitized,
            &self.config.prompt_version,
            self.provider.model(),
        );

        // 3) Fast path: return fresh cache hit when available.
        if self.config.caching.enabled {
            match self.cache.get_fresh(&cache_key) {
                Ok(Some((cached, age_seconds))) => {
                    self.metrics.cache_hits += 1;
                    let _ = self.logger.info(
                        "cache_hit",
                        &request.request_id,
                        json!({ "cache_key": cache_key, "age_seconds": age_seconds }),
                    );
                    return self.success_response(
                        &request.request_id,
                        cached,
                        true,
                        Some(cache_key),
                        Some(age_seconds),
                    );
                }
                Ok(None) => {
                    self.metrics.cache_misses += 1;
                }
                Err(err) => {
                    let _ = self.logger.error(
                        "cache_read_error",
                        &request.request_id,
                        json!({ "error": err.to_string() }),
                    );
                }
            }
        }

        // 4) Local budget guard before external provider work.
        let token_estimate = estimate_tokens_from_request(&sanitized);
        if token_estimate > self.config.budgets.max_tokens_per_call {
            return self.failure_response(
                &request.request_id,
                RuntimeErrorCode::BudgetExceeded,
                "request exceeds max_tokens_per_call budget",
                CacheMeta {
                    hit: false,
                    key: Some(cache_key),
                    age_seconds: None,
                },
                None,
            );
        }

        // 5) Provider call with stale-cache/fallback handling.
        let candidate = match self.provider.generate_recap(&sanitized, &self.config) {
            Ok(candidate) => candidate,
            Err(err) => {
                self.metrics.provider_errors += 1;
                let _ = self.logger.error(
                    "provider_error",
                    &request.request_id,
                    json!({
                        "error": err.to_string(),
                        "provider_mode": provider_mode,
                    }),
                );
                if let Ok(Some(stale)) = self.cache.get_stale(&cache_key) {
                    return self.success_response(
                        &request.request_id,
                        stale,
                        true,
                        Some(cache_key),
                        None,
                    );
                }
                self.metrics.fallback_count += 1;
                let fallback_cached = CachedRecap {
                    recap: self.safety.fallback_recap(),
                    provider: ProviderMeta {
                        name: self.provider.name().to_string(),
                        model: self.provider.model().to_string(),
                    },
                    budgets: BudgetsMeta {
                        tokens_in: None,
                        tokens_out: None,
                    },
                    created_at_utc: Utc::now(),
                };
                let response = self.success_response(
                    &request.request_id,
                    fallback_cached,
                    false,
                    Some(cache_key.clone()),
                    None,
                );
                let _ = self.replay.export_bundle(&sanitized, &response, None);
                return response;
            }
        };

        // 6) Enforce output constraints and spoiler policy.
        let recap_payload = match self
            .safety
            .validate_candidate(&sanitized, candidate.clone())
        {
            Ok(payload) => payload,
            Err(err) => {
                self.metrics.validation_failures += 1;
                if let Ok(Some(stale)) = self.cache.get_stale(&cache_key) {
                    return self.success_response(
                        &request.request_id,
                        stale,
                        true,
                        Some(cache_key),
                        None,
                    );
                }
                self.metrics.fallback_count += 1;
                let fallback_cached = CachedRecap {
                    recap: self.safety.fallback_recap(),
                    provider: ProviderMeta {
                        name: self.provider.name().to_string(),
                        model: self.provider.model().to_string(),
                    },
                    budgets: BudgetsMeta {
                        tokens_in: candidate.tokens_in,
                        tokens_out: candidate.tokens_out,
                    },
                    created_at_utc: Utc::now(),
                };
                let response = self.success_response(
                    &request.request_id,
                    fallback_cached,
                    false,
                    Some(cache_key.clone()),
                    None,
                );
                let _ = self.logger.error(
                    "validation_failed_fallback",
                    &request.request_id,
                    json!({ "error": err }),
                );
                let _ = self.replay.export_bundle(&sanitized, &response, None);
                return response;
            }
        };

        // 7) Persist successful output in cache and optionally replay bundle.
        let cached = CachedRecap {
            recap: recap_payload,
            provider: ProviderMeta {
                name: candidate.provider_name,
                model: candidate.provider_model,
            },
            budgets: BudgetsMeta {
                tokens_in: candidate.tokens_in,
                tokens_out: candidate.tokens_out,
            },
            created_at_utc: Utc::now(),
        };

        if self.config.caching.enabled {
            let _ = self.cache.set(&cache_key, &cached);
        }

        let _ = self.logger.info(
            "provider_response",
            &request.request_id,
            json!({
                "provider_mode": provider_mode,
                "provider_model": cached.provider.model,
                "tokens_in": cached.budgets.tokens_in,
                "tokens_out": cached.budgets.tokens_out,
            }),
        );

        let response =
            self.success_response(&request.request_id, cached, false, Some(cache_key), Some(0));

        if self.config.replay.export_enabled {
            let _ = self.replay.export_bundle(&sanitized, &response, None);
        }

        response
    }

    fn success_response(
        &self,
        request_id: &str,
        cached: CachedRecap,
        cache_hit: bool,
        cache_key: Option<String>,
        age_seconds: Option<u64>,
    ) -> RecapResponseV1 {
        let meta = ResponseMeta {
            runtime_build_id: self.config.runtime_build_id.clone(),
            prompt_version: self.config.prompt_version.clone(),
            provider: cached.provider,
            cache: CacheMeta {
                hit: cache_hit,
                key: cache_key,
                age_seconds,
            },
            budgets: cached.budgets,
            created_at_utc: Utc::now(),
        };

        RecapResponseV1::success(request_id, cached.recap, meta)
    }

    fn failure_response(
        &self,
        request_id: &str,
        code: RuntimeErrorCode,
        message: &str,
        cache: CacheMeta,
        provider: Option<ProviderMeta>,
    ) -> RecapResponseV1 {
        let meta = ResponseMeta {
            runtime_build_id: self.config.runtime_build_id.clone(),
            prompt_version: self.config.prompt_version.clone(),
            provider: provider.unwrap_or(ProviderMeta {
                name: self.provider.name().to_string(),
                model: self.provider.model().to_string(),
            }),
            cache,
            budgets: BudgetsMeta {
                tokens_in: None,
                tokens_out: None,
            },
            created_at_utc: Utc::now(),
        };
        RecapResponseV1::failure(request_id, code, message, meta)
    }
}

fn estimate_tokens_from_request(request: &RecapRequestV1) -> u32 {
    // Cheap approximation good enough for PoC budget checks.
    let mut chars = request.game_context.player_location.chars().count() as u32;
    for event in &request.game_context.event_log {
        chars += event.text.chars().count() as u32;
    }
    chars / 4 + 32
}
