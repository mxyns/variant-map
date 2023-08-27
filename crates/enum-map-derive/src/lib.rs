mod maps;
mod common;
mod attrs;

use darling::{FromDeriveInput};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};
use crate::attrs::{MapAttr, MapType};

// TODO [x] rename keys and key enum
// TODO [x] merge make_map with impl EnumMapValue
// TODO [x] macro_rules for key and map syntactic sugar
// TODO [x] Index and IndexMut syntactic sugar
// TODO [x] choose map/table implementation with a derive attribute
// TODO [3/3] cleanup -derive code, split into functions, into different files
// TODO [2/2] handle generics + bounds
// TODO add struct and array versions of the "map"
// TODO? tight couple Map and MapValue if possible
// TODO doc
// TODO publish

#[proc_macro_derive(EnumMap, attributes(EnumMap, key_name))]
pub fn derive_enum_map(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    let enum_name = &ast.ident.clone();

    // EnumMap attribute parameters
    let (key_enum_name, map_type) = {
        let key_enum_name_attr = MapAttr::from_derive_input(&ast).expect("Wrong enum_name options");
        (
            key_enum_name_attr.enum_name(format_ident!("{}Key", enum_name)),
            key_enum_name_attr.map_impl_mod(),
        )
    };

    let key_enum = match map_type {
        MapType::HashMap
        | MapType::BTreeMap => {
            maps::generate_map_code(&mut ast, &map_type, &enum_name, &key_enum_name)
        }
    };

    let result = quote! {
        #key_enum
    };

    result.into()
}
