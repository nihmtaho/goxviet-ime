//! Build script: generate Vietnamese dictionary data at compile time.
//!
//! Produces three artifacts in OUT_DIR:
//! - `viet_syllables.rs`: phf::Set<&'static str> for TuDien.txt (~7K syllables, O(1))
//! - `viet_compound.bin`: sorted UTF-8 binary for TuDienTuGhep.txt (~68K phrases, O(log n))
//! - `double_consonant_words.rs`: phf::Set<&'static str> for double_consonant_words.txt (O(1))

use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR must be set by Cargo");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set");

    let tu_dien_path = Path::new(&manifest_dir).join("data/TuDien.dic");
    let tu_ghep_path = Path::new(&manifest_dir).join("data/TuDienTuGhep.dic");
    let double_consonant_path =
        Path::new(&manifest_dir).join("src/data/double_consonant_words.txt");
    let keep_english_path = Path::new(&manifest_dir).join("data/keep_english.dic");

    // Tell Cargo to re-run if dictionary files change
    println!("cargo:rerun-if-changed=data/TuDien.dic");
    println!("cargo:rerun-if-changed=data/TuDienTuGhep.dic");
    println!("cargo:rerun-if-changed=src/data/double_consonant_words.txt");
    println!("cargo:rerun-if-changed=data/keep_english.dic");

    generate_viet_syllables(&tu_dien_path, &out_dir);
    generate_viet_compound_bin(&tu_ghep_path, &out_dir);
    generate_double_consonant_words(&double_consonant_path, &out_dir);
    generate_keep_english(&keep_english_path, &out_dir);
}

/// Generate phf::Set<&'static str> for single-syllable Vietnamese words.
/// Output: OUT_DIR/viet_syllables.rs
fn generate_viet_syllables(path: &Path, out_dir: &str) {
    let file = File::open(path).expect("Cannot open TuDien.txt");
    let reader = BufReader::new(file);

    let mut entries: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read TuDien.txt line");
        let entry = line.trim().to_string();
        if !entry.is_empty() {
            entries.push(entry);
        }
    }

    let out_path = Path::new(out_dir).join("viet_syllables.rs");
    let mut out = File::create(&out_path).expect("Cannot create viet_syllables.rs");

    // Generate phf::Set using phf_codegen
    let mut set_builder = phf_codegen::Set::new();
    for entry in &entries {
        set_builder.entry(entry.as_str());
    }

    write!(
        out,
        "/// Vietnamese single-syllable dictionary (~{} entries).
/// Generated at build time from data/TuDien.txt.
/// O(1) lookup via perfect hash function.
static VIET_SYLLABLES: phf::Set<&'static str> = {};",
        entries.len(),
        set_builder.build()
    )
    .expect("Failed to write viet_syllables.rs");

    eprintln!(
        "cargo:warning=Generated VIET_SYLLABLES with {} entries",
        entries.len()
    );
}

/// Generate sorted UTF-8 binary for Vietnamese compound phrases.
///
/// Binary format (little-endian):
/// - Each entry: [u16 byte_len][UTF-8 bytes]
/// - Entries sorted by UTF-8 byte comparison (NFC Unicode order)
///
/// Output: OUT_DIR/viet_compound.bin
fn generate_viet_compound_bin(path: &Path, out_dir: &str) {
    let file = File::open(path).expect("Cannot open TuDienTuGhep.txt");
    let reader = BufReader::new(file);

    let mut entries: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read TuDienTuGhep.txt line");
        let entry = line.trim().to_string();
        if !entry.is_empty() {
            entries.push(entry);
        }
    }

    // Sort by UTF-8 bytes for binary search
    entries.sort_unstable();
    entries.dedup(); // remove any duplicates

    let out_path = Path::new(out_dir).join("viet_compound.bin");
    let mut out = File::create(&out_path).expect("Cannot create viet_compound.bin");

    // Write header: u32 entry count (little-endian)
    let count = entries.len() as u32;
    out.write_all(&count.to_le_bytes())
        .expect("Failed to write count");

    // Write offset table: u32 offsets from start of data section
    // First pass: compute offsets
    let mut offsets: Vec<u32> = Vec::with_capacity(entries.len());
    let mut offset: u32 = 0;
    for entry in &entries {
        offsets.push(offset);
        // 2 bytes length + entry bytes
        offset += 2 + entry.len() as u32;
    }

    for off in &offsets {
        out.write_all(&off.to_le_bytes())
            .expect("Failed to write offset");
    }

    // Second pass: write entries
    for entry in &entries {
        let bytes = entry.as_bytes();
        let len = bytes.len() as u16;
        out.write_all(&len.to_le_bytes())
            .expect("Failed to write entry length");
        out.write_all(bytes).expect("Failed to write entry bytes");
    }

    let file_size = fs::metadata(&out_path).map(|m| m.len()).unwrap_or(0);

    eprintln!(
        "cargo:warning=Generated viet_compound.bin: {} entries, {} bytes ({:.0}KB)",
        entries.len(),
        file_size,
        file_size as f64 / 1024.0
    );
}

/// Generate phf::Set<&'static str> for English words with Telex tone-marker double consonants.
/// Used for triple-tone auto-correction at SPACE boundary (e.g. "assset" → "asset").
/// Output: OUT_DIR/double_consonant_words.rs
fn generate_double_consonant_words(path: &Path, out_dir: &str) {
    let file = File::open(path).expect("Cannot open double_consonant_words.txt");
    let reader = BufReader::new(file);

    let mut entries: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read double_consonant_words.txt line");
        let entry = line.trim().to_string();
        if !entry.is_empty() {
            entries.push(entry);
        }
    }

    let out_path = Path::new(out_dir).join("double_consonant_words.rs");
    let mut out = File::create(&out_path).expect("Cannot create double_consonant_words.rs");

    let mut set_builder = phf_codegen::Set::new();
    for entry in &entries {
        set_builder.entry(entry.as_str());
    }

    write!(
        out,
        "/// English words containing Telex tone-marker double consonants (~{} entries).
/// Generated at build time from src/data/double_consonant_words.txt.
/// O(1) lookup via perfect hash function.
static DOUBLE_CONSONANT_WORDS: phf::Set<&'static str> = {};",
        entries.len(),
        set_builder.build()
    )
    .expect("Failed to write double_consonant_words.rs");

    eprintln!(
        "cargo:warning=Generated DOUBLE_CONSONANT_WORDS with {} entries",
        entries.len()
    );
}

/// Generate phf::Set<&'static str> for short English words corrupted by Telex/VNI.
/// Output: OUT_DIR/keep_english.rs
fn generate_keep_english(path: &Path, out_dir: &str) {
    let file = File::open(path).expect("Cannot open keep_english.txt");
    let reader = BufReader::new(file);
    let words: Vec<String> = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.trim().to_lowercase())
        .filter(|l| !l.is_empty())
        .collect();

    let out_path = Path::new(out_dir).join("keep_english.rs");
    let mut out = File::create(&out_path).expect("Cannot create keep_english.rs");

    let mut set_builder = phf_codegen::Set::new();
    for w in &words {
        set_builder.entry(w.as_str());
    }

    write!(
        out,
        "/// Short English words that Telex/VNI tone modifiers corrupt (~{} entries).
/// Generated at build time from data/keep_english.txt.
/// O(1) lookup via perfect hash function.
static KEEP_ENGLISH: phf::Set<&'static str> = {};",
        words.len(),
        set_builder.build()
    )
    .expect("Failed to write keep_english.rs");

    eprintln!(
        "cargo:warning=Generated KEEP_ENGLISH with {} entries",
        words.len()
    );
}
