#include "skyrim_llm_skse_host/skyrim_ui_api.hpp"

#include <iostream>
#include <utility>

namespace skyrim_llm::skse_host {

SkyrimUiApi::SkyrimUiApi(std::string channel_name)
    : channel_name_(std::move(channel_name)) {}

void SkyrimUiApi::ShowStatusLine(std::string_view text) {
    std::cout << '[' << channel_name_ << "] " << text << '\n';
}

void SkyrimUiApi::ShowMessage(std::string_view title, std::string_view body) {
    std::cout << '[' << title << "]\n" << body << '\n';
}

}  // namespace skyrim_llm::skse_host
