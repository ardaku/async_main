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

mod async_executor;
mod async_std;
mod futures;
mod pasts;
mod smolscale;
mod tokio;

use proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream,
    TokenTree,
};

/// Mark the entry point of the program.
///
/// # Current Runtime/Executor list
///  - `async_executor`
///  - `async_std`
///  - `futures`
///  - `pasts`
///  - `smolscale`
///  - `tokio`
///
/// # Options
/// Currently, no options are available and the crate will error if you add more
/// than one attribute parameter.
#[proc_macro_attribute]
pub fn async_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: Vec<_> = attr.into_iter().collect();
    let mut tokens = TokenStream::new();

    if attr.len() > 1 {
        error(
            &mut tokens,
            &format!("Expected a single attribute, found {attr:?}"),
        );
        return tokens;
    }
    let Some(runtime) = attr.get(0) else {
        error(&mut tokens, concat!(
            "Expected one of: ",
            "[async_executor, async_std, futures, pasts, tokio]",
            " specifying which runtime"));
        return tokens;
    };
    let TokenTree::Ident(runtime) = runtime else {
        error(
            &mut tokens,
            &format!("Expected an identifier, found {runtime:?}"),
        );
        return tokens;
    };

    match runtime.to_string().as_str() {
        "async_executor" => async_executor::async_executor(&mut tokens, item),
        "async_std" => async_std::async_std(&mut tokens, item),
        "futures" => futures::futures(&mut tokens, item),
        "pasts" => pasts::pasts(&mut tokens, item),
        "smolscale" => smolscale::smolscale(&mut tokens, item),
        "tokio" => tokio::tokio(&mut tokens, item),
        other => error(
            &mut tokens,
            &format!(
                "{other:?} runtime not supported yet.  Feel free to open a PR",
            ),
        ),
    }

    tokens
}

fn error(tokens: &mut TokenStream, message: &str) {
    tokens.extend([
        TokenTree::Ident(Ident::new("compile_error", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Joint)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter([TokenTree::Literal(Literal::string(
                message,
            ))]),
        )),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ]);
}
