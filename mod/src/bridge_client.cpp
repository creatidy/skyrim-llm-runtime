#include "skyrim_llm/bridge_client.hpp"

#include <chrono>
#include <filesystem>
#include <fstream>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <thread>
#include <utility>

namespace skyrim_llm {
namespace {

std::string EscapeJson(const std::string& input) {
    std::string out;
    out.reserve(input.size() + 16);
    for (const char ch : input) {
        switch (ch) {
            case '\\':
                out += "\\\\";
                break;
            case '"':
                out += "\\\"";
                break;
            case '\n':
                out += "\\n";
                break;
            case '\r':
                out += "\\r";
                break;
            case '\t':
                out += "\\t";
                break;
            default:
                out += ch;
                break;
        }
    }
    return out;
}

std::string Quote(const std::string& input) {
    return "\"" + EscapeJson(input) + "\"";
}

std::string ReadFile(const std::filesystem::path& path) {
    std::ifstream in(path);
    if (!in) {
        throw std::runtime_error("unable to open file: " + path.string());
    }
    std::ostringstream buffer;
    buffer << in.rdbuf();
    return buffer.str();
}

void WriteFile(const std::filesystem::path& path, const std::string& body) {
    std::filesystem::create_directories(path.parent_path());
    std::ofstream out(path);
    if (!out) {
        throw std::runtime_error("unable to write file: " + path.string());
    }
    out << body;
}

std::size_t SkipWhitespace(const std::string& body, std::size_t pos) {
    while (pos < body.size() && (body[pos] == ' ' || body[pos] == '\n' || body[pos] == '\r' || body[pos] == '\t')) {
        ++pos;
    }
    return pos;
}

std::optional<std::size_t> FindKey(const std::string& body, const std::string& key) {
    const auto quoted = "\"" + key + "\"";
    const auto key_pos = body.find(quoted);
    if (key_pos == std::string::npos) {
        return std::nullopt;
    }
    auto colon = body.find(':', key_pos + quoted.size());
    if (colon == std::string::npos) {
        return std::nullopt;
    }
    return SkipWhitespace(body, colon + 1);
}

std::optional<std::string> ExtractJsonString(const std::string& body, const std::string& key) {
    const auto start_pos = FindKey(body, key);
    if (!start_pos.has_value() || start_pos.value() >= body.size() || body[start_pos.value()] != '"') {
        return std::nullopt;
    }

    std::string out;
    bool escaped = false;
    for (std::size_t i = start_pos.value() + 1; i < body.size(); ++i) {
        const char ch = body[i];
        if (escaped) {
            switch (ch) {
                case '"':
                case '\\':
                case '/':
                    out += ch;
                    break;
                case 'n':
                    out += '\n';
                    break;
                case 'r':
                    out += '\r';
                    break;
                case 't':
                    out += '\t';
                    break;
                default:
                    out += ch;
                    break;
            }
            escaped = false;
            continue;
        }

        if (ch == '\\') {
            escaped = true;
            continue;
        }
        if (ch == '"') {
            return out;
        }
        out += ch;
    }

    return std::nullopt;
}

std::optional<bool> ExtractJsonBool(const std::string& body, const std::string& key) {
    const auto start_pos = FindKey(body, key);
    if (!start_pos.has_value()) {
        return std::nullopt;
    }
    if (body.compare(start_pos.value(), 4, "true") == 0) {
        return true;
    }
    if (body.compare(start_pos.value(), 5, "false") == 0) {
        return false;
    }
    return std::nullopt;
}

std::optional<std::string> ExtractSection(const std::string& body, const std::string& key, char open_ch, char close_ch) {
    const auto start_pos = FindKey(body, key);
    if (!start_pos.has_value() || start_pos.value() >= body.size() || body[start_pos.value()] != open_ch) {
        return std::nullopt;
    }

    int depth = 0;
    bool in_string = false;
    bool escaped = false;
    for (std::size_t i = start_pos.value(); i < body.size(); ++i) {
        const char ch = body[i];
        if (in_string) {
            if (escaped) {
                escaped = false;
            } else if (ch == '\\') {
                escaped = true;
            } else if (ch == '"') {
                in_string = false;
            }
            continue;
        }

        if (ch == '"') {
            in_string = true;
            continue;
        }
        if (ch == open_ch) {
            ++depth;
        } else if (ch == close_ch) {
            --depth;
            if (depth == 0) {
                return body.substr(start_pos.value(), i - start_pos.value() + 1);
            }
        }
    }

    return std::nullopt;
}

std::vector<std::string> ExtractStringArray(const std::string& body, const std::string& key) {
    const auto section = ExtractSection(body, key, '[', ']');
    if (!section.has_value()) {
        return {};
    }

    std::vector<std::string> items;
    const std::string& array_body = section.value();
    for (std::size_t i = 0; i < array_body.size(); ++i) {
        if (array_body[i] != '"') {
            continue;
        }
        std::string item;
        bool escaped = false;
        for (std::size_t j = i + 1; j < array_body.size(); ++j) {
            const char ch = array_body[j];
            if (escaped) {
                switch (ch) {
                    case 'n':
                        item += '\n';
                        break;
                    case 'r':
                        item += '\r';
                        break;
                    case 't':
                        item += '\t';
                        break;
                    default:
                        item += ch;
                        break;
                }
                escaped = false;
                continue;
            }
            if (ch == '\\') {
                escaped = true;
                continue;
            }
            if (ch == '"') {
                items.push_back(item);
                i = j;
                break;
            }
            item += ch;
        }
    }
    return items;
}

std::string SerializeRequest(const RecapRequest& request) {
    std::ostringstream out;
    out << "{\n";
    out << "  \"contract_version\": " << Quote(request.contract_version) << ",\n";
    out << "  \"request_id\": " << Quote(request.request_id) << ",\n";
    out << "  \"feature\": " << Quote(request.feature) << ",\n";
    out << "  \"created_at_utc\": " << Quote(request.created_at_utc) << ",\n";
    out << "  \"spoiler_mode\": " << Quote(request.spoiler_mode) << ",\n";
    out << "  \"game_context\": {\n";
    if (request.snapshot.game_time.has_value()) {
        out << "    \"game_time\": " << Quote(request.snapshot.game_time.value()) << ",\n";
    } else {
        out << "    \"game_time\": null,\n";
    }
    out << "    \"player_location\": " << Quote(request.snapshot.player_location) << ",\n";
    out << "    \"event_log\": [\n";
    for (std::size_t i = 0; i < request.event_log.size(); ++i) {
        const auto& event = request.event_log[i];
        out << "      {\n";
        out << "        \"t\": " << Quote(event.timestamp) << ",\n";
        out << "        \"kind\": " << Quote(event.kind) << ",\n";
        out << "        \"text\": " << Quote(event.text) << "\n";
        out << "      }";
        if (i + 1 != request.event_log.size()) {
            out << ",";
        }
        out << "\n";
    }
    out << "    ]\n";
    out << "  },\n";
    out << "  \"client\": {\n";
    out << "    \"client_kind\": \"skyrim-mod\",\n";
    out << "    \"client_version\": \"0.1.0\",\n";
    out << "    \"profile\": \"player\"\n";
    out << "  }\n";
    out << "}\n";
    return out.str();
}

RecapResponse ParseResponse(const std::string& body) {
    RecapResponse response;
    response.ok = ExtractJsonBool(body, "ok").value_or(false);
    response.meta.runtime_build_id = ExtractJsonString(body, "runtime_build_id").value_or("");
    response.meta.prompt_version = ExtractJsonString(body, "prompt_version").value_or("");
    response.meta.provider_name = ExtractJsonString(body, "name").value_or("");
    response.meta.provider_model = ExtractJsonString(body, "model").value_or("");
    response.meta.cache_hit = ExtractJsonBool(body, "hit").value_or(false);

    if (const auto recap_section = ExtractSection(body, "recap", '{', '}'); recap_section.has_value()) {
        RecapPayload recap;
        recap.summary = ExtractJsonString(recap_section.value(), "summary").value_or("");
        recap.next_steps = ExtractStringArray(recap_section.value(), "next_steps");
        recap.spoiler_risk = ExtractJsonString(recap_section.value(), "spoiler_risk").value_or("none");
        if (!recap.summary.empty()) {
            response.recap = recap;
        }
    }

    if (const auto error_section = ExtractSection(body, "error", '{', '}'); error_section.has_value()) {
        RuntimeError error;
        error.code = ExtractJsonString(error_section.value(), "code").value_or("");
        error.message = ExtractJsonString(error_section.value(), "message").value_or("");
        if (!error.code.empty() || !error.message.empty()) {
            response.error = error;
        }
    }

    return response;
}

}  // namespace

BridgeClient::BridgeClient(BridgePaths paths) : paths_(std::move(paths)) {}

std::string BridgeClient::WriteRequest(const RecapRequest& request) const {
    const std::filesystem::path path = std::filesystem::path(paths_.requests_dir) / (request.request_id + ".json");
    WriteFile(path, SerializeRequest(request));
    return path.string();
}

std::optional<RecapResponse> BridgeClient::WaitForResponse(
    const std::string& request_id,
    std::chrono::milliseconds timeout,
    std::chrono::milliseconds poll_interval) const {
    const auto start = std::chrono::steady_clock::now();
    const std::filesystem::path path = std::filesystem::path(paths_.responses_dir) / (request_id + ".json");

    while (std::chrono::steady_clock::now() - start < timeout) {
        if (std::filesystem::exists(path)) {
            return ParseResponse(ReadFile(path));
        }
        std::this_thread::sleep_for(poll_interval);
    }

    return std::nullopt;
}

}  // namespace skyrim_llm
