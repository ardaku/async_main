// Copyright © 2022-2023 The async_main crate contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).
//
//! See [`async_main`](https://crates.io/crates/async_main)

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

    if attr.len() > 0 {
        error(
            &mut tokens,
            &format!("Expected no attributes, found {attr:?}"),
        );
        return tokens;
    }

    wrap(&mut tokens, item);

    tokens
}

fn ident(name: &str) -> TokenTree {
    TokenTree::Ident(Ident::new(name, Span::call_site()))
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

pub(crate) fn wrap(tokens: &mut TokenStream, item: TokenStream) {
    let mut body = item;

    body.extend([
        ident("let"),
        ident("spawner"),
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        ident("async_main"),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        ident("LocalSpawner"),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        ident("default"),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::new(),
        )),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
        ident("spawner"),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        ident("clone"),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::new(),
        )),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        ident("block_on"),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter([
                ident("main"),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from_iter([
                        ident("spawner"),
                    ]),
                )),
            ]),
        )),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ]);

    tokens.extend([
        ident("fn"),
        ident("main"),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::new(),
        )),
        TokenTree::Group(Group::new(Delimiter::Brace, body)),
    ]);
}
