use crate::contracts::{RecapRequestV1, RecapResponseV1};
use crate::traits::ReplayStore;
use chrono::Utc;
use serde_json::json;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct FileReplayStore {
    pub directory: PathBuf,
}

impl FileReplayStore {
    pub fn new(directory: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let directory = directory.into();
        fs::create_dir_all(&directory)?;
        Ok(Self { directory })
    }

    pub fn replay_bundle_summary(bundle_dir: impl Into<PathBuf>) -> anyhow::Result<String> {
        let dir = bundle_dir.into();
        let request = fs::read_to_string(dir.join("request.json"))?;
        let response = fs::read_to_string(dir.join("response.json"))?;
        let req: RecapRequestV1 = serde_json::from_str(&request)?;
        let res: RecapResponseV1 = serde_json::from_str(&response)?;

        let summary = if let Some(recap) = res.recap {
            recap.summary
        } else {
            "no recap in response".to_string()
        };

        Ok(format!(
            "request_id={} feature={} ok={} summary={}",
            req.request_id, req.feature, res.ok, summary
        ))
    }
}

impl ReplayStore for FileReplayStore {
    fn export_bundle(
        &self,
        request: &RecapRequestV1,
        response: &RecapResponseV1,
        redaction_report: Option<&str>,
    ) -> anyhow::Result<()> {
        let bundle_dir = self.directory.join(&request.request_id);
        fs::create_dir_all(&bundle_dir)?;

        fs::write(
            bundle_dir.join("request.json"),
            serde_json::to_string_pretty(request)?,
        )?;
        fs::write(
            bundle_dir.join("response.json"),
            serde_json::to_string_pretty(response)?,
        )?;
        fs::write(
            bundle_dir.join("runtime_build_id.txt"),
            response.meta.runtime_build_id.as_bytes(),
        )?;
        fs::write(
            bundle_dir.join("prompt_version.txt"),
            response.meta.prompt_version.as_bytes(),
        )?;
        fs::write(
            bundle_dir.join("provider.json"),
            serde_json::to_string_pretty(&response.meta.provider)?,
        )?;
        fs::write(
            bundle_dir.join("timestamps.json"),
            serde_json::to_string_pretty(&json!({
                "request_created_at_utc": request.created_at_utc,
                "response_created_at_utc": response.meta.created_at_utc,
                "bundle_exported_at_utc": Utc::now(),
            }))?,
        )?;

        if let Some(report) = redaction_report {
            fs::write(bundle_dir.join("redaction_report.json"), report)?;
        }

        fs::write(
            bundle_dir.join("notes.txt"),
            b"Replay bundle exported in redacted mode by default.\n",
        )?;

        Ok(())
    }
}
