#include "skyrim_llm_skse_host/plugin_shell.hpp"
#include "skyrim_llm_skse_host/workflow.hpp"

#if __has_include(<SKSE/SKSE.h>)
#include <SKSE/SKSE.h>
#define SKYRIM_LLM_HAS_SKSE_API 1
#else
#define SKYRIM_LLM_HAS_SKSE_API 0
#endif

namespace skyrim_llm::skse_host {
namespace {

HostConfig BuildDefaultHostConfig() {
    return HostConfig{
        .plugin_title = "Skyrim LLM",
        .requests_dir = "Data/SKSE/Plugins/SkyrimLLMRuntime/requests",
        .responses_dir = "Data/SKSE/Plugins/SkyrimLLMRuntime/responses",
    };
}

SkyrimPluginShell& GetPluginShell() {
    static SkyrimPluginShell shell(BuildDefaultHostConfig());
    return shell;
}

bool InitializeShell() {
    auto& shell = GetPluginShell();
    if (!shell.Initialize()) {
        return false;
    }

    shell.SeedInitialState();
    return shell.RegisterRecapHotkey();
}

}  // namespace

bool InitializePluginHost(HostContext& host, UiApi& ui) {
    if (!host.Initialize()) {
        ui.ShowStatusLine("Failed to initialize Skyrim LLM plugin host");
        return false;
    }

    ShowPluginReady(ui, host.Config());
    return true;
}

bool InitializePluginShellForTesting() {
    return InitializeShell();
}

void TriggerRecapHotkeyForTesting() {
    GetPluginShell().OnRecapHotkeyPressed();
}

}  // namespace skyrim_llm::skse_host

#if SKYRIM_LLM_HAS_SKSE_API

SKSEPluginLoad(const SKSE::LoadInterface* skse) {
    SKSE::Init(skse);

    // TODO(skyrim-phase2): Move additional Skyrim hooks that depend on later
    // lifecycle stages onto the SKSE messaging interface if needed. For now the
    // plugin shell only initializes the recap host and registers its hotkey.
    return skyrim_llm::skse_host::InitializePluginShellForTesting();
}

#endif
