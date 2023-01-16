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
//! # Async Executor (with `futures-lite`)
//! ```rust
#![doc = include_str!("../examples/async_executor.rs")]
//! ```
//! 
//! # Async Std
//! ```rust
#![doc = include_str!("../examples/async_std.rs")]
//! ```
//! 
//! # Futures
//! ```rust
#![doc = include_str!("../examples/futures.rs")]
//! ```
//! 
//! # Pasts
//! ```rust
#![doc = include_str!("../examples/pasts.rs")]
//! ```
//! 
//! # Smolscale
//! ```rust
#![doc = include_str!("../examples/smolscale.rs")]
//! ```
//! 
//! # Tokio
//! ```rust
#![doc = include_str!("../examples/tokio.rs")]
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

#[cfg_attr(feature = "async-executor", path = "async_executor.rs")]
#[cfg_attr(feature = "async-std", path = "async_std.rs")]
#[cfg_attr(feature = "futures", path = "futures.rs")]
#[cfg_attr(feature = "pasts", path = "pasts.rs")]
#[cfg_attr(feature = "smolscale", path = "smolscale.rs")]
#[cfg_attr(feature = "tokio", path = "tokio.rs")]
mod spawn;

pub use async_main_macro::async_main;
pub use pasts::Spawn;

pub use self::spawn::{LocalSpawner};
