use runtime_core::contracts::{RecapRequestV1, RecapResponseV1};
use runtime_core::traits::{Transport, TransportEnvelope, TransportError};
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct FileTransport {
    requests_dir: PathBuf,
    responses_dir: PathBuf,
}

impl FileTransport {
    pub fn new(requests_dir: impl Into<PathBuf>, responses_dir: impl Into<PathBuf>) -> Result<Self, TransportError> {
        let requests_dir = requests_dir.into();
        let responses_dir = responses_dir.into();
        fs::create_dir_all(&requests_dir)
            .map_err(|e| TransportError::Io(format!("create requests dir: {e}")))?;
        fs::create_dir_all(&responses_dir)
            .map_err(|e| TransportError::Io(format!("create responses dir: {e}")))?;
        Ok(Self {
            requests_dir,
            responses_dir,
        })
    }

    pub fn write_request_file(&self, request: &RecapRequestV1) -> Result<PathBuf, TransportError> {
        let path = self
            .requests_dir
            .join(format!("{}.json", request.request_id));
        let body = serde_json::to_string_pretty(request)
            .map_err(|e| TransportError::Encode(e.to_string()))?;
        fs::write(&path, body).map_err(|e| TransportError::Io(e.to_string()))?;
        Ok(path)
    }

    pub fn wait_for_response(
        &self,
        request_id: &str,
        timeout: Duration,
    ) -> Result<Option<RecapResponseV1>, TransportError> {
        let path = self.responses_dir.join(format!("{}.json", request_id));
        let start = Instant::now();

        while start.elapsed() < timeout {
            if path.exists() {
                let body = fs::read_to_string(&path).map_err(|e| TransportError::Io(e.to_string()))?;
                let response = serde_json::from_str::<RecapResponseV1>(&body)
                    .map_err(|e| TransportError::Decode(e.to_string()))?;
                return Ok(Some(response));
            }
            thread::sleep(Duration::from_millis(120));
        }

        Ok(None)
    }

    fn next_request_path(&self) -> Result<Option<PathBuf>, TransportError> {
        let mut files = fs::read_dir(&self.requests_dir)
            .map_err(|e| TransportError::Io(format!("read requests dir: {e}")))?
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("json"))
            .collect::<Vec<_>>();

        files.sort();
        Ok(files.into_iter().next())
    }
}

impl Transport for FileTransport {
    fn pop_next_request(&mut self, wait_for: Duration) -> Result<Option<TransportEnvelope>, TransportError> {
        let start = Instant::now();
        while start.elapsed() < wait_for {
            if let Some(request_path) = self.next_request_path()? {
                let body = fs::read_to_string(&request_path)
                    .map_err(|e| TransportError::Io(format!("read request file: {e}")))?;
                let request = serde_json::from_str::<RecapRequestV1>(&body)
                    .map_err(|e| TransportError::Decode(format!("decode request: {e}")))?;
                let response_path = self
                    .responses_dir
                    .join(request_path.file_name().ok_or_else(|| {
                        TransportError::Io("request file has no file_name".to_string())
                    })?);
                return Ok(Some(TransportEnvelope {
                    request_path,
                    response_path,
                    request,
                }));
            }
            thread::sleep(Duration::from_millis(120));
        }

        Ok(None)
    }

    fn write_response(
        &mut self,
        envelope: &TransportEnvelope,
        response: &RecapResponseV1,
    ) -> Result<(), TransportError> {
        let temp_path = envelope.response_path.with_extension("tmp");
        let body = serde_json::to_string_pretty(response)
            .map_err(|e| TransportError::Encode(format!("encode response: {e}")))?;
        fs::write(&temp_path, body)
            .map_err(|e| TransportError::Io(format!("write response temp: {e}")))?;
        fs::rename(&temp_path, &envelope.response_path)
            .map_err(|e| TransportError::Io(format!("rename response temp: {e}")))?;
        fs::remove_file(&envelope.request_path)
            .map_err(|e| TransportError::Io(format!("remove request file: {e}")))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::FileTransport;
    use runtime_core::contracts::{
        BudgetsMeta, CacheMeta, ClientInfo, ClientProfile, EventEntryV1, GameContext, ProviderMeta,
        RecapPayload, RecapRequestV1, RecapResponseV1, ResponseMeta, SpoilerMode, SpoilerRisk,
        RECAP_REQUEST_VERSION,
    };
    use runtime_core::traits::Transport;
    use tempfile::tempdir;

    #[test]
    fn transport_roundtrip_reads_request_and_writes_response() {
        let tmp = tempdir().expect("tmp");
        let requests = tmp.path().join("requests");
        let responses = tmp.path().join("responses");

        let mut transport = FileTransport::new(&requests, &responses).expect("transport");

        let req = RecapRequestV1 {
            contract_version: RECAP_REQUEST_VERSION.to_string(),
            request_id: "r1".to_string(),
            feature: "recap".to_string(),
            created_at_utc: chrono::Utc::now(),
            spoiler_mode: SpoilerMode::Safe,
            game_context: GameContext {
                game_time: None,
                player_location: "Riverwood".to_string(),
                event_log: vec![EventEntryV1 {
                    t: "now".to_string(),
                    kind: "note".to_string(),
                    text: "arrived".to_string(),
                }],
            },
            client: ClientInfo {
                client_kind: "skyrim-mod".to_string(),
                client_version: "0.1.0".to_string(),
                profile: ClientProfile::Developer,
            },
        };
        transport.write_request_file(&req).expect("write req");

        let envelope = transport
            .pop_next_request(std::time::Duration::from_secs(1))
            .expect("pop")
            .expect("some");

        let response = RecapResponseV1::success(
            &req.request_id,
            RecapPayload {
                summary: "x".to_string(),
                next_steps: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                spoiler_risk: SpoilerRisk::None,
            },
            ResponseMeta {
                runtime_build_id: "dev".to_string(),
                prompt_version: "v1".to_string(),
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
                    tokens_in: Some(10),
                    tokens_out: Some(20),
                },
                created_at_utc: chrono::Utc::now(),
            },
        );

        transport
            .write_response(&envelope, &response)
            .expect("write response");

        let found = transport
            .wait_for_response(&req.request_id, std::time::Duration::from_secs(1))
            .expect("wait")
            .expect("response file");
        assert!(found.ok);
    }
}
