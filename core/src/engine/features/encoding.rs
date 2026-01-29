//! Multi-Encoding Output Support
//!
//! Converts Vietnamese text to different legacy encodings:
//! - Unicode (UTF-8/UTF-16) - default
//! - TCVN3 (Vietnamese legacy encoding)
//! - VNI (VNI Windows encoding)
//! - CP1258 (Windows Vietnam codepage)

pub use OutputEncoding as Encoding;

/// A freestanding function to convert a string to a specific encoding.
pub fn convert_to_encoding(s: &str, encoding: Encoding) -> Vec<u8> {
    let mut converter = EncodingConverter::new();
    converter.set_encoding(encoding);
    converter.convert_string(s)
}

/// Output encoding types
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum OutputEncoding {
    /// Unicode (UTF-8/UTF-16) - default, no conversion needed
    #[default]
    Unicode,
    /// TCVN3 - Vietnamese legacy encoding (single-byte)
    TCVN3,
    /// VNI - VNI Windows encoding
    VNI,
    /// CP1258 (Windows-1258) - Windows Vietnam codepage
    CP1258,
}

impl OutputEncoding {
    /// Convert from integer (for FFI)
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => OutputEncoding::Unicode,
            1 => OutputEncoding::TCVN3,
            2 => OutputEncoding::VNI,
            3 => OutputEncoding::CP1258,
            _ => OutputEncoding::Unicode,
        }
    }

    /// Convert to integer (for FFI)
    pub fn to_u8(self) -> u8 {
        match self {
            OutputEncoding::Unicode => 0,
            OutputEncoding::TCVN3 => 1,
            OutputEncoding::VNI => 2,
            OutputEncoding::CP1258 => 3,
        }
    }
}

/// Encoding converter for Vietnamese text
#[derive(Debug, Default)]
pub struct EncodingConverter {
    encoding: OutputEncoding,
}

impl EncodingConverter {
    pub fn new() -> Self {
        Self {
            encoding: OutputEncoding::Unicode,
        }
    }

    /// Const constructor for static initialization
    pub const fn new_const() -> Self {
        Self {
            encoding: OutputEncoding::Unicode,
        }
    }

    /// Set output encoding
    pub fn set_encoding(&mut self, encoding: OutputEncoding) {
        self.encoding = encoding;
    }

    /// Get current encoding
    pub fn encoding(&self) -> OutputEncoding {
        self.encoding
    }

    /// Convert a Unicode Vietnamese character to target encoding
    ///
    /// Returns the converted bytes. For Unicode, returns the UTF-8 bytes.
    /// For legacy encodings, returns single-byte or multi-byte sequence.
    pub fn convert_char(&self, ch: char) -> Vec<u8> {
        match self.encoding {
            OutputEncoding::Unicode => {
                let mut buf = [0u8; 4];
                ch.encode_utf8(&mut buf).as_bytes().to_vec()
            }
            OutputEncoding::TCVN3 => self.to_tcvn3(ch),
            OutputEncoding::VNI => self.to_vni(ch),
            OutputEncoding::CP1258 => self.to_cp1258(ch),
        }
    }

    /// Convert a full string to target encoding
    pub fn convert_string(&self, s: &str) -> Vec<u8> {
        if self.encoding == OutputEncoding::Unicode {
            return s.as_bytes().to_vec();
        }

        let mut result = Vec::with_capacity(s.len() * 2);
        for ch in s.chars() {
            result.extend(self.convert_char(ch));
        }
        result
    }

