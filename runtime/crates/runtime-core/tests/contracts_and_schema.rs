use jsonschema::JSONSchema;
use runtime_core::contracts::{
    BudgetsMeta, CacheMeta, ClientInfo, ClientProfile, EventEntryV1, GameContext, ProviderMeta,
    RecapPayload, RecapRequestV1, RecapResponseV1, ResponseMeta, SpoilerMode, SpoilerRisk,
    RECAP_REQUEST_VERSION,
};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn sample_request() -> RecapRequestV1 {
    RecapRequestV1 {
        contract_version: RECAP_REQUEST_VERSION.to_string(),
        request_id: "req-1".to_string(),
        feature: "recap".to_string(),
        created_at_utc: chrono::Utc::now(),
        spoiler_mode: SpoilerMode::Safe,
        game_context: GameContext {
            game_time: Some("4E 201, 10:10".to_string()),
            player_location: "Whiterun".to_string(),
            event_log: vec![EventEntryV1 {
                t: "10:00".to_string(),
                kind: "quest".to_string(),
                text: "Returned the Dragonstone".to_string(),
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
fn request_and_response_roundtrip_json() {
    let req = sample_request();
    let req_json = serde_json::to_string(&req).expect("serialize request");
    let req_back: RecapRequestV1 = serde_json::from_str(&req_json).expect("deserialize request");
    assert_eq!(req.feature, req_back.feature);

    let resp = RecapResponseV1::success(
        &req.request_id,
        RecapPayload {
            summary: "Short recap".to_string(),
            next_steps: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            spoiler_risk: SpoilerRisk::None,
        },
        ResponseMeta {
            runtime_build_id: "dev-build".to_string(),
            prompt_version: "recap-v1".to_string(),
            provider: ProviderMeta {
                name: "openai".to_string(),
                model: "gpt-4.1-mini".to_string(),
            },
            cache: CacheMeta {
                hit: false,
                key: None,
                age_seconds: None,
            },
            budgets: BudgetsMeta {
                tokens_in: Some(100),
                tokens_out: Some(60),
            },
            created_at_utc: chrono::Utc::now(),
        },
    );

    let resp_json = serde_json::to_string(&resp).expect("serialize response");
    let resp_back: RecapResponseV1 =
        serde_json::from_str(&resp_json).expect("deserialize response");
    assert!(resp_back.ok);
}

#[test]
fn request_schema_validation_and_versioning() {
    let req = sample_request();
    let req_value = serde_json::to_value(&req).expect("value");

    let schema_body =
        fs::read_to_string(workspace_root().join("contracts/recap-request-v1.schema.json"))
            .expect("load request schema");
    let schema_json: Value = serde_json::from_str(&schema_body).expect("schema json");
    let compiled = JSONSchema::compile(&schema_json).expect("compile schema");

    assert!(compiled.validate(&req_value).is_ok());

    let mut bad = req_value;
    bad["contract_version"] = Value::String("v2".to_string());
    assert!(compiled.validate(&bad).is_err());
}

#[test]
fn response_schema_validation() {
    let req = sample_request();
    let resp = RecapResponseV1::success(
        &req.request_id,
        RecapPayload {
            summary: "Short recap".to_string(),
            next_steps: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            spoiler_risk: SpoilerRisk::None,
        },
        ResponseMeta {
            runtime_build_id: "dev-build".to_string(),
            prompt_version: "recap-v1".to_string(),
            provider: ProviderMeta {
                name: "openai".to_string(),
                model: "gpt-4.1-mini".to_string(),
            },
            cache: CacheMeta {
                hit: false,
                key: Some("abc".to_string()),
                age_seconds: Some(0),
            },
            budgets: BudgetsMeta {
                tokens_in: Some(100),
                tokens_out: Some(60),
            },
            created_at_utc: chrono::Utc::now(),
        },
    );

    let value = serde_json::to_value(resp).expect("to value");
    let schema_body =
        fs::read_to_string(workspace_root().join("contracts/recap-response-v1.schema.json"))
            .expect("load response schema");
    let schema_json: Value = serde_json::from_str(&schema_body).expect("schema json");
    let compiled = JSONSchema::compile(&schema_json).expect("compile schema");
    assert!(compiled.validate(&value).is_ok());
}
