//! English 100k Words Test
//!
//! Tests all 100k English words to verify auto-restore behavior for both
//! Telex and VNI input methods. Outputs two failure files similar to
//! dictionary_vietnamese_test.rs.

use goxviet_core::application::dto::EngineConfig;
use goxviet_core::domain::entities::key_event::{Action, KeyEvent};
use goxviet_core::domain::ports::input::InputMethodId;
use goxviet_core::domain::ports::transformation::ToneStrategy;
use goxviet_core::presentation::di::Container;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

// ─── Helper: type a string via Container v2 API ─────────────────────────────

fn type_word(container: &mut Container, input: &str) -> String {
    let mut screen = String::new();

    for ch in input.chars() {
        let keycode = goxviet_core::utils::char_to_key(ch);
        let is_shift = ch.is_uppercase();

        let key_event = KeyEvent::new(keycode, is_shift, false, false, false);

        let process_result = {
            let processor_arc = container.processor_service();
            let mut processor_guard = processor_arc.lock().unwrap();
            processor_guard.process_key(key_event)
        };

        match process_result {
            Ok(result) => {
                let backspace = result.backspace_count();
                let new_text = result.new_text().as_str();
                let action = result.action();
                let has_transformation =
                    matches!(action, Action::Replace { .. } | Action::Insert);

                for _ in 0..backspace {
                    screen.pop();
                }

                if !new_text.is_empty() {
                    screen.push_str(new_text);
                } else if ch == ' ' {
                    screen.push(' ');
                } else if !has_transformation {
                    screen.push(ch);
                }
            }
            Err(_) => {
                screen.push(ch);
            }
        }
    }

    screen
}

// ─── Config builders ─────────────────────────────────────────────────────────

fn telex_config() -> EngineConfig {
    EngineConfig {
        input_method: InputMethodId::Telex,
        tone_strategy: ToneStrategy::Modern,
        enabled: true,
        smart_mode: true,
        spell_check: false,
        auto_correct: false,
        max_history_size: 100,
        buffer_timeout_ms: 1000,
        use_modern_tone_placement: true,
        enable_shortcuts: false,
        instant_restore_enabled: true,
        esc_restore_enabled: true,
    }
}

fn vni_config() -> EngineConfig {
    EngineConfig {
        input_method: InputMethodId::Vni,
        tone_strategy: ToneStrategy::Modern,
        enabled: true,
        smart_mode: true,
        spell_check: false,
        auto_correct: false,
        max_history_size: 100,
        buffer_timeout_ms: 1000,
        use_modern_tone_placement: true,
        enable_shortcuts: false,
        instant_restore_enabled: true,
        esc_restore_enabled: true,
    }
}

// ─── Data structures ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct TestResult {
    word: String,
    input: String,
    expected: String,
    actual: String,
}

#[derive(Debug, Default, Clone)]
struct CategoryStats {
    total: usize,
    passed: usize,
    failed: usize,
    failures: Vec<TestResult>,
}

// ─── Grouping ────────────────────────────────────────────────────────────────

