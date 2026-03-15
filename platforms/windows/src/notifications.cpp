#include "notifications.h"

namespace goxviet {

EventBus& EventBus::Instance() {
    static EventBus instance;
    return instance;
}

void EventBus::Subscribe(AppEvent event, void* owner, EventHandler handler) {
    std::lock_guard<std::mutex> lock(mutex_);
    auto& vec = subs_[static_cast<int>(event)];
    for (auto& s : vec) {
        if (s.owner == owner) { s.handler = std::move(handler); return; }
    }
    vec.push_back({ owner, std::move(handler) });
}

void EventBus::Unsubscribe(void* owner) {
    std::lock_guard<std::mutex> lock(mutex_);
    for (auto& [key, vec] : subs_)
        vec.erase(std::remove_if(vec.begin(), vec.end(),
                  [owner](const Subscription& s){ return s.owner == owner; }),
                  vec.end());
}

void EventBus::Post(AppEvent event) {
    std::vector<Subscription> copy;
    {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = subs_.find(static_cast<int>(event));
        if (it != subs_.end()) copy = it->second;
    }
    for (auto& s : copy) s.handler(event);
}

}  // namespace goxviet
