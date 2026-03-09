#pragma once

#include <optional>
#include <string>

namespace skyrim_llm::skse_host {

class LocationCapture {
public:
    std::optional<std::string> CapturePlayerLocation() const;
};

}  // namespace skyrim_llm::skse_host
