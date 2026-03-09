#pragma once

#include "skyrim_llm/controller.hpp"

namespace skyrim_llm {

class SksePluginStub {
public:
    explicit SksePluginStub(BridgePaths paths);

    void OnHotkeyPressed(const GameSnapshot& snapshot, UiPresenter& ui);

private:
    RecapController controller_;
};

}  // namespace skyrim_llm
