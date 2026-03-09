#pragma once

#include "skyrim_llm_skse_host/ui_api.hpp"

#include <string>

namespace skyrim_llm::skse_host {

class SkyrimUiApi final : public UiApi {
public:
    explicit SkyrimUiApi(std::string channel_name = "Skyrim LLM");

    void ShowStatusLine(std::string_view text) override;
    void ShowMessage(std::string_view title, std::string_view body) override;

private:
    std::string channel_name_;
};

}  // namespace skyrim_llm::skse_host
