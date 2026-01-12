use crate::data::keys;

/// Combined bitmask for quick character property checking
pub const PROP_VOWEL: u32 = 1 << 0;
pub const PROP_CONSONANT: u32 = 1 << 1;
pub const PROP_INITIAL_INVALID: u32 = 1 << 2; // F, J, W, Z
pub const PROP_CODA_INVALID: u32 = 1 << 3; // b, d, f, g, h (unless nh/ng), etc.

/// Property table indexed by keycode (0-127)
/// Provides O(1) property lookup
pub static CHAR_PROPS: [u32; 128] = {
    let mut props = [0u32; 128];

    // Letters
    props[keys::A as usize] = PROP_VOWEL;
    props[keys::E as usize] = PROP_VOWEL;
    props[keys::I as usize] = PROP_VOWEL;
    props[keys::O as usize] = PROP_VOWEL;
    props[keys::U as usize] = PROP_VOWEL;
    props[keys::Y as usize] = PROP_VOWEL;

    props[keys::B as usize] = PROP_CONSONANT | PROP_CODA_INVALID;
    props[keys::C as usize] = PROP_CONSONANT;
    props[keys::D as usize] = PROP_CONSONANT | PROP_CODA_INVALID;
    props[keys::F as usize] = PROP_CONSONANT | PROP_INITIAL_INVALID | PROP_CODA_INVALID;
    props[keys::G as usize] = PROP_CONSONANT; // valid in ng, gh
    props[keys::H as usize] = PROP_CONSONANT; // valid in nh, ch, th, ph, kh
    props[keys::J as usize] = PROP_CONSONANT | PROP_INITIAL_INVALID | PROP_CODA_INVALID;
    props[keys::K as usize] = PROP_CONSONANT | PROP_CODA_INVALID;
    props[keys::L as usize] = PROP_CONSONANT | PROP_CODA_INVALID;
    props[keys::M as usize] = PROP_CONSONANT;
    props[keys::N as usize] = PROP_CONSONANT;
    props[keys::P as usize] = PROP_CONSONANT;
    props[keys::Q as usize] = PROP_CONSONANT;
    props[keys::R as usize] = PROP_CONSONANT | PROP_CODA_INVALID;
    props[keys::S as usize] = PROP_CONSONANT | PROP_CODA_INVALID;
    props[keys::T as usize] = PROP_CONSONANT;
    props[keys::V as usize] = PROP_CONSONANT | PROP_CODA_INVALID;
    props[keys::W as usize] = PROP_CONSONANT | PROP_INITIAL_INVALID | PROP_CODA_INVALID;
    props[keys::X as usize] = PROP_CONSONANT | PROP_CODA_INVALID;
    props[keys::Z as usize] = PROP_CONSONANT | PROP_INITIAL_INVALID | PROP_CODA_INVALID;

    props
};

/// Bigram matrix for Vietnamese phonotactics
/// bit N is set if key N can follow current key
pub type BigramRow = u128; // Use 128 bits for 128 keys

pub static VIETNAMESE_BIGRAMS: [BigramRow; 128] = {
    let mut matrix = [0u128; 128];

    // Helper to set bits
    macro_rules! set {
        ($key:expr, $($next:expr),*) => {
            $( matrix[$key as usize] |= 1 << ($next as usize); )*
        };
    }

    // A follows...
    set!(
        keys::A,
        keys::N,
        keys::M,
        keys::C,
        keys::T,
        keys::P,
        keys::I,
        keys::U,
        keys::O,
        keys::A // aa -> â
    );
    // B follows (start only)
    set!(
        keys::B,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y
    );
    // C
    set!(
        keys::C,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y,
        keys::H
    );
    // D
    set!(
        keys::D,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y,
        keys::D // dd -> đ
    );
    // G
    set!(
        keys::G,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y,
        keys::H
    );
    // H
    set!(
        keys::H,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y
    );
    // K
    set!(
        keys::K,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y,
        keys::H
    );
    // L
    set!(
        keys::L,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y
    );
    // M
    set!(
        keys::M,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y
    );
    // N
    set!(
        keys::N,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y,
        keys::G,
        keys::H
    );
    // P
    set!(
        keys::P,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y,
        keys::H
    );
    // Q
    set!(keys::Q, keys::U);
    // R
    set!(
        keys::R,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y
    );
    // S
    set!(
        keys::S,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y
    );
    // T
    set!(
        keys::T,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y,
        keys::H,
        keys::R
    );
    // V
    set!(
        keys::V,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y
    );
    // X
    set!(
        keys::X,
        keys::A,
        keys::E,
        keys::I,
        keys::O,
        keys::U,
        keys::Y
    );
    // E follows...
    set!(
        keys::E,
        keys::N,
        keys::M,
        keys::C,
        keys::T,
        keys::P,
        keys::E, // ee -> ê
        keys::O  // eo -> eo
    );
    // I follows...
    set!(
        keys::I,
        keys::N,
        keys::M,
        keys::C,
        keys::T,
        keys::P,
        keys::A, // ia
        keys::E, // ie -> iê
        keys::U  // iu
    );
    // O follows...
    set!(
        keys::O,
        keys::N,
        keys::M,
        keys::C,
        keys::T,
        keys::P,
        keys::A, // oa
        keys::E, // oe
        keys::I, // oi
        keys::O  // oo -> ô
    );
    // U follows...
    set!(
        keys::U,
        keys::N,
        keys::M,
        keys::C,
        keys::T,
        keys::P,
        keys::A, // ua
        keys::E, // ue -> uê
        keys::I, // ui
        keys::O, // uo -> uô / ươ
        keys::U, // uu -> ư
        keys::Y  // uy
    );
    // Y follows...
    set!(
        keys::Y,
        keys::N,
        keys::M,
        keys::A, // ya
        keys::E  // ye -> yê (uyê)
    );
    // ... complete matrix would go here ...
    // For O(1) performance, we need this matrix precomputed.

    matrix
};
