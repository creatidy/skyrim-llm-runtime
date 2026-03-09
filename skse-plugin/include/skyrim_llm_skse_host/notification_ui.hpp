#pragma once

#include <string>
#include <string_view>

namespace skyrim_llm::skse_host {

class NotificationUi {
public:
    void ShowStatusLine(std::string_view channel_name, std::string_view text) const;
    void ShowMessage(std::string_view title, std::string_view body) const;

private:
    void ShowHudNotification(std::string_view text) const;
    void ShowMessageBox(std::string_view title, std::string_view body) const;
    static std::string BuildStatusText(std::string_view channel_name, std::string_view text);
    static std::string BuildMessageText(std::string_view title, std::string_view body);
};

}  // namespace skyrim_llm::skse_host
