#pragma once

#include "skyrim_llm_skse_host/hotkey_binding.hpp"
#include "skyrim_llm_skse_host/host_context.hpp"

namespace skyrim_llm::skse_host {

bool InitializePluginHost(HostContext& host, UiApi& ui);
bool RegisterRecapHotkey(RecapHotkeyBinding& binding, UiApi& ui);
void HandleRecapHotkey(HostContext& host, UiApi& ui);
void SeedInitialEventLog(HostContext& host, GameApi& game_api);
void ShowPluginReady(UiApi& ui, const HostConfig& config);

}  // namespace skyrim_llm::skse_host
