#include "skyrim_llm_skse_host/notification_ui.hpp"

#include <iostream>

namespace skyrim_llm::skse_host {

void NotificationUi::ShowStatusLine(std::string_view channel_name, std::string_view text) const {
    // TODO(skyrim-phase2): Map this to a real in-game HUD notification or status line.
    std::cout << '[' << channel_name << "] " << text << '\n';
}

void NotificationUi::ShowMessage(std::string_view title, std::string_view body) const {
    // TODO(skyrim-phase2): Map this to a real message box or text panel in Skyrim.
    std::cout << '[' << title << "]\n" << body << '\n';
}

}  // namespace skyrim_llm::skse_host
