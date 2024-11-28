mod input_source;
mod vec_source;

pub use input_source::InputSource;
pub use vec_source::VecSource;

#[cfg(target_os = "linux")]
pub use crate::linux::source::*;
