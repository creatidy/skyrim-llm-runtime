#pragma once

#include <string_view>

namespace skyrim_llm::skse_host {

class UiApi {
public:
    virtual ~UiApi() = default;

    virtual void ShowStatusLine(std::string_view text) = 0;
    virtual void ShowMessage(std::string_view title, std::string_view body) = 0;
};

}  // namespace skyrim_llm::skse_host
