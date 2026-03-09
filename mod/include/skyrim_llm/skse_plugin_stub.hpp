#pragma once

#include "skyrim_llm/controller.hpp"

#include <chrono>

namespace skyrim_llm {

class SksePluginStub {
public:
    explicit SksePluginStub(BridgePaths paths);

    void RecordLocationChange(const std::string& game_time, const std::string& location);
    void RecordQuestObjective(const std::string& game_time, const std::string& objective);
    void RecordNote(const std::string& game_time, const std::string& note);
    void OnHotkeyPressed(
        const GameSnapshot& snapshot,
        UiPresenter& ui,
        std::chrono::milliseconds timeout = std::chrono::seconds(10));

private:
    RecapController controller_;
};

}  // namespace skyrim_llm
