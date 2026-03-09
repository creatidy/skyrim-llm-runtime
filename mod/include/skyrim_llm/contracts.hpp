#pragma once

#include <optional>
#include <string>
#include <string_view>
#include <vector>

namespace skyrim_llm {

struct EventEntry {
    std::string timestamp;
    std::string kind;
    std::string text;
};

struct GameSnapshot {
    std::optional<std::string> game_time;
    std::string player_location;
};

struct RecapRequest {
    std::string contract_version;
    std::string request_id;
    std::string feature;
    std::string created_at_utc;
    std::string spoiler_mode;
    GameSnapshot snapshot;
    std::vector<EventEntry> event_log;
};

struct RecapPayload {
    std::string summary;
    std::vector<std::string> next_steps;
    std::string spoiler_risk;
};

struct RuntimeError {
    std::string code;
    std::string message;
};

struct ResponseMeta {
    std::string runtime_build_id;
    std::string prompt_version;
    std::string provider_name;
    std::string provider_model;
    bool cache_hit{false};
};

struct RecapResponse {
    bool ok{false};
    std::optional<RecapPayload> recap;
    std::optional<RuntimeError> error;
    ResponseMeta meta;
};

struct BridgePaths {
    std::string requests_dir;
    std::string responses_dir;
};

constexpr std::string_view kRequestVersion = "v1";
constexpr std::string_view kResponseVersion = "v1";

}  // namespace skyrim_llm
