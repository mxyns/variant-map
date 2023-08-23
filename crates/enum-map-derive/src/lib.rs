use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

// TODO choose map implementation with a derive attribute
// TODO macro_rules for getting types more easily
#[proc_macro_derive(EnumMap, attributes(unimarc))]
pub fn derive(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let keys = match &mut ast.data {
        syn::Data::Enum(ref mut enum_data) => {
            let key_enum_name = format_ident!("{}Key", name);

            let key_variants = enum_data.variants.iter().map(|variant| {
                let key_name = variant.ident.clone();

                quote! {
                    #key_name,
                }
            });

            quote! {
                enum #key_enum_name {
                    #(#key_variants)*
                }
            }
        }
        _ => panic!("EnumMap works only on enums"),
    };

    let result = quote! {
        #keys
    };

    result.into()
}
