#include "skyrim_llm_skse_host/hotkey_binding.hpp"

namespace skyrim_llm::skse_host {

void RecapHotkeyBinding::SetHandler(std::function<void()> handler) {
    handler_ = std::move(handler);
}

bool RecapHotkeyBinding::RegisterDefaultHotkey() {
    // TODO(skyrim-phase2): Register one real in-game hotkey through the chosen
    // SKSE/CommonLibSSE input path and invoke handler_ when it fires.
    registered_ = (handler_ != nullptr);
    return registered_;
}

bool RecapHotkeyBinding::IsRegistered() const {
    return registered_;
}

void RecapHotkeyBinding::DispatchForTesting() const {
    if (handler_ != nullptr) {
        handler_();
    }
}

}  // namespace skyrim_llm::skse_host
