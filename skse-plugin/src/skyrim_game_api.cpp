#include "skyrim_llm_skse_host/skyrim_game_api.hpp"

#include <utility>

namespace skyrim_llm::skse_host {

std::optional<SnapshotData> SkyrimGameApi::CaptureSnapshot() {
    return snapshot_override_;
}

void SkyrimGameApi::SetDebugSnapshot(
    std::optional<std::string> game_time,
    std::string player_location,
    std::optional<std::string> current_objective) {
    snapshot_override_ = SnapshotData{
        .game_time = std::move(game_time),
        .player_location = std::move(player_location),
        .current_objective = std::move(current_objective),
    };
}

void SkyrimGameApi::ClearDebugSnapshot() {
    snapshot_override_.reset();
}

}  // namespace skyrim_llm::skse_host
