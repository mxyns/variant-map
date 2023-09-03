//! Enum variants stored in Maps.
//!
//! Provides derive macros for `variant_map`
//!
//! Includes a `StructMap` which is a struct with a field per variant of the enum
//!
//! Pro: This struct has instant access to the fields (compared to the other Maps that need a lookup)
//!
//! Con: Restricted API
//!
//! # Example
//!
//! ```
//!     use variant_map_derive::VariantStore;
//!
//!     #[derive(VariantStore)]
//!     enum MyEnum {
//!         A,
//!         B(i32),
//!     }
//! ```
//!
//! For more detailed examples check out the [example project](https://github.com/mxyns/variant-map/tree/master/example) on this crates' [repo](https://github.com/mxyns/variant-map/)
//!

/// Parameters of the macros attributes
pub(crate) mod attrs;

/// Helper functions for the [maps] and [structs] implementations
pub(crate) mod common;

/// Implementation of the derive for variant_map
pub(crate) mod maps;

/// Implementation of the derive for `StructMap`
/// This type is derive-only (not included in the base crate)
///
/// This macro expansion contains a new Key Enum, a `struct` specific to the enum type
/// with one field per variant
///
/// It also features implementation of the same traits as a normal variant Map
pub(crate) mod structs;

use crate::attrs::{MapType, BaseAttr};
use crate::common::EnumType;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};
use syn::spanned::Spanned;

// TODO fix "private documentation" rustdoc
// TODO publish
// TODO allow using user generated (possibly generic or tuple variant) keys
// TODO [1/2] add struct and array versions of the "map"
// TODO? trait for all maps to reduce duplicate code
// TODO? tight couple Map trait and MapValue if possible

/// The only derive macro of this crate
///
/// Apply it on an enum to automatically generate an enum of keys and a map to store the variants
///
/// # Arguments
///
/// See [attrs::BaseAttr]
///
/// See other attributes in [attrs]
///
#[proc_macro_derive(VariantStore, attributes(VariantStore, VariantMap, VariantStruct, key_name))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let enum_name = ast.ident.clone();

    // VariantStore attribute parameters
    let (key_enum_name, map_type) = {
        let base_attr = BaseAttr::from_derive_input(&ast).expect("Wrong VariantStore parameters");

        (
            base_attr.keys_name(format_ident!("{}Key", &enum_name)),
            base_attr.map_type()
        )
    };

    let enum_type = &EnumType {
        enum_name: &enum_name,
        generics: &ast.generics,
    };

    let result = match map_type {
        MapType::HashMap | MapType::BTreeMap => {
            maps::generate_map_code(&ast, &map_type, enum_type, &key_enum_name)
        }
        MapType::Struct => {
            structs::generate_struct_code(&ast, &map_type, enum_type, &key_enum_name)
        }
    };

    let (out_of_const, inside_const) = match result {
        Ok( tup ) => tup,
        Err(_) => (Some(syn::Error::new(ast.span(), "VariantStore works only on enums").into_compile_error()), None)
    };

    let result = quote! {

        #out_of_const

        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate variant_map as _variant_map;
            use _variant_map::common::*;
            use _variant_map::serde;

            #inside_const
        };
    };

    result.into()
}