    /// Convert Unicode Vietnamese character to TCVN3
    ///
    /// TCVN3 is a single-byte encoding where Vietnamese diacritics are
    /// mapped to specific byte values in the 128-255 range.
    fn to_tcvn3(&self, ch: char) -> Vec<u8> {
        // TCVN3 mapping for common Vietnamese characters
        // This is a subset - full implementation would cover all 134 Vietnamese chars
        let byte = match ch {
            // Lowercase vowels with diacritics
            'à' => 0xB5,
            'á' => 0xB8,
            'ả' => 0xB6,
            'ã' => 0xB7,
            'ạ' => 0xB9,
            'ă' => 0xBE,
            'ằ' => 0xBF,
            'ắ' => 0xC1,
            'ẳ' => 0xC0,
            'ẵ' => 0xC2,
            'ặ' => 0xC3,
            'â' => 0xC4,
            'ầ' => 0xC5,
            'ấ' => 0xC7,
            'ẩ' => 0xC6,
            'ẫ' => 0xC8,
            'ậ' => 0xC9,
            'è' => 0xCC,
            'é' => 0xCE,
            'ẻ' => 0xCD,
            'ẽ' => 0xCF,
            'ẹ' => 0xD0,
            'ê' => 0xD1,
            'ề' => 0xD2,
            'ế' => 0xD4,
            'ể' => 0xD3,
            'ễ' => 0xD5,
            'ệ' => 0xD6,
            'ì' => 0xD7,
            'í' => 0xD9,
            'ỉ' => 0xD8,
            'ĩ' => 0xDA,
            'ị' => 0xDB,
            'ò' => 0xDC,
            'ó' => 0xDE,
            'ỏ' => 0xDD,
            'õ' => 0xDF,
            'ọ' => 0xE0,
            'ô' => 0xE1,
            'ồ' => 0xE2,
            'ố' => 0xE4,
            'ổ' => 0xE3,
            'ỗ' => 0xE5,
            'ộ' => 0xE6,
            'ơ' => 0xE7,
            'ờ' => 0xE8,
            'ớ' => 0xEA,
            'ở' => 0xE9,
            'ỡ' => 0xEB,
            'ợ' => 0xEC,
            'ù' => 0xED,
            'ú' => 0xEF,
            'ủ' => 0xEE,
            'ũ' => 0xF0,
            'ụ' => 0xF1,
            'ư' => 0xF2,
            'ừ' => 0xF3,
            'ứ' => 0xF5,
            'ử' => 0xF4,
            'ữ' => 0xF6,
            'ự' => 0xF7,
            'ỳ' => 0xF8,
            'ý' => 0xFA,
            'ỷ' => 0xF9,
            'ỹ' => 0xFB,
            'ỵ' => 0xFC,
            'đ' => 0xAE,

            // Uppercase vowels (a subset)
            'À' => 0x80,
            'Á' => 0x81,
            'Ả' => 0x82,
            'Ã' => 0x83,
            'Ạ' => 0x84,
            'Ă' => 0x85,
            'Ằ' => 0x86,
            'Ắ' => 0x87,
            'Ẳ' => 0x88,
            'Ẵ' => 0x89,
            'Ặ' => 0x8A,
            'Â' => 0x8B,
            'Ầ' => 0x8C,
            'Ấ' => 0x8D,
            'Ẩ' => 0x8E,
            'Ẫ' => 0x8F,
            'Ậ' => 0x90,
            'È' => 0x91,
            'É' => 0x92,
            'Ẻ' => 0x93,
            'Ẽ' => 0x94,
            'Ẹ' => 0x95,
            'Ê' => 0x96,
            'Ề' => 0x97,
            'Ế' => 0x98,
            'Ể' => 0x99,
            'Ễ' => 0x9A,
            'Ệ' => 0x9B,
            'Ì' => 0x9C,
            'Í' => 0x9D,
            'Ỉ' => 0x9E,
            'Ĩ' => 0x9F,
            'Ị' => 0xA0,
            'Ò' => 0xA1,
            'Ó' => 0xA2,
            'Ỏ' => 0xA3,
            'Õ' => 0xA4,
            'Ọ' => 0xA5,
            'Ô' => 0xA6,
            'Ồ' => 0xA7,
            'Ố' => 0xA8,
            'Ổ' => 0xA9,
            'Ỗ' => 0xAA,
            'Ộ' => 0xAB,
            'Đ' => 0xAC,

            // ASCII passthrough
            c if c.is_ascii() => return vec![c as u8],

            // Unmapped - return question mark
            _ => return vec![b'?'],
        };
        vec![byte]
    }

