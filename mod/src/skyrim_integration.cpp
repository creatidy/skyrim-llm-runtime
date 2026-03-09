#include "skyrim_llm/skyrim_integration.hpp"

#include <sstream>
#include <utility>

namespace skyrim_llm {

SkyrimUiPresenter::SkyrimUiPresenter(NotificationSink& sink, std::string title)
    : sink_(sink), title_(std::move(title)) {}

void SkyrimUiPresenter::ShowStatus(const std::string& text) {
    sink_.ShowStatusLine(text);
}

void SkyrimUiPresenter::ShowRecap(const RecapPayload& recap, bool cache_hit) {
    std::ostringstream body;
    body << recap.summary;

    if (!recap.next_steps.empty()) {
        body << "\n\nNext steps:";
        for (std::size_t i = 0; i < recap.next_steps.size(); ++i) {
            body << "\n" << (i + 1) << ". " << recap.next_steps[i];
        }
    }

    body << "\n\nResult: " << (cache_hit ? "cached" : "fresh");
    sink_.ShowMessage(title_, body.str());
}

void SkyrimUiPresenter::ShowError(const std::string& text) {
    sink_.ShowStatusLine(text);
}

SkyrimModIntegration::SkyrimModIntegration(
    IntegrationConfig config,
    GameSnapshotProvider& snapshot_provider,
    NotificationSink& notification_sink)
    : snapshot_provider_(snapshot_provider),
      notification_sink_(notification_sink),
      ui_(notification_sink_, std::move(config.plugin_title)),
      stub_(std::move(config.bridge_paths)),
      recap_timeout_(config.recap_timeout) {}

void SkyrimModIntegration::RecordLocationChange(const std::string& game_time, const std::string& location) {
    stub_.RecordLocationChange(game_time, location);
}

void SkyrimModIntegration::RecordQuestObjective(const std::string& game_time, const std::string& objective) {
    stub_.RecordQuestObjective(game_time, objective);
}

void SkyrimModIntegration::RecordNote(const std::string& game_time, const std::string& note) {
    stub_.RecordNote(game_time, note);
}

bool SkyrimModIntegration::TriggerHotkeyRecap() {
    const auto snapshot = snapshot_provider_.CaptureSnapshot();
    if (!snapshot.has_value()) {
        notification_sink_.ShowStatusLine("Unable to capture Skyrim state");
        return false;
    }

    stub_.OnHotkeyPressed(snapshot.value(), ui_, recap_timeout_);
    return true;
}

}  // namespace skyrim_llm
