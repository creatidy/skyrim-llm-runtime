#include "skyrim_llm_skse_host/notification_ui.hpp"

#if __has_include(<RE/M/Misc.h>)
#include <RE/M/Misc.h>
#define SKYRIM_LLM_HAS_RE_UI 1
#elif __has_include(<RE/Skyrim.h>)
#include <RE/Skyrim.h>
#define SKYRIM_LLM_HAS_RE_UI 1
#else
#define SKYRIM_LLM_HAS_RE_UI 0
#endif

#include <iostream>
#include <string>

namespace skyrim_llm::skse_host {

void NotificationUi::ShowStatusLine(std::string_view channel_name, std::string_view text) const {
#if SKYRIM_LLM_HAS_RE_UI
    const std::string notification = std::string(channel_name) + ": " + std::string(text);
    RE::DebugNotification(notification.c_str());
    return;
#endif

    std::cout << '[' << channel_name << "] " << text << '\n';
}

void NotificationUi::ShowMessage(std::string_view title, std::string_view body) const {
#if SKYRIM_LLM_HAS_RE_UI
    const std::string message = std::string(title) + "\n\n" + std::string(body);
    RE::DebugMessageBox(message.c_str());
    return;
#endif

    std::cout << '[' << title << "]\n" << body << '\n';
}

}  // namespace skyrim_llm::skse_host