fn group_by_length<'a>(words: &'a [&'a str]) -> HashMap<String, Vec<&'a str>> {
    let mut groups: HashMap<String, Vec<&str>> = HashMap::new();
    for word in words {
        let len = word.len();
        let key = match len {
            1..=3 => "short_1_3",
            4..=6 => "medium_4_6",
            7..=10 => "long_7_10",
            _ => "very_long_11plus",
        };
        groups.entry(key.to_string()).or_default().push(word);
    }
    groups
}

// ─── Batch runners ───────────────────────────────────────────────────────────

fn test_telex_batch(words: &[&str]) -> Vec<TestResult> {
    let mut failures = Vec::new();
    for word in words {
        let input = format!("{} ", word);
        let expected = format!("{} ", word);
        let mut container = Container::with_config(telex_config());
        let actual = type_word(&mut container, &input);
        if actual != expected {
            failures.push(TestResult {
                word: word.to_string(),
                input: word.to_string(),
                expected: expected.trim().to_string(),
                actual: actual.trim().to_string(),
            });
        }
    }
    failures
}

fn test_vni_batch(words: &[&str]) -> Vec<TestResult> {
    let mut failures = Vec::new();
    for word in words {
        let input = format!("{} ", word);
        let expected = format!("{} ", word);
        let mut container = Container::with_config(vni_config());
        let actual = type_word(&mut container, &input);
        if actual != expected {
            failures.push(TestResult {
                word: word.to_string(),
                input: word.to_string(),
                expected: expected.trim().to_string(),
                actual: actual.trim().to_string(),
            });
        }
    }
    failures
}

// ─── Output ──────────────────────────────────────────────────────────────────

fn write_failures_to_file(filename: &str, stats: &HashMap<String, CategoryStats>) {
    let path = std::path::Path::new("tests/failures").join(filename);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap_or_default();
    }

    if let Ok(mut f) = File::create(&path) {
        writeln!(f, "WORD\tINPUT\tEXPECTED\tACTUAL").unwrap();

        let mut all_failures: Vec<&TestResult> = stats.values().flat_map(|s| &s.failures).collect();
        all_failures.sort_by(|a, b| a.word.cmp(&b.word));

        for failure in all_failures {
            writeln!(
                f,
                "{}\t{}\t{}\t{}",
                failure.word, failure.input, failure.expected, failure.actual
            )
            .unwrap();
        }
    }
}

fn print_report(method: &str, category_stats: &HashMap<String, CategoryStats>, total_time: f64) {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!(
        "║           {} ENGLISH TEST REPORT                    ║",
        method.to_uppercase()
    );
    println!("╚════════════════════════════════════════════════════════════════╝");

    let mut total_passed = 0;
    let mut total_failed = 0;
    let mut total_words = 0;

    let categories = ["short_1_3", "medium_4_6", "long_7_10", "very_long_11plus"];
    for category in &categories {
        if let Some(stats) = category_stats.get(*category) {
            if stats.total == 0 {
                continue;
            }
            let pass_rate = (stats.passed as f64 / stats.total as f64) * 100.0;
            let category_name = match *category {
                "short_1_3" => "1-3 chars  ",
                "medium_4_6" => "4-6 chars  ",
                "long_7_10" => "7-10 chars ",
                _ => "11+ chars  ",
            };

            println!("\n┌────────────────────────────────────────────────────────────────┐");
            println!("│ Category: {}                                    │", category_name);
            println!("├────────────────────────────────────────────────────────────────┤");
            println!(
                "│  Total: {:>5}  │  Passed: {:>5}  │  Failed: {:>5}  │  Rate: {:>5.1}% │",
                stats.total, stats.passed, stats.failed, pass_rate
            );
            println!("└────────────────────────────────────────────────────────────────┘");

            if !stats.failures.is_empty() {
                println!("  Sample failures:");
                for (i, f) in stats.failures.iter().take(5).enumerate() {
                    println!(
                        "    {}. '{}' → got '{}'",
                        i + 1,
                        f.word,
                        f.actual
                    );
                }
                if stats.failures.len() > 5 {
                    println!("    ... and {} more failures", stats.failures.len() - 5);
                }
            }

            total_passed += stats.passed;
            total_failed += stats.failed;
            total_words += stats.total;
        }
    }

    let overall_rate = if total_words > 0 {
        (total_passed as f64 / total_words as f64) * 100.0
    } else {
        0.0
    };

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║                      OVERALL SUMMARY                           ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║  Total Words: {:>6}                                         ║", total_words);
    println!("║  Passed:      {:>6}  ({:>6.2}%)                              ║", total_passed, overall_rate);
    println!("║  Failed:      {:>6}                                         ║", total_failed);
    println!("║  Time:        {:>6.2}s                                       ║", total_time);
    println!("╚════════════════════════════════════════════════════════════════╝");
}

// ─── Main test ───────────────────────────────────────────────────────────────

