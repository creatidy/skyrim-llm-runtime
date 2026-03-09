#pragma once

#include "skyrim_llm_skse_host/host_context.hpp"
#include "skyrim_llm_skse_host/hotkey_binding.hpp"
#include "skyrim_llm_skse_host/skyrim_game_api.hpp"
#include "skyrim_llm_skse_host/skyrim_ui_api.hpp"

namespace skyrim_llm::skse_host {

class SkyrimPluginShell {
public:
    explicit SkyrimPluginShell(HostConfig config = {});

    bool Initialize();
    bool RegisterRecapHotkey();
    void SeedInitialState();
    void OnRecapHotkeyPressed();

    SkyrimGameApi& Game();
    SkyrimUiApi& Ui();
    HostContext& Host();

private:
    HostConfig config_;
    SkyrimGameApi game_api_;
    SkyrimUiApi ui_api_;
    HostContext host_;
    RecapHotkeyBinding hotkey_binding_;
};

}  // namespace skyrim_llm::skse_host
