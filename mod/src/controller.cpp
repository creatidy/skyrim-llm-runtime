#include "skyrim_llm/controller.hpp"

#include <algorithm>
#include <chrono>
#include <cstdint>
#include <ctime>
#include <iomanip>
#include <iostream>
#include <sstream>

namespace skyrim_llm {
namespace {

std::string Truncate(const std::string& text, std::size_t limit) {
    if (text.size() <= limit) {
        return text;
    }
    return text.substr(0, limit);
}

std::string CurrentUtcTimestamp() {
    const auto now = std::chrono::system_clock::now();
    const std::time_t time_value = std::chrono::system_clock::to_time_t(now);
    std::tm tm{};
#if defined(_WIN32)
    gmtime_s(&tm, &time_value);
#else
    gmtime_r(&time_value, &tm);
#endif
    std::ostringstream out;
    out << std::put_time(&tm, "%Y-%m-%dT%H:%M:%SZ");
    return out.str();
}

std::string CurrentUtcCompactTimestamp() {
    const auto now = std::chrono::system_clock::now();
    const std::time_t time_value = std::chrono::system_clock::to_time_t(now);
    std::tm tm{};
#if defined(_WIN32)
    gmtime_s(&tm, &time_value);
#else
    gmtime_r(&time_value, &tm);
#endif
    std::ostringstream out;
    out << std::put_time(&tm, "%Y%m%dT%H%M%SZ");
    return out.str();
}

}  // namespace

RecapController::RecapController(BridgePaths paths) : bridge_(std::move(paths)) {}

void RecapController::RecordLocationChange(const std::string& game_time, const std::string& location) {
    AppendEvent(game_time, "location", "Entered " + location);
}

void RecapController::RecordQuestObjective(const std::string& game_time, const std::string& objective) {
    AppendEvent(game_time, "quest", objective);
}

void RecapController::RecordNote(const std::string& game_time, const std::string& note) {
    AppendEvent(game_time, "note", note);
}

void RecapController::TriggerHotkeyRecap(
    const GameSnapshot& snapshot,
    UiPresenter& ui,
    std::chrono::milliseconds timeout) {
    ui.ShowStatus("Generating recap...");

    RecapRequest request;
    request.contract_version = std::string(kRequestVersion);
    request.request_id = BuildRequestId();
    request.feature = "recap";
    request.created_at_utc = CurrentUtcTimestamp();
    request.spoiler_mode = "safe";
    request.snapshot = snapshot;
    request.event_log.assign(events_.begin(), events_.end());

    bridge_.WriteRequest(request);

    const auto response = bridge_.WaitForResponse(
        request.request_id,
        timeout,
        std::chrono::milliseconds(150));

    if (!response.has_value()) {
        ui.ShowError("Runtime unavailable");
        return;
    }

    if (response->ok && response->recap.has_value()) {
        ui.ShowRecap(response->recap.value(), response->meta.cache_hit);
        return;
    }

    ui.ShowError(MapRuntimeError(response->error));
}

std::string RecapController::BuildRequestId() {
    static std::uint64_t counter = 0;
    ++counter;
    return "skyrim-mod-" + CurrentUtcCompactTimestamp() + "-" + std::to_string(counter);
}

void RecapController::AppendEvent(const std::string& timestamp, const std::string& kind, const std::string& text) {
    events_.push_back(EventEntry{
        .timestamp = timestamp,
        .kind = kind,
        .text = Truncate(text, max_chars_per_event_),
    });
    while (events_.size() > max_events_) {
        events_.pop_front();
    }
}

void ConsoleUiPresenter::ShowStatus(const std::string& text) {
    std::cout << "[status] " << text << '\n';
}

void ConsoleUiPresenter::ShowRecap(const RecapPayload& recap, bool cache_hit) {
    std::cout << "[recap] " << recap.summary << '\n';
    std::cout << "[cache] " << (cache_hit ? "cached" : "fresh") << '\n';
    for (std::size_t i = 0; i < recap.next_steps.size(); ++i) {
        std::cout << "  " << (i + 1) << ". " << recap.next_steps[i] << '\n';
    }
}

void ConsoleUiPresenter::ShowError(const std::string& text) {
    std::cout << "[error] " << text << '\n';
}

std::string MapRuntimeError(const std::optional<RuntimeError>& error) {
    if (!error.has_value()) {
        return "Runtime unavailable";
    }

    if (error->code == "runtime_offline") {
        return "Runtime unavailable";
    }
    if (error->code == "provider_error") {
        return "Provider failed, fallback used if available";
    }
    if (error->code == "budget_exceeded") {
        return "Budget cap reached";
    }
    if (error->code == "validation_failed") {
        return "Response invalid; safe fallback applied";
    }
    if (error->code == "transport_error") {
        return "Bridge file error";
    }
    if (!error->message.empty()) {
        return error->message;
    }
    return "Runtime unavailable";
}

}  // namespace skyrim_llm
