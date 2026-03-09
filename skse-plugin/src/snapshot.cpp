#include "skyrim_llm_skse_host/workflow.hpp"

namespace skyrim_llm::skse_host {

void SeedInitialEventLog(HostContext& host, GameApi& game_api) {
    const auto snapshot = game_api.CaptureSnapshot();
    if (!snapshot.has_value()) {
        return;
    }

    const auto game_time = snapshot->game_time.value_or("unknown");
    if (!snapshot->player_location.empty()) {
        host.RecordLocationChange(game_time, snapshot->player_location);
    }
    if (snapshot->current_objective.has_value() && !snapshot->current_objective->empty()) {
        host.RecordQuestObjective(game_time, snapshot->current_objective.value());
    }
}

}  // namespace skyrim_llm::skse_host
