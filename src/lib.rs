extern crate proc_macro;

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
        "pasts" => pasts(&mut tokens, item),
        other => error(
            &mut tokens,
            &format!(
                "{other:?} runtime not supported yet.  Feel free to open a PR",
            ),
        ),
    }

    tokens
}

fn pasts(tokens: &mut TokenStream, item: TokenStream) {
    let mut body = item;

    body.extend([
        TokenTree::Ident(Ident::new("pasts", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("Executor", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("default", Span::call_site())),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::new(),
        )),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new("spawn", Span::call_site())),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter([
                TokenTree::Ident(Ident::new("main", Span::call_site())),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::new(),
                )),
            ]),
        )),
    ]);

    tokens.extend([
        TokenTree::Ident(Ident::new("fn", Span::call_site())),
        TokenTree::Ident(Ident::new("main", Span::call_site())),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::new(),
        )),
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            body,
        )),
    ]);
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
