// dictionary.rs
//
// DEVELOPER NOTE: Dictionary hashes are calculated using macOS virtual keycodes.
// Algorithm: hash = (hash * 31) + keycode
//
// Common macOS Keycodes:
// A=0, S=1, D=2, F=3, H=4, G=5, Z=6, X=7, C=8, V=9, B=11, Q=12, W=13, E=14, R=15, Y=16, T=17
// O=31, U=32, I=34, P=35, L=37, J=38, K=40, N=45, M=46
//
// To add a new word:
// 1. Get the keycode sequence (e.g., "the" -> [17, 4, 14]) 16475
// 2. Calculate hash: ((17 * 31 + 4) * 31 + 14) = 16475
// 3. Add to the corresponding check_lenN function.

/// Optimized dictionary for common words
/// Uses a simple static hash table for O(1) lookup
pub struct Dictionary;

impl Dictionary {
    /// Check if a sequence of keys is a common word (any language)
    #[inline]
    pub fn is_common(keys: &[u16]) -> bool {
        Self::is_english(keys) || Self::is_vietnamese(keys)
    }

    /// Check if a sequence of keys is a common English word
    #[inline]
    pub fn is_english(keys: &[u16]) -> bool {
        let (len, hash) = Self::get_hash(keys);
        match len {
            2 => Self::is_english_len2(hash),
            3 => Self::is_english_len3(hash),
            4 => Self::is_english_len4(hash),
            5 => Self::is_english_len5(hash),
            6 => Self::is_english_len6(hash),
            7 => Self::is_english_len7(hash),
            8 => Self::is_english_len8(hash),
            11 => Self::is_english_len11(hash),
            _ => false,
        }
    }

    /// Check if a sequence of keys is a common Vietnamese word (raw ASCII form)
    #[inline]
    pub fn is_vietnamese(keys: &[u16]) -> bool {
        let (len, hash) = Self::get_hash(keys);
        match len {
            2 => Self::is_vietnamese_len2(hash),
            3 => Self::is_vietnamese_len3(hash),
            4 => Self::is_vietnamese_len4(hash),
            5 => Self::is_vietnamese_len5(hash),
            6 => Self::is_vietnamese_len6(hash),
            7 => Self::is_vietnamese_len7(hash),
            _ => false,
        }
    }

    #[inline(always)]
    fn get_hash(keys: &[u16]) -> (usize, u32) {
        let len = keys.len();
        if len == 0 || len > 11 {
            return (0, 0);
        }

        // Unrolled hash loop for better performance
        let mut hash = 0u32;
        for &k in keys {
            hash = hash.wrapping_mul(31).wrapping_add(k as u32);
        }
        (len, hash)
    }

    #[inline(always)]
    fn is_english_len2(hash: u32) -> bool {
        hash == 417 || hash == 357 // we, by
    }

    #[inline(always)]
    fn is_vietnamese_len2(hash: u32) -> bool {
        matches!(hash, 1147 | 375 | 279 | 96 | 8 | 976 | 62 | 561 | 559 | 313)
    }

    #[inline(always)]
    fn is_english_len3(hash: u32) -> bool {
        matches!(
            hash,
            1397 | 3859
                | 479
                | 11580
                | 44223
                | 16369
                | 1184
                | 1411
                | 7733
                | 3846
                | 12494
                | 31200
                | 30798
                | 30800
                | 1938
                | 5256
                | 43692
                | 301240
        )
    }

    #[inline(always)]
    fn is_vietnamese_len3(hash: u32) -> bool {
        matches!(
            hash,
            7843 | 8694
                | 9644
                | 43261
                | 1399
                | 45184
                | 12524
                | 31191
                | 45267
                | 11539
                | 9706
                | 1402
                | 32152
                | 31200
        )
    }

    #[inline(always)]
    fn is_english_len4(hash: u32) -> bool {
        matches!(
            hash,
            512404
                | 138542
                | 528854
                | 528931
                | 114676
                | 528846
                | 417217
                | 348575
                | 161245
                | 1441634
                | 1426014
                | 283546
                | 506518
                | 301240
                | 460367
                | 342563
                | 74444
                | 46575
                | 5256
                | 242619
                | 954722  // user
                | 1073472 // push
        )
    }

    #[inline(always)]
    fn is_vietnamese_len4(hash: u32) -> bool {
        matches!(hash, 283545 | 1426015 | 43260 | 45315 | 1394174 | 967205)
    }

    #[inline(always)]
    fn is_english_len5(hash: u32) -> bool {
        matches!(
            hash,
            16388432
                | 16388414
                | 1365045
                | 16053307
                | 15973442
                | 81543015
                | 1441715
                | 15973742
                | 8354982
                | 42483933
                | 16152929  // telex
                | 42913624 // merge
        )
    }

    #[inline(always)]
    fn is_vietnamese_len5(hash: u32) -> bool {
        matches!(hash, 31180485 | 47000451 | 15151515 | 15832955 | 16726684)
    }

    #[inline(always)]
    fn is_english_len6(hash: u32) -> bool {
        matches!(
            hash,
            508041415 | 440330456 | 503023077 | 902375294 | 442491930 |
            442368526 | 1031664297 | 1015398411 | 41560412 |
            344997123 | 258186149 | 1072239639 | 1016887961 |
            1318312728 | 143594016 | 271039914 | 144100463 |
            44789359 |   // stress
            1016946058 | // import
            1036607681 | // please
            408345719 // export
        )
    }

    #[inline(always)]
    fn is_vietnamese_len6(hash: u32) -> bool {
        matches!(
            hash,
            508041415 | 440330456 | 503023077 | 289493796 | 518525947
        )
    }

    #[inline(always)]
    fn is_vietnamese_len7(hash: u32) -> bool {
        hash == 3189400552 || hash == 1145497276
    }

    #[inline(always)]
    fn is_english_len7(hash: u32) -> bool {
        matches!(
            hash,
            829921679 | 863048837 | 436977564 | // restore, release, archive
            1460095212 | // improve
            837204664 |  // reverse
            1456175993 | // process
            4068304585 | // express
            59565695 // address
        )
    }

    #[inline(always)]
    fn is_english_len8(hash: u32) -> bool {
        matches!(
            hash,
            840296816 | 1454329886 | // personal, homebrew
            965955224 // generate
        )
    }

    #[inline(always)]
    fn is_english_len11(hash: u32) -> bool {
        hash == 2134251024 // improvement
    }
}
