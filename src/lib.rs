use convert_case::{Case, Casing};
use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse_macro_input, Ident, ItemEnum};

#[allow(dead_code)]
fn import_crate() -> Ident {
    let found_crate =
        crate_name("serde-dot-case").expect("serde-dot-case is present in `Cargo.toml`");
    match found_crate {
        FoundCrate::Itself => Ident::new("crate", Span::call_site()),
        FoundCrate::Name(name) => Ident::new(&name, Span::call_site()),
    }
}

#[proc_macro_attribute]
pub fn serde_dot_case(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemEnum {
        attrs,
        vis,
        enum_token,
        ident,
        generics,
        brace_token: _,
        variants,
    } = parse_macro_input!(item);
    let variants = variants.iter().map(|v| {
        let name = v.ident.to_string().to_case(Case::Snake).replace('_', ".");
        quote! {
            #[serde(rename = #name)]
            #v
        }
    });
    quote! {
        #(#attrs)*
        #vis #enum_token #ident #generics {
            #(#variants,)*
        }
    }
    .into()
}
