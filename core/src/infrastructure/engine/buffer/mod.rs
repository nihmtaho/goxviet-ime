//! Buffer management — re-exported from shared/buffer/
//!
//! Original implementation moved to `crate::shared::buffer`.

pub use crate::shared::buffer::buffer;
pub use crate::shared::buffer::raw_input_buffer;
pub use crate::shared::buffer::rebuild;

pub use crate::shared::buffer::{Buffer, Char, MAX};
pub use crate::shared::buffer::RawInputBuffer;
