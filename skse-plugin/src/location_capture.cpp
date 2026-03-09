#include "skyrim_llm_skse_host/location_capture.hpp"

#if __has_include(<RE/Skyrim.h>)
#include <RE/Skyrim.h>
#define SKYRIM_LLM_HAS_RE_API 1
#else
#define SKYRIM_LLM_HAS_RE_API 0
#endif

namespace skyrim_llm::skse_host {
namespace {

#if SKYRIM_LLM_HAS_RE_API
template <class T>
std::optional<std::string> ReadName(const T* form) {
    if (form == nullptr) {
        return std::nullopt;
    }

    const char* name = form->GetName();
    if (name == nullptr || name[0] == '\0') {
        return std::nullopt;
    }

    return std::string(name);
}
#endif

}  // namespace

std::optional<std::string> LocationCapture::CapturePlayerLocation() const {
#if SKYRIM_LLM_HAS_RE_API
    auto* player = RE::PlayerCharacter::GetSingleton();
    if (player == nullptr) {
        return std::nullopt;
    }

    // Prefer the broader location record when it has a display name.
    if (const auto* location = player->GetCurrentLocation()) {
        if (const auto location_name = ReadName(location); location_name.has_value()) {
            return location_name;
        }
    }

    // Fall back to the current cell/interior name if no location name is available.
    if (const auto* cell = player->GetParentCell()) {
        if (const auto cell_name = ReadName(cell); cell_name.has_value()) {
            return cell_name;
        }
    }
#endif

    return std::nullopt;
}

}  // namespace skyrim_llm::skse_host
