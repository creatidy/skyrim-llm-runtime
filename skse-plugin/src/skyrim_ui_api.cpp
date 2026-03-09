#include "skyrim_llm_skse_host/skyrim_ui_api.hpp"

#include <iostream>
#include <utility>

namespace skyrim_llm::skse_host {

SkyrimUiApi::SkyrimUiApi(std::string channel_name)
    : channel_name_(std::move(channel_name)) {}

void SkyrimUiApi::ShowStatusLine(std::string_view text) {
    notifications_.ShowStatusLine(channel_name_, text);
}

void SkyrimUiApi::ShowMessage(std::string_view title, std::string_view body) {
    notifications_.ShowMessage(title, body);
}

}  // namespace skyrim_llm::skse_host
