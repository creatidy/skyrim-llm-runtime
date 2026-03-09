#pragma once

#include <string_view>

namespace skyrim_llm::skse_host {

class NotificationUi {
public:
    void ShowStatusLine(std::string_view channel_name, std::string_view text) const;
    void ShowMessage(std::string_view title, std::string_view body) const;
};

}  // namespace skyrim_llm::skse_host
