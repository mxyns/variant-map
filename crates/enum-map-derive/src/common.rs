use darling::FromVariant;
use quote::{quote};
use syn::{DataEnum, Generics, Ident, Variant};
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

    let derives_quote = match map_type {
        MapType::HashMap => {
            quote! { #[derive(Debug, PartialEq, Eq, Hash, ::serde::Serialize, ::serde::Deserialize)] }
        }
        MapType::BTreeMap => {
            quote! { #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ::serde::Serialize, ::serde::Deserialize)] }
        }
    };

    quote! {
        #derives_quote
        enum #key_enum_name {
            #(#key_variants),*
        }
    }
}


pub(crate) fn generate_impl_map_value(
    _map_type: &MapType,
    enum_type: (&Generics, &Ident),
    enum_data: &DataEnum,
    key_enum_name: &Ident,
) -> proc_macro2::TokenStream {
    let (generics, enum_name) = enum_type;

    let match_case = enum_data.variants.iter().map(|variant| {
        let key_name = KeyNameAttr::from_variant(variant)
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

        quote! {
            #enum_name::#ident #skip_fields => #key_enum_name::#key_name
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics MapValue for #enum_name #ty_generics #where_clause {
            type Key = #key_enum_name;
            type Map = Map<Self::Key, Self>;

            fn to_key(&self) -> Self::Key {
                match self {
                    #(#match_case),*
                }
            }


            fn make_map() -> Self::Map {
               Self::Map::default()
            }
        }
    }
}