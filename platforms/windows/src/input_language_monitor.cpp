#include "input_language_monitor.h"
#include "notifications.h"
#include "utils.h"

// Guard constants that are absent from older / ARM64 Windows SDK versions.
#ifndef LANG_SINHALA
#define LANG_SINHALA   0x5B
#endif
#ifndef LANG_KAZAKH
#define LANG_KAZAKH    0x3F
#endif
#ifndef LANG_AMHARIC
#define LANG_AMHARIC   0x5E
#endif
#ifndef LANG_TIBETAN
#define LANG_TIBETAN   0x51
#endif

namespace goxviet {

// Primary language IDs considered non-Latin — matches macOS InputSourceMonitor list.
// Explicit size keeps the array a complete type for range-based for.
static const LANGID kNonLatinPrimaries[] = {
    LANG_JAPANESE,     // ja
    LANG_KOREAN,       // ko
    LANG_CHINESE,      // zh
    LANG_THAI,         // th
    LANG_ARABIC,       // ar
    LANG_HEBREW,       // he
    LANG_RUSSIAN,      // ru
    LANG_GREEK,        // el
    LANG_HINDI,        // hi
    LANG_TAMIL,        // ta
    LANG_TELUGU,       // te
    LANG_KANNADA,      // kn
    LANG_MALAYALAM,    // ml
    LANG_GUJARATI,     // gu
    LANG_BENGALI,      // bn
    LANG_PUNJABI,      // pa
    LANG_ORIYA,        // or
    LANG_MARATHI,      // mr
    LANG_NEPALI,       // ne
    LANG_SINHALA,      // si
    LANG_TIBETAN,      // bo
    LANG_GEORGIAN,     // ka
    LANG_ARMENIAN,     // hy
    LANG_UKRAINIAN,    // uk
    LANG_BULGARIAN,    // bg
    LANG_MACEDONIAN,   // mk
    LANG_SERBIAN,      // sr Cyrillic
    LANG_BELARUSIAN,   // be
    LANG_KAZAKH,       // kk
    LANG_AZERBAIJANI,  // az Cyrillic
    LANG_MONGOLIAN,    // mn
    LANG_PERSIAN,      // fa
    LANG_URDU,         // ur
    LANG_AMHARIC,      // am
};

InputLanguageMonitor& InputLanguageMonitor::Instance() {
    static InputLanguageMonitor instance;
    return instance;
}

bool InputLanguageMonitor::IsLatin(HKL hkl) {
    LANGID lang    = LOWORD(reinterpret_cast<UINT_PTR>(hkl));
    WORD   primary = PRIMARYLANGID(lang);
    for (LANGID p : kNonLatinPrimaries)
        if (primary == p) return false;
    return true;
}

void InputLanguageMonitor::OnInputLangChange(HKL newLayout) {
    bool latin = IsLatin(newLayout);
    if (latin == currentIsLatin_) return;
    currentIsLatin_ = latin;
    LogInfo(latin ? L"Input language: Latin" : L"Input language: Non-Latin (auto-disable)");
    EventBus::Instance().Post(AppEvent::InputLanguageChanged);
}

}  // namespace goxviet
