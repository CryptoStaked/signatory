//! Use either of `std` prelude or `alloc` prelude (latter only on nightly)

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::prelude::*;

#[cfg(feature = "std")]
pub use std::prelude::v1::*;
