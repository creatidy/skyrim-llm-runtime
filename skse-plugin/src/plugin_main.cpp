#include "skyrim_llm_skse_host/plugin_shell.hpp"
#include "skyrim_llm_skse_host/workflow.hpp"

#if __has_include(<SKSE/SKSE.h>)
#include <SKSE/SKSE.h>
#define SKYRIM_LLM_HAS_SKSE_API 1
#else
#define SKYRIM_LLM_HAS_SKSE_API 0
#endif

namespace skyrim_llm::skse_host
{
    namespace
    {
        bool g_shell_initialized = false;

        HostConfig BuildDefaultHostConfig()
        {
#if defined(SKYRIM_LLM_BRIDGE_BASE_DIR)
            const std::string bridge_base = SKYRIM_LLM_BRIDGE_BASE_DIR;
            return HostConfig{
                .plugin_title = "Skyrim LLM",
                .requests_dir = bridge_base + "/requests",
                .responses_dir = bridge_base + "/responses",
            };
#else
            return HostConfig{
                .plugin_title = "Skyrim LLM",
                .requests_dir = "Data/SKSE/Plugins/SkyrimLLMRuntime/requests",
                .responses_dir = "Data/SKSE/Plugins/SkyrimLLMRuntime/responses",
            };
#endif
        }

        SkyrimPluginShell &GetPluginShell()
        {
            static SkyrimPluginShell shell(BuildDefaultHostConfig());
            return shell;
        }

        bool InitializeShell()
        {
            if (g_shell_initialized) {
                return true;
            }

            auto &shell = GetPluginShell();
            if (!shell.Initialize())
            {
                return false;
            }

            shell.SeedInitialState();
            g_shell_initialized = shell.RegisterRecapHotkey();
            return g_shell_initialized;
        }

#if SKYRIM_LLM_HAS_SKSE_API
        void OnSkseMessage(SKSE::MessagingInterface::Message *message)
        {
            if (message == nullptr) {
                return;
            }

            if (message->type == SKSE::MessagingInterface::kDataLoaded) {
                InitializeShell();
            }
        }

        bool RegisterLifecycleListener()
        {
            const auto *messaging = SKSE::GetMessagingInterface();
            if (messaging == nullptr) {
                return false;
            }

            return messaging->RegisterListener(&OnSkseMessage);
        }
#endif

    } // namespace

    bool InitializePluginHost(HostContext &host, UiApi &ui)
    {
        if (!host.Initialize())
        {
            ui.ShowStatusLine("Failed to initialize Skyrim LLM plugin host");
            return false;
        }

        ShowPluginReady(ui, host.Config());
        return true;
    }

    bool InitializePluginShellForTesting()
    {
        return InitializeShell();
    }

    void TriggerRecapHotkeyForTesting()
    {
        GetPluginShell().OnRecapHotkeyPressed();
    }

} // namespace skyrim_llm::skse_host

#if SKYRIM_LLM_HAS_SKSE_API

SKSEPluginLoad(const SKSE::LoadInterface *skse)
{
    SKSE::Init(skse);

    return skyrim_llm::skse_host::RegisterLifecycleListener();
}

#endif
