#include "skyrim_llm_skse_host/host_context.hpp"

#include <string>

namespace skyrim_llm::skse_host {

HostContext::HostContext(HostConfig config, GameApi& game_api, UiApi& ui_api)
    : config_(std::move(config)), game_api_(game_api), ui_api_(ui_api) {}

HostContext::~HostContext() {
    if (handle_ != nullptr) {
        SkyrimLlm_Destroy(handle_);
        handle_ = nullptr;
    }
}

bool HostContext::Initialize() {
    if (handle_ != nullptr) {
        return true;
    }

    SkyrimLlmApiConfig config{};
    config.plugin_title = config_.plugin_title.c_str();
    config.requests_dir = config_.requests_dir.c_str();
    config.responses_dir = config_.responses_dir.c_str();
    config.recap_timeout_ms = static_cast<std::uint32_t>(config_.recap_timeout.count());
    config.userdata = this;
    config.get_snapshot = &HostContext::CaptureSnapshotThunk;
    config.show_status_line = &HostContext::ShowStatusLineThunk;
    config.show_message = &HostContext::ShowMessageThunk;

    handle_ = SkyrimLlm_Create(&config);
    return handle_ != nullptr;
}

bool HostContext::IsInitialized() const {
    return handle_ != nullptr;
}

const HostConfig& HostContext::Config() const {
    return config_;
}

bool HostContext::TriggerRecapHotkey() {
    if (handle_ == nullptr && !Initialize()) {
        return false;
    }
    return SkyrimLlm_TriggerHotkeyRecap(handle_);
}

void HostContext::RecordLocationChange(const std::string& game_time, const std::string& location) {
    if (handle_ == nullptr && !Initialize()) {
        return;
    }
    SkyrimLlm_RecordLocationChange(handle_, game_time.c_str(), location.c_str());
}

void HostContext::RecordQuestObjective(const std::string& game_time, const std::string& objective) {
    if (handle_ == nullptr && !Initialize()) {
        return;
    }
    SkyrimLlm_RecordQuestObjective(handle_, game_time.c_str(), objective.c_str());
}

void HostContext::RecordNote(const std::string& game_time, const std::string& note) {
    if (handle_ == nullptr && !Initialize()) {
        return;
    }
    SkyrimLlm_RecordNote(handle_, game_time.c_str(), note.c_str());
}

bool HostContext::CaptureSnapshotThunk(void* userdata, SkyrimLlmSnapshot* out_snapshot) {
    if (userdata == nullptr || out_snapshot == nullptr) {
        return false;
    }

    auto& host = *static_cast<HostContext*>(userdata);
    const auto snapshot = host.game_api_.CaptureSnapshot();
    if (!snapshot.has_value() || snapshot->player_location.empty()) {
        return false;
    }

    static thread_local std::string game_time_buffer;
    static thread_local std::string location_buffer;

    game_time_buffer = snapshot->game_time.value_or("");
    location_buffer = snapshot->player_location;

    out_snapshot->game_time = game_time_buffer.empty() ? nullptr : game_time_buffer.c_str();
    out_snapshot->player_location = location_buffer.c_str();
    return true;
}

void HostContext::ShowStatusLineThunk(void* userdata, const char* text) {
    if (userdata == nullptr || text == nullptr) {
        return;
    }

    auto& host = *static_cast<HostContext*>(userdata);
    host.ui_api_.ShowStatusLine(text);
}

void HostContext::ShowMessageThunk(void* userdata, const char* title, const char* body) {
    if (userdata == nullptr || title == nullptr || body == nullptr) {
        return;
    }

    auto& host = *static_cast<HostContext*>(userdata);
    host.ui_api_.ShowMessage(title, body);
}

}  // namespace skyrim_llm::skse_host
