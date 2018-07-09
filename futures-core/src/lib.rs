//! Core traits and types for asynchronous operations in Rust.

#![feature(pin, arbitrary_self_types, futures_api)]

#![no_std]

#![deny(missing_docs, missing_debug_implementations, warnings)]
#![deny(bare_trait_objects)]

#![doc(html_root_url = "https://docs.rs/futures-core/0.3.0-alpha")]

#![cfg_attr(feature = "nightly", feature(cfg_target_has_atomic))]

#[macro_use]
#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "either")]
extern crate either;

#[doc(hidden)]
pub mod core_reexport {
    pub use core::{mem, future, task};
}

#[doc(hidden)] pub use crate::future::Future;
#[doc(hidden)] pub use crate::future::CoreFutureExt;
#[doc(hidden)] pub use crate::future::TryFuture;

#[doc(hidden)] pub use crate::stream::Stream;
#[doc(hidden)] pub use crate::stream::TryStream;

#[doc(hidden)] pub use crate::task::Poll;
#[doc(hidden)] pub use crate::task::Context;
#[cfg(feature = "std")] #[doc(hidden)] pub use crate::task::ContextExt;

macro_rules! if_std {
    ($($i:item)*) => ($(
        #[cfg(feature = "std")]
        $i
    )*)
}

#[macro_use]
mod macros;

pub mod future;

pub mod stream;

pub mod task;
