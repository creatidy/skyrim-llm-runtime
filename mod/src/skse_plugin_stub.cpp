#include "skyrim_llm/skse_plugin_stub.hpp"

namespace skyrim_llm {

SksePluginStub::SksePluginStub(BridgePaths paths) : controller_(std::move(paths)) {}

void SksePluginStub::RecordLocationChange(const std::string& game_time, const std::string& location) {
    controller_.RecordLocationChange(game_time, location);
}

void SksePluginStub::RecordQuestObjective(const std::string& game_time, const std::string& objective) {
    controller_.RecordQuestObjective(game_time, objective);
}

void SksePluginStub::RecordNote(const std::string& game_time, const std::string& note) {
    controller_.RecordNote(game_time, note);
}

void SksePluginStub::OnHotkeyPressed(
    const GameSnapshot& snapshot,
    UiPresenter& ui,
    std::chrono::milliseconds timeout) {
    // Real SKSE input and UI hooks belong here. The controller already owns the
    // bounded event log, file-bridge request write, response polling, and error mapping.
    controller_.TriggerHotkeyRecap(snapshot, ui, timeout);
}

}  // namespace skyrim_llm