    /// Convert Unicode Vietnamese character to VNI encoding
    ///
    /// VNI uses combining sequences: base char + diacritic code
    fn to_vni(&self, ch: char) -> Vec<u8> {
        // VNI encoding uses specific byte sequences
        // Base vowel + separate byte for diacritic
        // Simplified implementation - returns UTF-8 for now
        // Full implementation would use VNI-specific mapping

        // For now, return UTF-8 as placeholder
        // TODO: Implement full VNI mapping
        let mut buf = [0u8; 4];
        ch.encode_utf8(&mut buf).as_bytes().to_vec()
    }

    /// Convert Unicode Vietnamese character to CP1258 (Windows-1258)
    ///
    /// CP1258 is mostly compatible with Windows-1252, with Vietnamese
    /// diacritics added using combining characters.
    fn to_cp1258(&self, ch: char) -> Vec<u8> {
        // CP1258 mapping for Vietnamese characters
        // Many Vietnamese chars need combining sequences
        let byte = match ch {
            // Characters that have direct mappings
            'À' => 0xC0,
            'Á' => 0xC1,
            'Â' => 0xC2,
            'Ã' => 0xC3,
            'È' => 0xC8,
            'É' => 0xC9,
            'Ê' => 0xCA,
            'Ì' => 0xCC,
            'Í' => 0xCD,
            'Ò' => 0xD2,
            'Ó' => 0xD3,
            'Ô' => 0xD4,
            'Õ' => 0xD5,
            'Ù' => 0xD9,
            'Ú' => 0xDA,
            'Ý' => 0xDD,
            'à' => 0xE0,
            'á' => 0xE1,
            'â' => 0xE2,
            'ã' => 0xE3,
            'è' => 0xE8,
            'é' => 0xE9,
            'ê' => 0xEA,
            'ì' => 0xEC,
            'í' => 0xED,
            'ò' => 0xF2,
            'ó' => 0xF3,
            'ô' => 0xF4,
            'õ' => 0xF5,
            'ù' => 0xF9,
            'ú' => 0xFA,
            'ý' => 0xFD,

            // Vietnamese-specific
            'Đ' => 0xD0,
            'đ' => 0xF0,
            'Ơ' => 0xD6,
            'ơ' => 0xF6,
            'Ư' => 0xDC,
            'ư' => 0xFC,

            // ASCII passthrough
            c if c.is_ascii() => return vec![c as u8],

            // Unmapped characters need combining sequences
            // For simplicity, return UTF-8 for complex chars
            _ => {
                return {
                    let mut buf = [0u8; 4];
                    ch.encode_utf8(&mut buf).as_bytes().to_vec()
                }
            }
        };
        vec![byte]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_from_u8() {
        assert_eq!(OutputEncoding::from_u8(0), OutputEncoding::Unicode);
        assert_eq!(OutputEncoding::from_u8(1), OutputEncoding::TCVN3);
        assert_eq!(OutputEncoding::from_u8(2), OutputEncoding::VNI);
        assert_eq!(OutputEncoding::from_u8(3), OutputEncoding::CP1258);
        assert_eq!(OutputEncoding::from_u8(99), OutputEncoding::Unicode); // Invalid defaults to Unicode
    }

    #[test]
    fn test_encoding_to_u8() {
        assert_eq!(OutputEncoding::Unicode.to_u8(), 0);
        assert_eq!(OutputEncoding::TCVN3.to_u8(), 1);
        assert_eq!(OutputEncoding::VNI.to_u8(), 2);
        assert_eq!(OutputEncoding::CP1258.to_u8(), 3);
    }

    #[test]
    fn test_unicode_passthrough() {
        let converter = EncodingConverter::new();
        let result = converter.convert_string("Việt Nam");
        assert_eq!(result, "Việt Nam".as_bytes().to_vec());
    }

    #[test]
    fn test_ascii_passthrough_all_encodings() {
        let mut converter = EncodingConverter::new();
        let text = "Hello World 123";

        // Unicode
        converter.set_encoding(OutputEncoding::Unicode);
        assert_eq!(converter.convert_string(text), text.as_bytes().to_vec());

        // TCVN3
        converter.set_encoding(OutputEncoding::TCVN3);
        assert_eq!(converter.convert_string(text), text.as_bytes().to_vec());

        // CP1258
        converter.set_encoding(OutputEncoding::CP1258);
        assert_eq!(converter.convert_string(text), text.as_bytes().to_vec());
    }

    #[test]
    fn test_tcvn3_single_char() {
        let mut converter = EncodingConverter::new();
        converter.set_encoding(OutputEncoding::TCVN3);

        // Test a few known TCVN3 mappings
        assert_eq!(converter.convert_char('đ'), vec![0xAE]);
        assert_eq!(converter.convert_char('à'), vec![0xB5]);
        assert_eq!(converter.convert_char('á'), vec![0xB8]);
    }

    #[test]
    fn test_cp1258_basic() {
        let mut converter = EncodingConverter::new();
        converter.set_encoding(OutputEncoding::CP1258);

        assert_eq!(converter.convert_char('Đ'), vec![0xD0]);
        assert_eq!(converter.convert_char('đ'), vec![0xF0]);
        assert_eq!(converter.convert_char('á'), vec![0xE1]);
    }

    #[test]
    fn test_set_encoding() {
        let mut converter = EncodingConverter::new();
        assert_eq!(converter.encoding(), OutputEncoding::Unicode);

        converter.set_encoding(OutputEncoding::TCVN3);
        assert_eq!(converter.encoding(), OutputEncoding::TCVN3);
    }

    #[test]
    fn test_full_string_conversions_and_unmapped() {
        let mut converter = EncodingConverter::new();
        let sample = "Việt Nam";
        
        // TCVN3 Full String
        converter.set_encoding(OutputEncoding::TCVN3);
        // Correct bytes for "Việt Nam": V-i-ệ-t- -N-a-m
        let expected_tcvn3: Vec<u8> = vec![b'V', b'i', 0xD6, b't', b' ', b'N', b'a', b'm'];
        assert_eq!(converter.convert_string(sample), expected_tcvn3);

        // TCVN3 Unmapped Character
        assert_eq!(converter.convert_string("Hello €"), vec![b'H', b'e', b'l', b'l', b'o', b' ', b'?']);

        // TCVN3 Uppercase
        let sample_upper = "VIỆT NAM";
        // Correct bytes for "VIỆT NAM": V-I-Ệ-T- -N-A-M
        let expected_tcvn3_upper: Vec<u8> = vec![b'V', b'I', 0x9B, b'T', b' ', b'N', b'A', b'M'];
        assert_eq!(converter.convert_string(sample_upper), expected_tcvn3_upper);

        // VNI (should be passthrough for now)
        converter.set_encoding(OutputEncoding::VNI);
        assert_eq!(converter.convert_string(sample), sample.as_bytes().to_vec());
        
        // CP1258 Full String (based on current implementation)
        converter.set_encoding(OutputEncoding::CP1258);
        // "Việt" -> V, i, ệ(passthrough), t
        // "Nam" -> N, a, m
        let mut expected_cp1258 = "V".as_bytes().to_vec();
        expected_cp1258.extend("i".as_bytes());
        expected_cp1258.extend("ệ".to_string().as_bytes()); // Currently passes through
        expected_cp1258.extend("t Nam".as_bytes());
        assert_eq!(converter.convert_string(sample), expected_cp1258);
    }
}
