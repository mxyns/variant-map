use darling::FromVariant;
use proc_macro2::TokenStream;
use quote::{quote};
use syn::{DataEnum, Ident, Variant};
use crate::attrs::{KeyNameAttr, MapType};

pub(crate) fn generate_key_enum(
    map_type: &MapType,
    enum_data: &DataEnum,
    key_enum_name: &Ident,
) -> proc_macro2::TokenStream {
    let key_variants = enum_data.variants.iter().map(|variant| {
        let key_name_attr = KeyNameAttr::from_variant(variant).expect("Wrong key_name options");

        // Useful in case of variant identifier renaming
        let key_name = key_name_attr.key_name(variant);
        let serde_rename = key_name_attr.serde_rename(variant);

        quote! {
            #[serde(rename=#serde_rename)]


            #key_name
        }
    });

    // TODO cleanup useless derives, check if serde derives and #[serde(rename)] attributes are required on keys
    let derives_quote = match map_type {
        MapType::HashMap => {
            quote! { #[derive(Debug, PartialEq, Eq, Hash, ::serde::Serialize, ::serde::Deserialize)] }
        }
        MapType::BTreeMap => {
            quote! { #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ::serde::Serialize, ::serde::Deserialize)] }
        }
        MapType::StructMap => { quote! {#[derive(Debug, ::serde::Serialize, ::serde::Deserialize)] } }
    };

    quote! {
        #derives_quote
        enum #key_enum_name {
            #(#key_variants),*
        }
    }
}

pub(crate) fn enum_entries_map_to<F>(enum_name: &Ident, enum_data: &DataEnum, key_enum_name: &Ident, to: F) -> TokenStream
    where F: Fn(&Ident, &Ident, Option<Option<TokenStream>>, &Ident, &Ident) -> TokenStream {
    let match_cases = enum_data.variants.iter().map(|variant| {
        let key_name = &KeyNameAttr::from_variant(variant)
            .expect("Wrong key_name options")
            .key_name(variant);

        let Variant {
            attrs: _,
            ident,
            fields,
            discriminant: _,
        } = variant;

        let skip_fields = if !fields.is_empty() {
            Some(fields)
        } else {
            None
        }
        .map(|fields| {
            let skip_fields = fields.iter().map(|_| quote!(_));
            Some(quote! { (#(#skip_fields),*) })
        });

        let to = to(enum_name, ident, skip_fields, key_enum_name, key_name);

        to
    });

    quote! {
        #(#match_cases),*
    }
}