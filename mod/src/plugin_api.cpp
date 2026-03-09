#include "skyrim_llm/plugin_api.hpp"

#include "skyrim_llm/skyrim_integration.hpp"

#include <new>
#include <optional>
#include <string>
#include <utility>

namespace skyrim_llm {
namespace {

class CallbackSnapshotProvider final : public GameSnapshotProvider {
public:
    CallbackSnapshotProvider(void* userdata, SkyrimLlmGetSnapshotFn get_snapshot)
        : userdata_(userdata), get_snapshot_(get_snapshot) {}

    std::optional<GameSnapshot> CaptureSnapshot() override {
        if (get_snapshot_ == nullptr) {
            return std::nullopt;
        }

        SkyrimLlmSnapshot raw{};
        if (!get_snapshot_(userdata_, &raw) || raw.player_location == nullptr || raw.player_location[0] == '\0') {
            return std::nullopt;
        }

        GameSnapshot snapshot;
        if (raw.game_time != nullptr && raw.game_time[0] != '\0') {
            snapshot.game_time = std::string(raw.game_time);
        }
        snapshot.player_location = std::string(raw.player_location);
        return snapshot;
    }

private:
    void* userdata_;
    SkyrimLlmGetSnapshotFn get_snapshot_;
};

class CallbackNotificationSink final : public NotificationSink {
public:
    CallbackNotificationSink(
        void* userdata,
        SkyrimLlmShowStatusLineFn show_status_line,
        SkyrimLlmShowMessageFn show_message)
        : userdata_(userdata),
          show_status_line_(show_status_line),
          show_message_(show_message) {}

    void ShowStatusLine(const std::string& text) override {
        if (show_status_line_ != nullptr) {
            show_status_line_(userdata_, text.c_str());
        }
    }

    void ShowMessage(const std::string& title, const std::string& body) override {
        if (show_message_ != nullptr) {
            show_message_(userdata_, title.c_str(), body.c_str());
            return;
        }

        if (show_status_line_ != nullptr) {
            show_status_line_(userdata_, body.c_str());
        }
    }

private:
    void* userdata_;
    SkyrimLlmShowStatusLineFn show_status_line_;
    SkyrimLlmShowMessageFn show_message_;
};

IntegrationConfig BuildIntegrationConfig(const SkyrimLlmApiConfig& config) {
    IntegrationConfig integration;
    if (config.plugin_title != nullptr && config.plugin_title[0] != '\0') {
        integration.plugin_title = config.plugin_title;
    }
    if (config.recap_timeout_ms != 0) {
        integration.recap_timeout = std::chrono::milliseconds(config.recap_timeout_ms);
    }
    integration.bridge_paths.requests_dir = config.requests_dir != nullptr ? config.requests_dir : "";
    integration.bridge_paths.responses_dir = config.responses_dir != nullptr ? config.responses_dir : "";
    return integration;
}

bool HasRequiredConfig(const SkyrimLlmApiConfig& config) {
    return config.requests_dir != nullptr && config.requests_dir[0] != '\0' &&
           config.responses_dir != nullptr && config.responses_dir[0] != '\0' &&
           config.get_snapshot != nullptr;
}

}  // namespace
}  // namespace skyrim_llm

struct SkyrimLlmPluginHandle {
    skyrim_llm::CallbackSnapshotProvider snapshot_provider;
    skyrim_llm::CallbackNotificationSink notification_sink;
    skyrim_llm::SkyrimModIntegration integration;

    explicit SkyrimLlmPluginHandle(const SkyrimLlmApiConfig& config)
        : snapshot_provider(config.userdata, config.get_snapshot),
          notification_sink(config.userdata, config.show_status_line, config.show_message),
          integration(
              skyrim_llm::BuildIntegrationConfig(config),
              snapshot_provider,
              notification_sink) {}
};

extern "C" {

SkyrimLlmPluginHandle* SkyrimLlm_Create(const SkyrimLlmApiConfig* config) {
    if (config == nullptr || !skyrim_llm::HasRequiredConfig(*config)) {
        return nullptr;
    }

    return new (std::nothrow) SkyrimLlmPluginHandle(*config);
}

void SkyrimLlm_Destroy(SkyrimLlmPluginHandle* handle) {
    delete handle;
}

void SkyrimLlm_RecordLocationChange(
    SkyrimLlmPluginHandle* handle,
    const char* game_time,
    const char* location) {
    if (handle == nullptr || game_time == nullptr || location == nullptr) {
        return;
    }
    handle->integration.RecordLocationChange(game_time, location);
}

void SkyrimLlm_RecordQuestObjective(
    SkyrimLlmPluginHandle* handle,
    const char* game_time,
    const char* objective) {
    if (handle == nullptr || game_time == nullptr || objective == nullptr) {
        return;
    }
    handle->integration.RecordQuestObjective(game_time, objective);
}

void SkyrimLlm_RecordNote(
    SkyrimLlmPluginHandle* handle,
    const char* game_time,
    const char* note) {
    if (handle == nullptr || game_time == nullptr || note == nullptr) {
        return;
    }
    handle->integration.RecordNote(game_time, note);
}

bool SkyrimLlm_TriggerHotkeyRecap(SkyrimLlmPluginHandle* handle) {
    if (handle == nullptr) {
        return false;
    }
    return handle->integration.TriggerHotkeyRecap();
}

}