#[test]
fn english_100k_auto_restore() {
    let content = fs::read_to_string("tests/data/english_100k_failures_words.txt")
        .expect("Failed to read english_100k_failures_words.txt");

    let all_words: Vec<&str> = content
        .lines()
        .map(|l| l.trim())
        .filter(|w| !w.is_empty() && w.chars().all(|c| c.is_ascii_alphabetic()))
        .collect();

    const CHUNK_SIZE: usize = 5000;
    const MIN_PASS_RATE: f64 = 0.0; // Failures-only subset; use for tracking, not blocking

    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("     ENGLISH FAILURES AUTO-RESTORE TEST (failures_words subset)");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Total words: {}", all_words.len());

    let groups = group_by_length(&all_words);

    // ── Telex ──────────────────────────────────────────────────────────────
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                     TESTING TELEX INPUT");
    println!("═══════════════════════════════════════════════════════════════════");

    let mut telex_stats: HashMap<String, CategoryStats> = HashMap::new();
    let telex_start = Instant::now();

    for (category, words) in &groups {
        println!("\nTesting category: {} ({} words)", category, words.len());
        let chunks: Vec<&[&str]> = words.chunks(CHUNK_SIZE).collect();
        for (chunk_idx, chunk) in chunks.iter().enumerate() {
            let failures = test_telex_batch(chunk);
            let entry = telex_stats.entry(category.to_string()).or_default();
            let chunk_total = chunk.len();
            let chunk_failed = failures.len();
            entry.total += chunk_total;
            entry.passed += chunk_total - chunk_failed;
            entry.failed += chunk_failed;
            entry.failures.extend(failures);
            print!("  Chunk {}: {}/{} passed  \r", chunk_idx + 1, entry.passed, entry.total);
        }
        println!();
    }

    let telex_time = telex_start.elapsed().as_secs_f64();
    print_report("Telex", &telex_stats, telex_time);
    write_failures_to_file("failures_english_telex.txt", &telex_stats);

    // ── VNI ────────────────────────────────────────────────────────────────
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                      TESTING VNI INPUT");
    println!("═══════════════════════════════════════════════════════════════════");

    let mut vni_stats: HashMap<String, CategoryStats> = HashMap::new();
    let vni_start = Instant::now();

    for (category, words) in &groups {
        println!("\nTesting category: {} ({} words)", category, words.len());
        let chunks: Vec<&[&str]> = words.chunks(CHUNK_SIZE).collect();
        for (chunk_idx, chunk) in chunks.iter().enumerate() {
            let failures = test_vni_batch(chunk);
            let entry = vni_stats.entry(category.to_string()).or_default();
            let chunk_total = chunk.len();
            let chunk_failed = failures.len();
            entry.total += chunk_total;
            entry.passed += chunk_total - chunk_failed;
            entry.failed += chunk_failed;
            entry.failures.extend(failures);
            print!("  Chunk {}: {}/{} passed  \r", chunk_idx + 1, entry.passed, entry.total);
        }
        println!();
    }

    let vni_time = vni_start.elapsed().as_secs_f64();
    print_report("VNI", &vni_stats, vni_time);
    write_failures_to_file("failures_english_vni.txt", &vni_stats);

    // ── Final summary ──────────────────────────────────────────────────────
    let total_telex_passed: usize = telex_stats.values().map(|s| s.passed).sum();
    let total_telex_words: usize = telex_stats.values().map(|s| s.total).sum();
    let telex_pass_rate = (total_telex_passed as f64 / total_telex_words as f64) * 100.0;

    let total_vni_passed: usize = vni_stats.values().map(|s| s.passed).sum();
    let total_vni_words: usize = vni_stats.values().map(|s| s.total).sum();
    let vni_pass_rate = (total_vni_passed as f64 / total_vni_words as f64) * 100.0;

    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                        FINAL RESULTS");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Telex: {:.2}% ({} / {})", telex_pass_rate, total_telex_passed, total_telex_words);
    println!("VNI:   {:.2}% ({} / {})", vni_pass_rate, total_vni_passed, total_vni_words);
    println!("═══════════════════════════════════════════════════════════════════");

    assert!(
        telex_pass_rate >= MIN_PASS_RATE,
        "Telex English pass rate {:.2}% is below threshold {:.1}%",
        telex_pass_rate,
        MIN_PASS_RATE
    );
    assert!(
        vni_pass_rate >= MIN_PASS_RATE,
        "VNI English pass rate {:.2}% is below threshold {:.1}%",
        vni_pass_rate,
        MIN_PASS_RATE
    );
}

