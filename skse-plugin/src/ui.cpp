#include "skyrim_llm_skse_host/workflow.hpp"

namespace skyrim_llm::skse_host {

void ShowPluginReady(UiApi& ui, const HostConfig& config) {
    ui.ShowStatusLine(config.plugin_title + " host ready");
}

}  // namespace skyrim_llm::skse_host
