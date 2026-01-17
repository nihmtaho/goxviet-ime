//! Typing buffer

pub const MAX: usize = 256;

use crate::utils;

/// Single character in buffer
///
/// Modifiers:
/// - `tone`: vowel diacritics (^, horn, breve)
/// - `mark`: tone marks (sắc, huyền, hỏi, ngã, nặng)
/// - `stroke`: consonant stroke (d → đ)
#[derive(Clone, Copy, Default)]
pub struct Char {
    pub key: u16,
    pub caps: bool,
    pub tone: u8,     // 0=none, 1=circumflex(^), 2=horn/breve
    pub mark: u8,     // 0=none, 1=sắc, 2=huyền, 3=hỏi, 4=ngã, 5=nặng
    pub stroke: bool, // true if 'd' → 'đ' (stroke through)
}

impl Char {
    #[inline]
    pub fn new(key: u16, caps: bool) -> Self {
        Self {
            key,
            caps,
            tone: 0,
            mark: 0,
            stroke: false,
        }
    }

    #[inline]
    pub fn has_tone(&self) -> bool {
        self.tone > 0
    }

    #[inline]
    pub fn has_mark(&self) -> bool {
        self.mark > 0
    }
}

/// Typing buffer
#[derive(Clone)]
pub struct Buffer {
    data: [Char; MAX],
    len: usize,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    #[inline]
    pub fn new() -> Self {
        Self {
            data: [Char::default(); MAX],
            len: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, c: Char) {
        if self.len < MAX {
            self.data[self.len] = c;
            self.len += 1;
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Option<Char> {
        if self.len > 0 {
            self.len -= 1;
            Some(self.data[self.len])
        } else {
            None
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn get(&self, i: usize) -> Option<&Char> {
        if i < self.len {
            Some(&self.data[i])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&mut self, i: usize) -> Option<&mut Char> {
        if i < self.len {
            Some(&mut self.data[i])
        } else {
            None
        }
    }

    #[inline]
    pub fn last(&self) -> Option<&Char> {
        if self.len > 0 {
            Some(&self.data[self.len - 1])
        } else {
            None
        }
    }

    /// Find indices of vowels in buffer
    #[inline]
    pub fn find_vowels(&self) -> Vec<usize> {
        use crate::data::keys;
        let mut positions = Vec::with_capacity(self.len);
        for i in 0..self.len {
            if keys::is_vowel(self.data[i].key) {
                positions.push(i);
            }
        }
        positions
    }

    /// Find vowel position by key (from end)
    #[inline]
    pub fn find_vowel_by_key(&self, key: u16) -> Option<usize> {
        use crate::data::keys;
        if !keys::is_vowel(key) {
            return None;
        }
        for i in (0..self.len).rev() {
            if self.data[i].key == key {
                return Some(i);
            }
        }
        None
    }

    /// Iterate over chars
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Char> {
        self.data[..self.len].iter()
    }

    /// Convert buffer to lowercase string (for shortcut matching)
    #[inline]
    pub fn to_lowercase_string(&self) -> String {
        let mut out = String::with_capacity(self.len);
        for c in &self.data[..self.len] {
            if let Some(ch) = utils::key_to_char(c.key, false) {
                out.push(ch);
            }
        }
        out
    }

    /// Convert buffer to string preserving case (for shortcut case matching)
    #[inline]
    pub fn to_string_preserve_case(&self) -> String {
        let mut out = String::with_capacity(self.len);
        for c in &self.data[..self.len] {
            if let Some(ch) = utils::key_to_char(c.key, c.caps) {
                out.push(ch);
            }
        }
        out
    }

    /// Convert buffer to full Vietnamese string with diacritics (for shortcut matching)
    ///
    /// This includes tone marks (sắc/huyền/hỏi/ngã/nặng), vowel marks (circumflex/horn/breve),
    /// and stroked consonants (đ). Use this for shortcut matching to ensure exact comparison.
    #[inline]
    pub fn to_full_string(&self) -> String {
        use crate::data::{chars, keys};
        let mut out = String::with_capacity(self.len);
        for c in &self.data[..self.len] {
            // Handle đ/Đ (stroked D)
            if c.key == keys::D && c.stroke {
                out.push(chars::get_d(c.caps));
                continue;
            }
            // Try to get full Vietnamese character with diacritics
            if let Some(ch) = chars::to_char(c.key, c.caps, c.tone, c.mark) {
                out.push(ch);
                continue;
            }
            // Fallback to basic character
            if let Some(ch) = utils::key_to_char(c.key, c.caps) {
                out.push(ch);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let mut buf = Buffer::new();
        assert!(buf.is_empty());

        buf.push(Char::new(0, false));
        buf.push(Char::new(1, true));
        assert_eq!(buf.len(), 2);

        let c = buf.pop().unwrap();
        assert_eq!(c.key, 1);
        assert!(c.caps);

        buf.clear();
        assert!(buf.is_empty());
    }
}
