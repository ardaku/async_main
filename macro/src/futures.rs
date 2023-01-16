// Copyright © 2022-2023 The async_main crate contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use super::*;

pub(crate) fn wrap(tokens: &mut TokenStream, item: TokenStream) {
    let mut body = item;

    body.extend([
        ident("let"),
        ident("mut"),
        ident("executor"),
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        ident("futures"),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        ident("executor"),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        ident("LocalPool"),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        ident("new"),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::new(),
        )),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
        ident("executor"),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        ident("run_until"),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter([
                ident("main"),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from_iter([
                        ident("executor"),
                        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                        ident("spawner"),
                        TokenTree::Group(Group::new(
                            Delimiter::Parenthesis,
                            TokenStream::new(),
                        )),
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
