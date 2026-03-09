#include "skyrim_llm/controller.hpp"

#include <iostream>

int main() {
    skyrim_llm::BridgePaths paths{
        .requests_dir = "runtime/bridge/requests",
        .responses_dir = "runtime/bridge/responses",
    };
    skyrim_llm::RecapController controller(paths);
    skyrim_llm::ConsoleUiPresenter ui;

    controller.RecordLocationChange("17:00", "Whiterun");
    controller.RecordQuestObjective("17:03", "Spoke to Farengar and received a dragonstone objective.");
    controller.RecordNote("17:10", "Left Dragonsreach and prepared to travel.");

    std::cout << "Press Enter to simulate the recap hotkey..." << '\n';
    std::string ignored;
    std::getline(std::cin, ignored);

    controller.TriggerHotkeyRecap(
        skyrim_llm::GameSnapshot{
            .game_time = std::optional<std::string>("4E 201, 17:20"),
            .player_location = "Whiterun",
        },
        ui);

    return 0;
}
