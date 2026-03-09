#include "skyrim_llm_skse_host/workflow.hpp"

namespace skyrim_llm::skse_host {

bool InitializePluginHost(HostContext& host, UiApi& ui) {
    if (!host.Initialize()) {
        ui.ShowStatusLine("Failed to initialize Skyrim LLM plugin host");
        return false;
    }

    ShowPluginReady(ui, host.Config());
    return true;
}

}  // namespace skyrim_llm::skse_host
