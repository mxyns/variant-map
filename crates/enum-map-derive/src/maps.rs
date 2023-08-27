use crate::attrs::MapType;
use crate::common;
use crate::common::EnumType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{DataEnum, DeriveInput, Ident};

fn generate_impl_key_trait_for_key_enum(
    map_type: &MapType,
    key_enum_name: &Ident,
) -> Option<TokenStream> {
    match map_type {
        MapType::HashMap => Some(quote! {impl HashKey for #key_enum_name {}}),
        MapType::BTreeMap => Some(quote! {impl OrdHashKey for #key_enum_name {}}),
        MapType::Struct => None,
    }
}

pub(crate) fn generate_map_code(
    ast: &DeriveInput,
    map_type: &MapType,
    enum_type: &EnumType,
    key_enum_name: &Ident,
) -> TokenStream {
    match &ast.data {
        syn::Data::Enum(ref enum_data) => {
            let key_enum_quote = common::generate_key_enum(map_type, enum_data, key_enum_name);

            let impl_map_value_for_enum_quote =
                generate_impl_map_value(map_type, enum_type, enum_data, key_enum_name);

            let impl_hash_key_for_enum_key_quote =
                generate_impl_key_trait_for_key_enum(map_type, key_enum_name);

            quote! {
                use _enum_map::#map_type::*;

                #[automatically_derived]
                #key_enum_quote

                #[automatically_derived]
                #impl_map_value_for_enum_quote

                #[automatically_derived]
                #impl_hash_key_for_enum_key_quote
            }
        }
        _ => syn::Error::new(ast.span(), "EnumMap works only on enums").into_compile_error(),
    }
}

pub(crate) fn generate_impl_map_value(
    _map_type: &MapType,
    enum_type: &EnumType,
    enum_data: &DataEnum,
    key_enum_name: &Ident,
) -> TokenStream {
    let EnumType {
        enum_name,
        generics,
    } = enum_type;

    let match_body = common::enum_entries_map_to(
        enum_name,
        enum_data,
        key_enum_name,
        |enum_name, variant_name, skip_fields, key_enum_name, key_name| {
            quote! {
                #enum_name::#variant_name #skip_fields => #key_enum_name::#key_name
            }
        },
    );

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics MapValue for #enum_name #ty_generics #where_clause {
            type Key = #key_enum_name;
            type Map = Map<Self::Key, Self>;

            fn to_key(&self) -> Self::Key {
                match self {
                   #match_body
                }
            }


            fn make_map() -> Self::Map {
               Self::Map::default()
            }
        }
    }
}
