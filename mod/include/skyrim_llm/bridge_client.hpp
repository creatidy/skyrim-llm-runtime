#pragma once

#include "skyrim_llm/contracts.hpp"

#include <chrono>
#include <optional>
#include <string>

namespace skyrim_llm {

class BridgeClient {
public:
    explicit BridgeClient(BridgePaths paths);

    std::string WriteRequest(const RecapRequest& request) const;
    std::optional<RecapResponse> WaitForResponse(
        const std::string& request_id,
        std::chrono::milliseconds timeout,
        std::chrono::milliseconds poll_interval) const;

private:
    BridgePaths paths_;
};

}  // namespace skyrim_llm
