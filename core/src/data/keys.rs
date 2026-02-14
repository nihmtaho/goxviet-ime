//! macOS virtual keycodes

// Letters
pub const A: u16 = 0;
pub const S: u16 = 1;
pub const D: u16 = 2;
pub const F: u16 = 3;
pub const H: u16 = 4;
pub const G: u16 = 5;
pub const Z: u16 = 6;
pub const X: u16 = 7;
pub const C: u16 = 8;
pub const V: u16 = 9;
pub const B: u16 = 11;
pub const Q: u16 = 12;
pub const W: u16 = 13;
pub const E: u16 = 14;
pub const R: u16 = 15;
pub const Y: u16 = 16;
pub const T: u16 = 17;
pub const O: u16 = 31;
pub const U: u16 = 32;
pub const I: u16 = 34;
pub const P: u16 = 35;
pub const L: u16 = 37;
pub const J: u16 = 38;
pub const K: u16 = 40;
pub const N: u16 = 45;
pub const M: u16 = 46;

// Numbers
pub const N1: u16 = 18;
pub const N2: u16 = 19;
pub const N3: u16 = 20;
pub const N4: u16 = 21;
pub const N5: u16 = 23;
pub const N6: u16 = 22;
pub const N7: u16 = 26;
pub const N8: u16 = 28;
pub const N9: u16 = 25;
pub const N0: u16 = 29;

// Special
pub const SPACE: u16 = 49;
pub const DELETE: u16 = 51;
pub const TAB: u16 = 48;
pub const RETURN: u16 = 36;
pub const ENTER: u16 = 76;
pub const ESC: u16 = 53;
pub const LEFT: u16 = 123;
pub const RIGHT: u16 = 124;
pub const DOWN: u16 = 125;
pub const UP: u16 = 126;

// Punctuation
pub const DOT: u16 = 47;
pub const COMMA: u16 = 43;
pub const SLASH: u16 = 44;
pub const SEMICOLON: u16 = 41;
pub const QUOTE: u16 = 39;
pub const LBRACKET: u16 = 33;
pub const RBRACKET: u16 = 30;
pub const BACKSLASH: u16 = 42;
pub const MINUS: u16 = 27;
pub const EQUAL: u16 = 24;
pub const BACKQUOTE: u16 = 50;

/// Check if key breaks word (space, punctuation, arrows, etc.)
pub fn is_break(key: u16) -> bool {
    matches!(
        key,
        SPACE
            | TAB
            | RETURN
            | ENTER
            | ESC
            | LEFT
            | RIGHT
            | UP
            | DOWN
            | DOT
            | COMMA
            | SLASH
            | SEMICOLON
            | QUOTE
            | LBRACKET
            | RBRACKET
            | BACKSLASH
            | MINUS
            | EQUAL
            | BACKQUOTE
            | N1
            | N2
            | N3
            | N4
            | N5
            | N6
            | N7
            | N8
            | N9
            | N0
    )
}

/// Check if key is a vowel (a, e, i, o, u, y)
pub fn is_vowel(key: u16) -> bool {
    matches!(key, A | E | I | O | U | Y)
}

/// Check if key is a letter
pub fn is_letter(key: u16) -> bool {
    matches!(
        key,
        A | B
            | C
            | D
            | E
            | F
            | G
            | H
            | I
            | J
            | K
            | L
            | M
            | N
            | O
            | P
            | Q
            | R
            | S
            | T
            | U
            | V
            | W
            | X
            | Y
            | Z
    )
}

/// Check if key is a consonant
pub fn is_consonant(key: u16) -> bool {
    is_letter(key) && !is_vowel(key)
}

/// Check if key is a number (0-9)
pub fn is_number(key: u16) -> bool {
    matches!(key, N0 | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9)
}

/// Convert ASCII character to macOS virtual keycode
///
/// The FFI layer receives ASCII characters from the platform,
/// but the legacy engine expects macOS virtual keycodes.
pub fn from_ascii(ascii: u8) -> Option<u16> {
    match ascii {
        b'a' | b'A' => Some(A),
        b's' | b'S' => Some(S),
        b'd' | b'D' => Some(D),
        b'f' | b'F' => Some(F),
        b'h' | b'H' => Some(H),
        b'g' | b'G' => Some(G),
        b'z' | b'Z' => Some(Z),
        b'x' | b'X' => Some(X),
        b'c' | b'C' => Some(C),
        b'v' | b'V' => Some(V),
        b'b' | b'B' => Some(B),
        b'q' | b'Q' => Some(Q),
        b'w' | b'W' => Some(W),
        b'e' | b'E' => Some(E),
        b'r' | b'R' => Some(R),
        b'y' | b'Y' => Some(Y),
        b't' | b'T' => Some(T),
        b'o' | b'O' => Some(O),
        b'u' | b'U' => Some(U),
        b'i' | b'I' => Some(I),
        b'p' | b'P' => Some(P),
        b'l' | b'L' => Some(L),
        b'j' | b'J' => Some(J),
        b'k' | b'K' => Some(K),
        b'n' | b'N' => Some(N),
        b'm' | b'M' => Some(M),
        b'1' => Some(N1),
        b'2' => Some(N2),
        b'3' => Some(N3),
        b'4' => Some(N4),
        b'5' => Some(N5),
        b'6' => Some(N6),
        b'7' => Some(N7),
        b'8' => Some(N8),
        b'9' => Some(N9),
        b'0' => Some(N0),
        b' ' => Some(SPACE),
        b'.' => Some(DOT),
        b',' => Some(COMMA),
        b'/' => Some(SLASH),
        b';' => Some(SEMICOLON),
        b'\'' => Some(QUOTE),
        b'[' => Some(LBRACKET),
        b']' => Some(RBRACKET),
        b'\\' => Some(BACKSLASH),
        b'-' => Some(MINUS),
        b'=' => Some(EQUAL),
        b'`' => Some(BACKQUOTE),
        0x08 | 0x7F => Some(DELETE),
        b'\t' => Some(TAB),
        b'\r' | b'\n' => Some(RETURN),
        0x1B => Some(ESC),
        _ => None,
    }
}
