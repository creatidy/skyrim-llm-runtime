#pragma once

#include "skyrim_llm/skse_plugin_stub.hpp"

#include <chrono>
#include <optional>
#include <string>

namespace skyrim_llm {

class GameSnapshotProvider {
public:
    virtual ~GameSnapshotProvider() = default;

    virtual std::optional<GameSnapshot> CaptureSnapshot() = 0;
};

class NotificationSink {
public:
    virtual ~NotificationSink() = default;

    virtual void ShowStatusLine(const std::string& text) = 0;
    virtual void ShowMessage(const std::string& title, const std::string& body) = 0;
};

class SkyrimUiPresenter final : public UiPresenter {
public:
    SkyrimUiPresenter(NotificationSink& sink, std::string title);

    void ShowStatus(const std::string& text) override;
    void ShowRecap(const RecapPayload& recap, bool cache_hit) override;
    void ShowError(const std::string& text) override;

private:
    NotificationSink& sink_;
    std::string title_;
};

struct IntegrationConfig {
    std::string plugin_title{"Skyrim LLM"};
    BridgePaths bridge_paths;
    std::chrono::milliseconds recap_timeout{std::chrono::seconds(10)};
};

class SkyrimModIntegration {
public:
    SkyrimModIntegration(
        IntegrationConfig config,
        GameSnapshotProvider& snapshot_provider,
        NotificationSink& notification_sink);

    void RecordLocationChange(const std::string& game_time, const std::string& location);
    void RecordQuestObjective(const std::string& game_time, const std::string& objective);
    void RecordNote(const std::string& game_time, const std::string& note);
    bool TriggerHotkeyRecap();

private:
    GameSnapshotProvider& snapshot_provider_;
    NotificationSink& notification_sink_;
    SkyrimUiPresenter ui_;
    SksePluginStub stub_;
    std::chrono::milliseconds recap_timeout_;
};

}  // namespace skyrim_llm
