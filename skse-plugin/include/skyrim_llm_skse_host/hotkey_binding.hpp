#pragma once

#include <functional>

namespace skyrim_llm::skse_host {

class RecapHotkeyBinding {
public:
    void SetHandler(std::function<void()> handler);
    bool RegisterDefaultHotkey();
    bool IsRegistered() const;
    void DispatchForTesting() const;

private:
    std::function<void()> handler_;
    bool registered_{false};
};

}  // namespace skyrim_llm::skse_host
