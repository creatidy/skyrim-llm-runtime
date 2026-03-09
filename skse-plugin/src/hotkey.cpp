#include "skyrim_llm_skse_host/workflow.hpp"

namespace skyrim_llm::skse_host {

bool RegisterRecapHotkey(RecapHotkeyBinding& binding, UiApi& ui) {
    if (!binding.RegisterDefaultHotkey()) {
        ui.ShowStatusLine("Failed to register Skyrim LLM recap hotkey");
        return false;
    }
    return true;
}

void HandleRecapHotkey(HostContext& host, UiApi& ui) {
    if (!host.TriggerRecapHotkey()) {
        ui.ShowStatusLine("Failed to trigger Skyrim LLM recap");
    }
}

}  // namespace skyrim_llm::skse_host
