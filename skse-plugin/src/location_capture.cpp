#include "skyrim_llm_skse_host/location_capture.hpp"

namespace skyrim_llm::skse_host {

std::optional<std::string> LocationCapture::CapturePlayerLocation() const {
    // TODO(skyrim-phase2): Read the current player cell/location name from Skyrim.
    // Expected result example: "Whiterun", "Bleak Falls Barrow", "Dragonsreach".
    return std::nullopt;
}

}  // namespace skyrim_llm::skse_host
