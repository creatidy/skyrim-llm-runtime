#include "skyrim_llm_skse_host/game_time_capture.hpp"

namespace skyrim_llm::skse_host {

std::optional<std::string> GameTimeCapture::CaptureGameTime() const {
    // TODO(skyrim-phase2): Convert Skyrim's current game date/time into a short
    // display string such as "4E 201, 17:20". Returning nullopt is acceptable
    // for the first smoke pass if only location capture is ready.
    return std::nullopt;
}

}  // namespace skyrim_llm::skse_host
