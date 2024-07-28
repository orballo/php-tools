use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{parse_macro_input, parse_str};

#[proc_macro_attribute]
pub fn add_tokens(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let root = env!("CARGO_MANIFEST_DIR");
    let tokens_enum_path = Path::new(root).join("../lexer/src/tokens.rs");
    let tokens_enum = extract_tokens_enum_from_file(tokens_enum_path);
    let tokens_enum_variants = extract_enum_variants(tokens_enum.variants);

    let base_enum = parse_macro_input!(input as syn::ItemEnum);
    let base_enum_name = &base_enum.ident;
    let base_enum_variants = extract_enum_variants(base_enum.variants);

    let output = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(u16)]
        pub enum #base_enum_name {
            #(#tokens_enum_variants,)*
            #(#base_enum_variants,)*
        }
    };

    output.into()
}

fn extract_tokens_enum_from_file(file_path: PathBuf) -> syn::ItemEnum {
    if !file_path.exists() {
        panic!("Tokens file does not exist");
    }

    let file_path = file_path.to_str().unwrap();
    let file =
        fs::read_to_string(file_path).expect("Failed to read tokens file");
    let parsed_file =
        parse_str::<syn::File>(&file).expect("Failed to parse tokens file");

    let tokens_enum = parsed_file
        .items
        .into_iter()
        .find_map(|item| {
            if let syn::Item::Enum(item_enum) = item {
                return Some(item_enum);
            } else {
                None
            }
        })
        .expect("Failed to find enum in tokens file");

    tokens_enum
}

fn extract_enum_variants(
    variants: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> Vec<syn::Ident> {
    variants.into_iter().map(|variant| variant.ident).collect()
}
