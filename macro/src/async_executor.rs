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
        TokenTree::Ident(Ident::new("let", Span::call_site())),
        TokenTree::Ident(Ident::new("executor", Span::call_site())),
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        TokenTree::Ident(Ident::new("async_executor", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("LocalExecutor", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("new", Span::call_site())),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::new(),
        )),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
        TokenTree::Ident(Ident::new("let", Span::call_site())),
        TokenTree::Ident(Ident::new("executor", Span::call_site())),
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        TokenTree::Ident(Ident::new("std", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("sync", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Arc", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("new", Span::call_site())),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter([TokenTree::Ident(Ident::new(
                "executor",
                Span::call_site(),
            ))]),
        )),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
        TokenTree::Ident(Ident::new("let", Span::call_site())),
        TokenTree::Ident(Ident::new("future", Span::call_site())),
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        TokenTree::Ident(Ident::new("executor", Span::call_site())),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new("run", Span::call_site())),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter([
                TokenTree::Ident(Ident::new("main", Span::call_site())),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from_iter([
                        TokenTree::Ident(Ident::new(
                            "executor",
                            Span::call_site(),
                        )),
                        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                        TokenTree::Ident(Ident::new(
                            "clone",
                            Span::call_site(),
                        )),
                        TokenTree::Group(Group::new(
                            Delimiter::Parenthesis,
                            TokenStream::new(),
                        )),
                    ]),
                )),
            ]),
        )),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
        TokenTree::Ident(Ident::new("futures_lite", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("future", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("block_on", Span::call_site())),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter([TokenTree::Ident(Ident::new(
                "future",
                Span::call_site(),
            ))]),
        )),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ]);

    tokens.extend([
        TokenTree::Ident(Ident::new("fn", Span::call_site())),
        TokenTree::Ident(Ident::new("main", Span::call_site())),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::new(),
        )),
        TokenTree::Group(Group::new(Delimiter::Brace, body)),
    ]);
}
