// Copyright © 2022-2023 The async_main crate contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).
//
//! Runtime-agnostic async main proc macro.  Currently, this crate only supports
//! single-threaded task pools, but in a future version will add a configuration
//! option to enable multi-threaded task pools.
//!
//! # Getting Started
//! Choose a runtime by enabling one of the following features:
//!
//!  - *`async-executor`*
//!  - *`async-std`*
//!  - *`futures`*
//!  - *`pasts`*
//!  - *`tokio`*
//!
//! ```rust
#![doc = include_str!("../examples/main.rs")]
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

#[allow(unused_extern_crates)]
extern crate alloc;

#[cfg_attr(feature = "async-executor", path = "async_executor.rs")]
#[cfg_attr(feature = "async-std", path = "async_std.rs")]
#[cfg_attr(feature = "futures", path = "futures.rs")]
#[cfg_attr(feature = "pasts", path = "pasts.rs")]
#[cfg_attr(feature = "tokio", path = "tokio.rs")]
mod spawn;

pub use async_main_macro::async_main;

pub use self::spawn::LocalSpawner;

/// Implementation for spawning tasks on an executor.
pub trait Spawn: Clone {
    /// Spawn a [`Future`](core::future::Future) without the [`Send`]
    /// requirement.
    ///
    /// This forces the executor to always run the task on the same thread that
    /// this method is called on.
    fn spawn_local(&self, f: impl core::future::Future<Output = ()> + 'static);

    /// Spawn a [`Future`](core::future::Future) that is [`Send`].
    ///
    /// This allows the executor to run the task on whatever thread it
    /// determines is most efficient.
    #[inline(always)]
    fn spawn(&self, f: impl core::future::Future<Output = ()> + Send + 'static) {
        self.spawn_local(f)
    }
}
