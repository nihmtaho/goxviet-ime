#[cfg(test)]
mod test {
    #[test]
    fn test_baau() {
        use goxviet_core::application::dto::EngineConfig;
        use goxviet_core::domain::entities::key_event::{Action, KeyEvent};
        use goxviet_core::domain::ports::input::InputMethodId;
        use goxviet_core::domain::ports::transformation::ToneStrategy;
        use goxviet_core::presentation::di::Container;
        
        let config = EngineConfig {
            input_method: InputMethodId::Telex,
            tone_strategy: ToneStrategy::Modern,
            enabled: true, smart_mode: true, spell_check: false, auto_correct: false,
            max_history_size: 100, buffer_timeout_ms: 1000,
            use_modern_tone_placement: true, enable_shortcuts: false,
            instant_restore_enabled: true, esc_restore_enabled: true,
        };
        let mut container = Container::new();
        container.update_config(config);
        let mut screen = String::new();
        for ch in "soong ".chars() {
            let keycode = goxviet_core::utils::char_to_key(ch);
            let key_event = KeyEvent::new(keycode, false, false, false, false);
            let result = { let p = container.processor_service(); let mut g = p.lock().unwrap(); g.process_key(key_event) };
            match result {
                Ok(r) => {
                    let bs = r.backspace_count();
                    let txt = r.new_text().as_str().to_string();
                    let act = format!("{:?}", r.action());
                    eprintln!("key={} bs={} text={:?} action={}", ch, bs, txt, act);
                    for _ in 0..bs { screen.pop(); }
                    if !txt.is_empty() { screen.push_str(&txt); }
                    else { screen.push(ch); }
                }
                Err(_) => { screen.push(ch); }
            }
            eprintln!("  screen: {}", screen);
        }
        assert_eq!(screen.trim(), "sông");
    }
}

    #[test]
    fn test_hoax_debug() {
        use goxviet_core::application::dto::EngineConfig;
        use goxviet_core::domain::entities::key_event::{Action, KeyEvent};
        use goxviet_core::domain::ports::input::InputMethodId;
        use goxviet_core::domain::ports::transformation::ToneStrategy;
        use goxviet_core::presentation::di::Container;
        
        let config = EngineConfig {
            input_method: InputMethodId::Telex,
            tone_strategy: ToneStrategy::Modern,
            enabled: true, smart_mode: true, spell_check: false, auto_correct: false,
            max_history_size: 100, buffer_timeout_ms: 1000,
            use_modern_tone_placement: true, enable_shortcuts: false,
            instant_restore_enabled: true, esc_restore_enabled: true,
        };
        let mut container = Container::new();
        container.update_config(config);
        let mut screen = String::new();
        for ch in "hoax".chars() {
            let keycode = goxviet_core::utils::char_to_key(ch);
            let key_event = KeyEvent::new(keycode, false, false, false, false);
            let result = { let p = container.processor_service(); let mut g = p.lock().unwrap(); g.process_key(key_event) };
            match result {
                Ok(r) => {
                    let bs = r.backspace_count();
                    let txt = r.new_text().as_str().to_string();
                    let act = format!("{:?}", r.action());
                    eprintln!("key={} bs={} text={:?} action={}", ch, bs, txt, act);
                    for _ in 0..bs { screen.pop(); }
                    if !txt.is_empty() { screen.push_str(&txt); }
                    else { screen.push(ch); }
                }
                Err(_) => { screen.push(ch); }
            }
            eprintln!("  screen: {:?}", screen);
        }
        assert_eq!(screen, "hoã", "hoax should produce hoã");
    }

    #[test]
    fn test_tone_compound_debug() {
        use goxviet_core::application::dto::EngineConfig;
        use goxviet_core::domain::entities::key_event::{Action, KeyEvent};
        use goxviet_core::domain::ports::input::InputMethodId;
        use goxviet_core::domain::ports::transformation::ToneStrategy;
        use goxviet_core::presentation::di::Container;
        
        let config = EngineConfig {
            input_method: InputMethodId::Telex,
            tone_strategy: ToneStrategy::Modern,
            enabled: true, smart_mode: true, spell_check: false, auto_correct: false,
            max_history_size: 100, buffer_timeout_ms: 1000,
            use_modern_tone_placement: true, enable_shortcuts: false,
            instant_restore_enabled: true, esc_restore_enabled: true,
        };
        for (expected, input) in &[("quyển", "quyenr "), ("nguyễn", "nguyenx "), ("miễn", "mienx ")] {
            let mut container = Container::new();
            container.update_config(config.clone());
            let mut screen = String::new();
            for ch in input.chars() {
                let keycode = goxviet_core::utils::char_to_key(ch);
                let key_event = KeyEvent::new(keycode, false, false, false, false);
                let result = { let p = container.processor_service(); let mut g = p.lock().unwrap(); g.process_key(key_event) };
                match result {
                    Ok(r) => {
                        let bs = r.backspace_count();
                        let txt = r.new_text().as_str().to_string();
                        for _ in 0..bs { screen.pop(); }
                        if !txt.is_empty() { screen.push_str(&txt); }
                        else { screen.push(ch); }
                    }
                    Err(_) => { screen.push(ch); }
                }
            }
            eprintln!("input={} expected={} got={}", input.trim(), expected, screen.trim());
        }
    }

    fn type_word_trace(input: &str) -> String {
        use goxviet_core::application::dto::EngineConfig;
        use goxviet_core::domain::entities::key_event::{Action, KeyEvent};
        use goxviet_core::domain::ports::input::InputMethodId;
        use goxviet_core::domain::ports::transformation::ToneStrategy;
        use goxviet_core::presentation::di::Container;
        let config = EngineConfig {
            input_method: InputMethodId::Telex, tone_strategy: ToneStrategy::Modern,
            enabled: true, smart_mode: true, spell_check: false, auto_correct: false,
            max_history_size: 100, buffer_timeout_ms: 1000,
            use_modern_tone_placement: true, enable_shortcuts: false,
            instant_restore_enabled: true, esc_restore_enabled: true,
        };
        let mut container = Container::new();
        container.update_config(config);
        let mut screen = String::new();
        for ch in format!("{} ", input).chars() {
            let keycode = goxviet_core::utils::char_to_key(ch);
            let key_event = KeyEvent::new(keycode, ch.is_uppercase(), false, false, false);
            let result = { let p = container.processor_service(); let mut g = p.lock().unwrap(); g.process_key(key_event) };
            match result {
                Ok(r) => {
                    let bs = r.backspace_count(); let txt = r.new_text().as_str().to_string();
                    eprintln!("  key={:?} bs={} text={:?}", ch, bs, txt);
                    for _ in 0..bs { screen.pop(); }
                    if !txt.is_empty() { screen.push_str(&txt); } else { screen.push(ch); }
                }
                Err(_) => { screen.push(ch); }
            }
            eprintln!("    screen={:?}", screen);
        }
        screen.trim().to_string()
    }

    #[test]
    fn test_failure_cases_trace() {
        let cases = &[
            ("cõng", "coxng"), ("rích", "risch"), ("ích", "isch"), ("ạch", "ajch"),
            ("yard", "yard"), ("khuýp", "khuyps"), ("khuýp", "khuysp"),
            ("quáu", "quaus"), ("tuýp", "tuyps"),
        ];
        for (expected, input) in cases {
            eprintln!("\n--- {} ({}) ---", expected, input);
            let got = type_word_trace(input);
            eprintln!("RESULT: got={:?} expected={:?} ok={}", got, expected, got == *expected);
        }
    }

    #[test]
    fn test_veen_giem_trace() {
        let cases = &[
            ("vên", "veen"),
            ("gièm", "giemf"),
            ("giẹp", "giepj"),
        ];
        for (expected, input) in cases {
            eprintln!("\n--- {} ({}) ---", expected, input);
            let got = type_word_trace(input);
            eprintln!("RESULT: got={:?} expected={:?} ok={}", got, expected, got == *expected);
        }
    }
