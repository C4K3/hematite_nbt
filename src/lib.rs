//! MC Named Binary Tag type.

extern crate byteorder;
extern crate flate2;

/* Re-export the core API from submodules. */
pub use blob::Blob;
pub use error::{Error, Result};
pub use value::Value;

mod raw;
mod blob;
mod error;
mod value;

#[cfg(feature = "serde")] #[macro_use] extern crate serde;

#[cfg(feature = "serde")] #[macro_use] mod macros;
#[cfg(feature = "serde")] pub mod de;
#[cfg(feature = "serde")] pub mod ser;

#[cfg(test)] mod tests;
