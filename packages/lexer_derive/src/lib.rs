use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, ItemEnum, Path};

#[proc_macro_derive(Tokens, attributes(tokens_from))]
pub fn tokens_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    dbg!(&name);

    let tokens_from = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("tokens_from"))
        .expect("Attribute 'tokens_from' is missing");

    let tokens_enum_ident = extract_tokens_enum_ident(tokens_from);

    dbg!(&tokens_enum_ident);

    // @TODO: I need to setup a builr.rs file in `parser` to set
    // an env var with the path to the lexer enum.
    //
    // Also we can try to make the parser enum to know of the lexer
    // through a method. and that somehow that brings both enums
    // data into the proc macro.

    let tokens_enum: ItemEnum = syn::parse2(quote! {
        #tokens_enum_ident
    })
    .expect("Failed to parse tokens enum");

    dbg!(&tokens_enum);

    TokenStream::new()
}

fn extract_tokens_enum_ident(tokens_from: &Attribute) -> String {
    let argument = tokens_from
        .parse_args::<Path>()
        .expect("Failed to parse 'tokens_from' argument");

    let tokens_enum =
        argument
            .segments
            .into_pairs()
            .fold(String::new(), |path, pair| {
                let (segment, separator) = pair.into_tuple();
                let segment = segment.ident.to_string();
                let separator = separator.map_or("", |_| "::");

                format!("{}{}{}", path, segment, separator)
            });

    tokens_enum
}
