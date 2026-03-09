#include "skyrim_llm_skse_host/workflow.hpp"

namespace skyrim_llm::skse_host {

void HandleRecapHotkey(HostContext& host, UiApi& ui) {
    if (!host.TriggerRecapHotkey()) {
        ui.ShowStatusLine("Failed to trigger Skyrim LLM recap");
    }
}

}  // namespace skyrim_llm::skse_host
