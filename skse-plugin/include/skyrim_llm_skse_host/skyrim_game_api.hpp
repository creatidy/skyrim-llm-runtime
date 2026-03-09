#pragma once

#include "skyrim_llm_skse_host/game_api.hpp"

#include <optional>
#include <string>

namespace skyrim_llm::skse_host {

class SkyrimGameApi final : public GameApi {
public:
    SkyrimGameApi() = default;

    std::optional<SnapshotData> CaptureSnapshot() override;

    void SetDebugSnapshot(
        std::optional<std::string> game_time,
        std::string player_location,
        std::optional<std::string> current_objective = std::nullopt);
    void ClearDebugSnapshot();

private:
    std::optional<SnapshotData> snapshot_override_;
};

}  // namespace skyrim_llm::skse_host
