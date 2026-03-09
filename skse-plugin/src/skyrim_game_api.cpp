#include "skyrim_llm_skse_host/skyrim_game_api.hpp"

#include <utility>

namespace skyrim_llm::skse_host {

std::optional<SnapshotData> SkyrimGameApi::CaptureSnapshot() {
    if (snapshot_override_.has_value()) {
        return snapshot_override_;
    }

    const auto player_location = location_capture_.CapturePlayerLocation();
    if (!player_location.has_value() || player_location->empty()) {
        return std::nullopt;
    }

    return SnapshotData{
        .game_time = game_time_capture_.CaptureGameTime(),
        .player_location = player_location.value(),
        .current_objective = CaptureCurrentObjective(),
    };
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

std::optional<std::string> SkyrimGameApi::CaptureCurrentObjective() const {
    // TODO(skyrim-phase2): Read the currently displayed objective text if it is
    // cheaply available. This is optional for the first real smoke pass.
    return std::nullopt;
}

}  // namespace skyrim_llm::skse_host
