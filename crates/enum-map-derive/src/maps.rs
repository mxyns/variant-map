use proc_macro2::TokenStream;
use quote::{quote};
use syn::spanned::Spanned;
use syn::{DeriveInput, Ident};
use crate::attrs::MapType;
use crate::common;


fn generate_impl_key_trait_for_key_enum(
    map_type: &MapType,
    key_enum_name: &Ident,
) -> proc_macro2::TokenStream {
    match map_type {
        MapType::HashMap => {
            quote! {impl HashKey for #key_enum_name {}}
        }
        MapType::BTreeMap => {
            quote! {impl OrdHashKey for #key_enum_name {}}
        }
    }
}

pub(crate) fn generate_map_code(ast: &mut DeriveInput, map_type: &MapType, enum_name: &Ident, key_enum_name: &Ident) -> TokenStream {
    match &mut ast.data {
        syn::Data::Enum(ref mut enum_data) => {
            let key_enum_quote = common::generate_key_enum(map_type, enum_data, &key_enum_name);

            let impl_map_value_for_enum_quote = common::generate_impl_map_value(
                map_type,
                (&ast.generics, enum_name),
                enum_data,
                &key_enum_name,
            );

            let impl_hash_key_for_enum_key_quote =
                generate_impl_key_trait_for_key_enum(map_type, &key_enum_name);

            quote! {
                #[doc(hidden)]
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate enum_map as _enum_map;
                    use _enum_map::common::*;
                    use _enum_map::#map_type::*;
                    use _enum_map::serde;

                    #[automatically_derived]
                    #key_enum_quote

                    #[automatically_derived]
                    #impl_map_value_for_enum_quote

                    #[automatically_derived]
                    #impl_hash_key_for_enum_key_quote
                };
            }
        }
        _ => syn::Error::new(ast.span(), "EnumMap works only on enums").into_compile_error(),
    }
}
