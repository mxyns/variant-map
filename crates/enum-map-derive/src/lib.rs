mod attrs;
mod common;
mod maps;
mod structs;

use crate::attrs::{MapAttr, MapType};
use crate::common::EnumType;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

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
// TODO add (de)serialize impl only if some attribute is set
// TODO custom visibility on keys, struct, impls, etc.
// TODO trait for all maps
// TODO? tight couple Map and MapValue if possible
// TODO doc
// TODO publish
// TODO allow using user generated (possibly generic or tuple variant) keys

#[proc_macro_derive(EnumMap, attributes(EnumMap, key_name))]
pub fn derive_enum_map(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let enum_name = ast.ident.clone();

    // EnumMap attribute parameters
    let (key_enum_name, map_type) = {
        let key_enum_name_attr = MapAttr::from_derive_input(&ast).expect("Wrong enum_name options");
        (
            key_enum_name_attr.enum_name(format_ident!("{}Key", &enum_name)),
            key_enum_name_attr.map_impl_mod(),
        )
    };

    let enum_type = &EnumType {
        enum_name: &enum_name,
        generics: &ast.generics,
    };
    let map_impl = match map_type {
        MapType::HashMap | MapType::BTreeMap => {
            maps::generate_map_code(&ast, &map_type, enum_type, &key_enum_name)
        }
        MapType::Struct => {
            structs::generate_struct_code(&ast, &map_type, enum_type, &key_enum_name)
        }
    };

    let result = quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate enum_map as _enum_map;
            use _enum_map::common::*;
            use _enum_map::serde;

            #map_impl
        };
    };

    result.into()
}
