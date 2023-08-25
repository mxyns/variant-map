use darling::{FromDeriveInput, FromVariant};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Ident, Variant};


#[derive(FromVariant, Default, Debug)]
#[darling(default, attributes(key_name))]
struct KeyNameAttr {
    code: Option<String>,
    serde: Option<String>,
}

impl KeyNameAttr {
    fn key_name(&self, variant: &Variant) -> Ident {
        self.code
            .as_ref()
            .map(|code| format_ident!("{}", code))
            .unwrap_or_else(|| variant.ident.clone())
    }

    fn serde_rename(&self, variant: &Variant) -> String {
        self.serde
            .clone()
            .unwrap_or_else(|| variant.ident.to_string())
    }
}

#[derive(FromDeriveInput, Default, Debug)]
#[darling(default, attributes(EnumMap))]
struct MapAttr {
    name: Option<String>,
    map: Option<String>
}

impl MapAttr {
    fn enum_name(&self, enum_name: Ident) -> Ident {
        self.name
            .as_ref()
            .map(|name| format_ident!("{}", name))
            .unwrap_or_else(|| enum_name)
    }

    fn map_impl_mod(&self) -> proc_macro2::TokenStream {
        match self.map.clone().map(|name| name.to_lowercase()) {
            Some(name) if name.as_str() == "hashmap"
                || name.as_str() == "btreemap" => {
                str::parse::<TokenStream>(name.as_str()).unwrap().into()
            }
            None => { quote!(hashmap) }, // default value
            _ => {
                panic!("Invalid 'map' argument, available {{ \"hashmap\", \"btreemap\" }}")
            }
        }
    }
}

// TODO [x] rename keys and key enum
// TODO [x] merge make_map with impl EnumMapValue
// TODO [x] macro_rules for key and map syntactic sugar
// TODO [x] Index and IndexMut syntactic sugar
// TODO [x] choose map/table implementation with a derive attribute
// TODO cleanup -derive code and split into functions
// TODO add struct version of the map
// TODO? tight couple EnumMap and EnumMapValue if possible
// TODO doc
// TODO publish
#[proc_macro_derive(EnumMap, attributes(EnumMap, key_name))]
pub fn derive_enum_map(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    let enum_name = &ast.ident;

    let (
        key_enum_name,
        impl_base
    ) = {
        let key_enum_name_attr =
            MapAttr::from_derive_input(&ast).expect("Wrong enum_name options");
        (
            key_enum_name_attr.enum_name(format_ident!("{}Key", enum_name)),
            key_enum_name_attr.map_impl_mod()
        )
    };

    let key_enum = match &mut ast.data {
        syn::Data::Enum(ref mut enum_data) => {
            let key_enum_quote = {
                let key_variants = enum_data.variants.iter().map(|variant| {
                    let key_name_attr =
                        KeyNameAttr::from_variant(variant).expect("Wrong key_name options");

                    // Useful in case of variant identifier renaming
                    let key_name = key_name_attr.key_name(variant);
                    let serde_rename = key_name_attr.serde_rename(variant);

                    quote! {
                        #[serde(rename=#serde_rename)]


                        #key_name
                    }
                });

                quote! {
                    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::serde::Serialize, ::serde::Deserialize)]
                    enum #key_enum_name {
                        #(#key_variants),*
                    }
                }
            };

            let impl_enum_map_value_for_enum_quote = {
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

                quote! {
                    impl MapValue for #enum_name {
                        type Key = #key_enum_name;
                        type Map = Map<Self::Key, Self>;

                        fn to_key(&self) -> Self::Key {
                            match self {
                                #(#match_case),*
                            }
                        }


                        fn make_map() -> <#enum_name as MapValue>::Map {
                           Self::Map::default()
                        }
                    }
                }
            };

            let impl_hash_key_for_enum_key_quote = quote! {
                impl HashKey for #key_enum_name {}
            };

            quote! {
                #[doc(hidden)]
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate enum_map as _enum_map;
                    use _enum_map::common::*;
                    use _enum_map::#impl_base::*;
                    use _enum_map::serde;

                    #key_enum_quote

                    #[automatically_derived]
                    #impl_enum_map_value_for_enum_quote

                    #[automatically_derived]
                    #impl_hash_key_for_enum_key_quote
                };
            }
        }
        _ => syn::Error::new(ast.span(), "EnumMap works only on enums")
            .into_compile_error()
    };

    let result = quote! {
        #key_enum
    };

    result.into()
}
