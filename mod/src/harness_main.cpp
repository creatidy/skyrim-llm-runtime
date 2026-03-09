#include "skyrim_llm/skyrim_integration.hpp"

#include <iostream>

namespace {

class HarnessSnapshotProvider final : public skyrim_llm::GameSnapshotProvider {
public:
    std::optional<skyrim_llm::GameSnapshot> CaptureSnapshot() override {
        return skyrim_llm::GameSnapshot{
            .game_time = std::optional<std::string>("4E 201, 17:20"),
            .player_location = "Whiterun",
        };
    }
};

class HarnessNotificationSink final : public skyrim_llm::NotificationSink {
public:
    void ShowStatusLine(const std::string& text) override {
        std::cout << "[status] " << text << '\n';
    }

    void ShowMessage(const std::string& title, const std::string& body) override {
        std::cout << '[' << title << "]\n" << body << '\n';
    }
};

}  // namespace

int main() {
    skyrim_llm::IntegrationConfig config{
        .plugin_title = "Skyrim LLM Harness",
        .bridge_paths =
            skyrim_llm::BridgePaths{
                .requests_dir = "runtime/bridge/requests",
                .responses_dir = "runtime/bridge/responses",
            },
    };
    HarnessSnapshotProvider snapshot_provider;
    HarnessNotificationSink notifications;
    skyrim_llm::SkyrimModIntegration integration(config, snapshot_provider, notifications);

    integration.RecordLocationChange("17:00", "Whiterun");
    integration.RecordQuestObjective("17:03", "Spoke to Farengar and received a dragonstone objective.");
    integration.RecordNote("17:10", "Left Dragonsreach and prepared to travel.");

    std::cout << "Press Enter to simulate the recap hotkey..." << '\n';
    std::string ignored;
    std::getline(std::cin, ignored);

    integration.TriggerHotkeyRecap();

    return 0;
}
