mod attrs;
mod common;
mod maps;
mod structs;

use crate::attrs::{MapType, BaseAttr};
use crate::common::EnumType;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};
use syn::spanned::Spanned;

// TODO [x] rename keys and key enum
// TODO [x] merge make_map with impl EnumMapValue
// TODO [x] macro_rules for key and map syntactic sugar
// TODO [x] Index and IndexMut syntactic sugar
// TODO [x] choose map/table implementation with a derive attribute
// TODO [3/3] cleanup -derive code, split into functions, into different files
// TODO [2/2] handle generics + bounds
// TODO [1/2] add struct and array versions of the "map"
// TODO [x] handle generics on struct
// TODO [x] (de)serialize derive on struct
// TODO [x] add (de)serialize impl only if some attribute is set
// TODO [x] custom visibility on keys, struct, impls, etc.
// TODO [x] split EnumMap and EnumStruct derive into 2 functions with different attributes
    // TODO [x] move declared structs
// TODO [x] rename crate to something unused
// TODO doc
// TODO publish
// TODO allow using user generated (possibly generic or tuple variant) keys
// TODO? trait for all maps
// TODO? tight couple Map and MapValue if possible

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
        Err(_) => (syn::Error::new(ast.span(), "VariantStore works only on enums").into_compile_error(), quote!())
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