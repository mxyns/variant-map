use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DeriveInput, Generics};
use syn::spanned::Spanned;
use crate::attrs::MapType;
use crate::common;

pub(crate) fn generate_struct_code(ast: &mut DeriveInput, map_type: &MapType, enum_name: &Ident, key_enum_name: &Ident) -> TokenStream {
    match &mut ast.data {
        Data::Enum(ref mut enum_data) => {

            // TODO attribute for struct:
            // rename struct
            // TODO move all automatically derived inside the functions

            let struct_name = &format_ident!("{}StructMap", enum_name);

            let key_enum_quote = common::generate_key_enum(map_type, enum_data, key_enum_name);

            let enum_struct_quote = generate_enum_struct_code(enum_name, enum_data, key_enum_name, struct_name);

            let impl_struct_map_functions_quote = generate_enum_struct_impl(enum_name, enum_data, key_enum_name, struct_name);

            let impl_index = generate_impl_index(enum_name, enum_data, key_enum_name, struct_name);

            let impl_enum_map_value = generate_impl_map_value(struct_name, (&ast.generics, enum_name), enum_data, key_enum_name);

            quote! {
                #[automatically_derived]
                #key_enum_quote

                #[automatically_derived]
                #enum_struct_quote

                #[automatically_derived]
                #impl_struct_map_functions_quote

                #impl_index

                #[automatically_derived]
                #impl_enum_map_value
            }
        }
        _ => syn::Error::new(ast.span(), "EnumMap works only on enums").into_compile_error(),
    }
}

fn generate_enum_struct_impl(enum_name: &Ident, enum_data: &mut DataEnum, key_enum_name: &Ident, struct_name: &Ident) -> TokenStream {
    let fn_remove_match_body = common::enum_entries_map_to(enum_name, enum_data, key_enum_name, |_enum_name, _variant_name, _skip_fields, key_enum_name, key_name| {
        quote! {
            #key_enum_name::#key_name => std::mem::take(&mut self.#key_name)
        }
    });

    let fn_insert_match_body = common::enum_entries_map_to(enum_name, enum_data, key_enum_name, |enum_name, variant_name, skip_fields, _key_enum_name, key_name| {
        quote! {
            #enum_name::#variant_name #skip_fields => std::mem::replace(&mut self.#key_name, Some(value))
        }
    });

    let fn_get_match_body = common::enum_entries_map_to(enum_name, enum_data, key_enum_name, |_enum_name, _variant_name, _skip_fields, key_enum_name, key_name| {
        quote! {
            #key_enum_name::#key_name => &self.#key_name
        }
    });

    let fn_get_mut_match_body = common::enum_entries_map_to(enum_name, enum_data, key_enum_name, |_enum_name, _variant_name, _skip_fields, key_enum_name, key_name| {
        quote! {
            #key_enum_name::#key_name => &mut self.#key_name
        }
    });

    quote! {
        #[allow(dead_code)]
        impl #struct_name {
            fn remove(&mut self, key: &#key_enum_name) -> Option<#enum_name> {
                match key {
                    #fn_remove_match_body
                }
            }

            fn insert(&mut self, value: #enum_name) -> Option<#enum_name> {
                match value {
                    #fn_insert_match_body
                }
            }

            fn get(&self, key: &#key_enum_name) -> &Option<#enum_name> {
                match key {
                    #fn_get_match_body
                }
            }

            fn get_mut(&mut self, key: &#key_enum_name) -> &mut Option<#enum_name> {
                match key {
                    #fn_get_mut_match_body
                }
            }
        }
    }
}

fn generate_enum_struct_code(enum_name: &Ident, enum_data: &DataEnum, key_enum_name: &Ident, struct_name: &Ident) -> TokenStream {
    let fields = common::enum_entries_map_to(enum_name, enum_data, key_enum_name, |enum_name, _variant_name, _skip_fields, _key_enum_name, key_name| {
        quote! {
            #key_name: Option<#enum_name>
        }
    });

    quote! {
        #[derive(Debug, Default)]
        #[allow(non_snake_case)]
        pub struct #struct_name {
            #fields
        }
    }
}

fn generate_impl_index(enum_name: &Ident, enum_data: &DataEnum, key_enum_name: &Ident, struct_name: &Ident) -> TokenStream {

    let impl_index = {
        let match_body = common::enum_entries_map_to(enum_name, enum_data, key_enum_name, |_enum_name, _variant_name, _skip_fields, key_enum_name, key_name| {
            quote! {
                #key_enum_name::#key_name => &self.#key_name
            }
        });

        quote! {
            impl Index<#key_enum_name> for #struct_name {
                type Output = Option<#enum_name>;

                fn index(&self, index: #key_enum_name) -> &Self::Output {
                    match index {
                        #match_body
                    }
                }
            }
        }
    };

    let impl_index_mut = {
        let match_body = common::enum_entries_map_to(enum_name, enum_data, key_enum_name, |_enum_name, _variant_name, _skip_fields, key_enum_name, key_name| {
            quote! {
                #key_enum_name::#key_name => &mut self.#key_name
            }
        });

        quote! {
            impl IndexMut<#key_enum_name> for #struct_name {
                fn index_mut(&mut self, index: #key_enum_name) -> &mut Self::Output {
                    match index {
                        #match_body
                    }
                }
            }
        }
    };

    quote! {
        use std::ops::{Index, IndexMut};
        #[automatically_derived]
        #impl_index


        #[automatically_derived]
        #impl_index_mut
    }
}


fn generate_impl_map_value(
    struct_name: &Ident,
    enum_type: (&Generics, &Ident),
    enum_data: &DataEnum,
    key_enum_name: &Ident,
) -> TokenStream {
    let (generics, enum_name) = enum_type;

    let match_body = common::enum_entries_map_to(enum_name, enum_data, key_enum_name, |enum_name, variant_name, skip_fields, key_enum_name, key_name| {
        quote! {
            #enum_name::#variant_name #skip_fields => #key_enum_name::#key_name
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics MapValue for #enum_name #ty_generics #where_clause {
            type Key = #key_enum_name;
            type Map = #struct_name;

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