#pragma once

#include "skyrim_llm/plugin_api.hpp"
#include "skyrim_llm_skse_host/game_api.hpp"
#include "skyrim_llm_skse_host/ui_api.hpp"

#include <chrono>
#include <string>

namespace skyrim_llm::skse_host {

struct HostConfig {
    std::string plugin_title{"Skyrim LLM"};
    std::string requests_dir{"Data/SKSE/Plugins/SkyrimLLMRuntime/requests"};
    std::string responses_dir{"Data/SKSE/Plugins/SkyrimLLMRuntime/responses"};
    std::chrono::milliseconds recap_timeout{std::chrono::seconds(10)};
};

class HostContext {
public:
    HostContext(HostConfig config, GameApi& game_api, UiApi& ui_api);
    ~HostContext();

    HostContext(const HostContext&) = delete;
    HostContext& operator=(const HostContext&) = delete;

    bool Initialize();
    bool IsInitialized() const;
    const HostConfig& Config() const;

    bool TriggerRecapHotkey();

    void RecordLocationChange(const std::string& game_time, const std::string& location);
    void RecordQuestObjective(const std::string& game_time, const std::string& objective);
    void RecordNote(const std::string& game_time, const std::string& note);

private:
    static bool CaptureSnapshotThunk(void* userdata, SkyrimLlmSnapshot* out_snapshot);
    static void ShowStatusLineThunk(void* userdata, const char* text);
    static void ShowMessageThunk(void* userdata, const char* title, const char* body);

    HostConfig config_;
    GameApi& game_api_;
    UiApi& ui_api_;
    SkyrimLlmPluginHandle* handle_{nullptr};
};

}  // namespace skyrim_llm::skse_host
