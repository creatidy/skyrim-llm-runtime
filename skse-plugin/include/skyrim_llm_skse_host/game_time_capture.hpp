#pragma once

#include <optional>
#include <string>

namespace skyrim_llm::skse_host {

class GameTimeCapture {
public:
    std::optional<std::string> CaptureGameTime() const;
};

}  // namespace skyrim_llm::skse_host
