#include "skyrim_llm_skse_host/game_time_capture.hpp"

#if __has_include(<RE/Skyrim.h>)
#include <RE/Skyrim.h>
#define SKYRIM_LLM_HAS_RE_API 1
#else
#define SKYRIM_LLM_HAS_RE_API 0
#endif

#include <iomanip>
#include <sstream>

namespace skyrim_llm::skse_host {

std::optional<std::string> GameTimeCapture::CaptureGameTime() const {
#if SKYRIM_LLM_HAS_RE_API
    auto* calendar = RE::Calendar::GetSingleton();
    if (calendar == nullptr) {
        return std::nullopt;
    }

    const auto year = calendar->GetYear();
    const auto time = calendar->GetTime();
    const auto hour = static_cast<unsigned int>(time.tm_hour) % 24;
    const auto minutes = static_cast<unsigned int>(time.tm_min) % 60;

    std::ostringstream out;
    out << "4E " << year << ", " << std::setfill('0') << std::setw(2) << hour << ':'
        << std::setw(2) << minutes;
    return out.str();
#endif

    return std::nullopt;
}

}  // namespace skyrim_llm::skse_host
