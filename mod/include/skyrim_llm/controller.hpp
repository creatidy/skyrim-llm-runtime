#pragma once

#include "skyrim_llm/bridge_client.hpp"

#include <chrono>
#include <deque>
#include <string>

namespace skyrim_llm {

class UiPresenter {
public:
    virtual ~UiPresenter() = default;

    virtual void ShowStatus(const std::string& text) = 0;
    virtual void ShowRecap(const RecapPayload& recap, bool cache_hit) = 0;
    virtual void ShowError(const std::string& text) = 0;
};

class RecapController {
public:
    explicit RecapController(BridgePaths paths);

    void RecordLocationChange(const std::string& game_time, const std::string& location);
    void RecordQuestObjective(const std::string& game_time, const std::string& objective);
    void RecordNote(const std::string& game_time, const std::string& note);
    void TriggerHotkeyRecap(
        const GameSnapshot& snapshot,
        UiPresenter& ui,
        std::chrono::milliseconds timeout = std::chrono::seconds(10));

private:
    static std::string BuildRequestId();
    void AppendEvent(const std::string& timestamp, const std::string& kind, const std::string& text);

    BridgeClient bridge_;
    std::deque<EventEntry> events_;
    std::size_t max_events_{40};
    std::size_t max_chars_per_event_{220};
};

class ConsoleUiPresenter final : public UiPresenter {
public:
    void ShowStatus(const std::string& text) override;
    void ShowRecap(const RecapPayload& recap, bool cache_hit) override;
    void ShowError(const std::string& text) override;
};

std::string MapRuntimeError(const std::optional<RuntimeError>& error);

}  // namespace skyrim_llm
