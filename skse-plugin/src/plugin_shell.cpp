#include "skyrim_llm_skse_host/plugin_shell.hpp"

#include "skyrim_llm_skse_host/workflow.hpp"

#include <utility>

namespace skyrim_llm::skse_host {

SkyrimPluginShell::SkyrimPluginShell(HostConfig config)
    : config_(std::move(config)),
      ui_api_(config_.plugin_title),
      host_(config_, game_api_, ui_api_) {}

bool SkyrimPluginShell::Initialize() {
    return InitializePluginHost(host_, ui_api_);
}

void SkyrimPluginShell::SeedInitialState() {
    SeedInitialEventLog(host_, game_api_);
}

void SkyrimPluginShell::OnRecapHotkeyPressed() {
    HandleRecapHotkey(host_, ui_api_);
}

SkyrimGameApi& SkyrimPluginShell::Game() {
    return game_api_;
}

SkyrimUiApi& SkyrimPluginShell::Ui() {
    return ui_api_;
}

HostContext& SkyrimPluginShell::Host() {
    return host_;
}

}  // namespace skyrim_llm::skse_host
