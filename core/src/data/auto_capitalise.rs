/// Abbreviation list for auto-capitalise exclusions.
///
/// When auto-capitalise is enabled, the engine must NOT capitalise the next letter
/// after any of these abbreviations followed by a period and space.
///
/// Stored as lowercase for case-insensitive matching. All entries end with '.'.
pub static ABBREVIATION_LIST: &[&str] = &[
    "bs.",    // bác sĩ
    "đ.",     // đồng (currency)
    "gs.",    // giáo sư
    "no.",    // number
    "pgs.",   // phó giáo sư
    "pgsts.", // phó giáo sư tiến sĩ
    "ths.",   // thạc sĩ
    "tp.",    // thành phố
    "tr.",    // trang
    "ts.",    // tiến sĩ
    "v.d.",   // ví dụ
    "v.v.",   // và vân vân (etc.)
];

/// Returns true if the given token (in any case) matches a known abbreviation.
/// The token should already end with '.'.
pub fn is_abbreviation(token: &str) -> bool {
    let lower = token.to_lowercase();
    ABBREVIATION_LIST.contains(&lower.as_str())
}
