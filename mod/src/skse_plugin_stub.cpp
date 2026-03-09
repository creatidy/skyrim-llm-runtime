#include "skyrim_llm/skse_plugin_stub.hpp"

namespace skyrim_llm {

SksePluginStub::SksePluginStub(BridgePaths paths) : controller_(std::move(paths)) {}

void SksePluginStub::OnHotkeyPressed(const GameSnapshot& snapshot, UiPresenter& ui) {
    // Real SKSE input and UI hooks belong here. The controller already owns the
    // bounded event log, file-bridge request write, response polling, and error mapping.
    controller_.TriggerHotkeyRecap(snapshot, ui);
}

}  // namespace skyrim_llm
