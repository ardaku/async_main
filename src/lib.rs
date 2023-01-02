// Copyright © 2022-2023 The async_main crate contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).
//
//! Runtime-agnostic async main proc macro.  By default, this crate uses a
//! single-threaded runtime.
//!
//! # Pasts
//! ```rust
#![doc = include_str!("../examples/pasts.rs")]
//! ```
//!
//! # Tokio
//! ```rust
#![doc = include_str!("../examples/tokio.rs")]
//! ```

extern crate proc_macro;

mod pasts;
mod tokio;

use proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream,
    TokenTree,
};

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
        error(&mut tokens, "Expected one of: [pasts] specifying which runtime");
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
        "pasts" => pasts::pasts(&mut tokens, item),
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
