#pragma once

#include <optional>
#include <string>

namespace skyrim_llm::skse_host {

struct SnapshotData {
    std::optional<std::string> game_time;
    std::string player_location;
    std::optional<std::string> current_objective;
};

class GameApi {
public:
    virtual ~GameApi() = default;

    virtual std::optional<SnapshotData> CaptureSnapshot() = 0;
};

}  // namespace skyrim_llm::skse_host
