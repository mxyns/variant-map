use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Variant};

// TODO rename keys and key enum
// TODO macro_rules for key and map syntactic sugar
// TODO Index and IndexMut syntactic sugar
// TODO choose map/table implementation with a derive attribute
// TODO doc
// TODO publish
#[proc_macro_derive(EnumMap, attributes(unimarc))]
pub fn derive(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    let enum_name = &ast.ident;

    let key_enum = match &mut ast.data {
        syn::Data::Enum(ref mut enum_data) => {
            let key_enum_name = format_ident!("{}Key", enum_name);
            let key_enum_quote = {
                let key_variants = enum_data.variants.iter().map(|variant| {
                    let key_name = variant.ident.clone();

                    // Useful in case of variant identifier renaming
                    let serde_rename = format!("{}", variant.ident);

                    quote! {
                        #[serde(rename=#serde_rename)]


                        #key_name
                    }
                });
                quote! {
                    #[derive(Debug, PartialEq, Eq, Hash, _enum_map::serde::Serialize, _enum_map::serde::Deserialize)]
                    enum #key_enum_name {
                        #(#key_variants),*
                    }
                }
            };

            let impl_enum_map_value_for_enum_quote = {
                let match_case = enum_data.variants.iter().map(|variant| {
                    let Variant {
                        attrs: _,
                        ident,
                        fields,
                        discriminant: _,
                    } = variant;
                    let key_name = format_ident!("{}", ident);
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
                    impl _enum_map::EnumMapValue for #enum_name {
                        type Key = #key_enum_name;
                        type Map = _enum_map::EnumMap<Self::Key, Self>;


                        fn to_key(&self) -> Self::Key {
                            match self {
                                #(#match_case),*
                            }
                        }
                    }
                }
            };

            // TODO merge with impl EnumMapValue
            let impl_enum_methods_quote = {
                quote! {
                    impl #enum_name {
                        pub fn make_map() -> <#enum_name as _enum_map::EnumMapValue>::Map {
                            _enum_map::EnumMap::default()
                        }
                    }
                }
            };

            let impl_hash_key_for_enum_key_quote = quote! {
                impl _enum_map::HashKey for #key_enum_name {}
            };

            quote! {
                #[doc(hidden)]
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate enum_map as _enum_map;

                    #key_enum_quote

                    #[automatically_derived]
                    #impl_enum_map_value_for_enum_quote

                    #[automatically_derived]
                    #impl_enum_methods_quote

                    #[automatically_derived]
                    #impl_hash_key_for_enum_key_quote
                };
            }
        }
        _ => panic!("EnumMap works only on enums"),
    };

    let result = quote! {
        #key_enum
    };

    result.into()
}