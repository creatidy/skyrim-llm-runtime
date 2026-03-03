use crate::contracts::{
    default_fallback_recap, CandidateRecap, RecapPayload, RecapRequestV1, SpoilerMode, SpoilerRisk,
};
use crate::traits::SafetyPipeline;

#[derive(Debug, Clone)]
pub struct DefaultSafetyPipeline {
    // Hard caps keep requests/outputs bounded and UI-friendly.
    redaction_enabled: bool,
    max_events: usize,
    max_chars_per_event: usize,
    max_summary_chars: usize,
    max_next_step_chars: usize,
}

impl DefaultSafetyPipeline {
    pub fn new(redaction_enabled: bool) -> Self {
        Self {
            redaction_enabled,
            max_events: 80,
            max_chars_per_event: 280,
            max_summary_chars: 420,
            max_next_step_chars: 180,
        }
    }

    fn redact_text(&self, s: &str) -> String {
        if !self.redaction_enabled {
            return s.to_string();
        }

        // PoC redaction patterns; keep simple and deterministic.
        let mut out = s.replace("C:\\Users\\", "<redacted-user-path>\\");
        out = out.replace("C:\\\\Users\\\\", "<redacted-user-path>\\");
        out = out.replace("/home/", "<redacted-home>/");
        out = out.replace("\\\\", "\\");

        for marker in ["DESKTOP-", "LAPTOP-"] {
            if let Some(idx) = out.find(marker) {
                let end = out[idx..]
                    .find(' ')
                    .map(|offset| idx + offset)
                    .unwrap_or(out.len());
                out.replace_range(idx..end, "<redacted-machine>");
            }
        }

        out
    }

    fn scrub_spoilers(&self, text: &str) -> String {
        // Very simple spoiler scrubber for PoC safe mode.
        let mut out = text.to_string();
        for token in ["Alduin", "Sovngarde", "Dragonborn"] {
            out = out.replace(token, "[spoiler-redacted]");
        }
        out
    }
}

impl SafetyPipeline for DefaultSafetyPipeline {
    fn sanitize_request(&self, request: &RecapRequestV1) -> RecapRequestV1 {
        let mut cloned = request.clone();

        // Keep only recent tail to cap payload size.
        if cloned.game_context.event_log.len() > self.max_events {
            let start = cloned.game_context.event_log.len() - self.max_events;
            cloned.game_context.event_log = cloned.game_context.event_log.split_off(start);
        }

        cloned.game_context.player_location =
            self.redact_text(&cloned.game_context.player_location);

        for event in &mut cloned.game_context.event_log {
            let text = self.redact_text(&event.text);
            let clipped = text
                .chars()
                .take(self.max_chars_per_event)
                .collect::<String>();
            event.text = clipped;
        }

        cloned
    }

    fn validate_candidate(
        &self,
        request: &RecapRequestV1,
        candidate: CandidateRecap,
    ) -> Result<RecapPayload, String> {
        // Validate presence and length of summary.
        let mut summary = candidate.summary.trim().to_string();
        if summary.is_empty() {
            return Err("summary is empty".to_string());
        }
        if summary.chars().count() > self.max_summary_chars {
            summary = summary.chars().take(self.max_summary_chars).collect();
        }

        // Normalize and bound next-step list for in-game UI.
        let mut next_steps = candidate
            .next_steps
            .into_iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        if next_steps.len() < 3 {
            return Err("next_steps must contain at least 3 items".to_string());
        }

        if next_steps.len() > 5 {
            next_steps.truncate(5);
        }

        for step in &mut next_steps {
            if step.chars().count() > self.max_next_step_chars {
                *step = step.chars().take(self.max_next_step_chars).collect();
            }
        }

        let spoiler_risk = match request.spoiler_mode {
            SpoilerMode::Safe => {
                // In safe mode, scrub known spoiler tokens from output text.
                summary = self.scrub_spoilers(&summary);
                next_steps = next_steps
                    .into_iter()
                    .map(|s| self.scrub_spoilers(&s))
                    .collect();
                SpoilerRisk::None
            }
            SpoilerMode::Full => SpoilerRisk::Low,
        };

        Ok(RecapPayload {
            summary,
            next_steps,
            spoiler_risk,
        })
    }

    fn fallback_recap(&self) -> RecapPayload {
        default_fallback_recap()
    }
}
