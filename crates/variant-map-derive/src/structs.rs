use crate::attrs::{StructAttr, MapType};
use crate::common;
use crate::common::EnumType;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{Data, DataEnum, DeriveInput, GenericParam, Lifetime, LifetimeParam, TypeGenerics, WhereClause};

/// Main function generating the entire code for a `StructMap`
pub(crate) fn generate_struct_code(
    ast: &DeriveInput,
    map_type: &MapType,
    enum_type: &EnumType,
    key_enum_name: &Ident,
) -> Result<(Option<TokenStream>, Option<TokenStream>), ()> {

    let struct_attr = &StructAttr::new(&ast);

    match &ast.data {
        Data::Enum(ref enum_data) => {

            let struct_name = &struct_attr.struct_name(enum_type);

            let key_enum_quote = common::generate_key_enum(map_type, struct_attr, enum_data, key_enum_name);

            let enum_struct_quote =
                generate_enum_struct_code(struct_attr, enum_type, enum_data, key_enum_name, struct_name);

            let impl_struct_map_functions_quote =
                generate_enum_struct_impl(enum_type, enum_data, key_enum_name, struct_name);

            let impl_map_value =
                generate_impl_map_value(struct_name, enum_type, enum_data, key_enum_name);

            let impl_index =
                if !struct_attr.features.use_index() { None }
                else { Some(generate_impl_index(enum_type, enum_data, key_enum_name, struct_name)) };

            let impl_serialize =
                if !struct_attr.features.use_serialize() { None }
                else { Some(generate_impl_serialize(struct_name, enum_type, enum_data, key_enum_name)) };

            let impl_deserialize =
                if !struct_attr.features.use_deserialize() { None }
                else { Some(generate_impl_deserialize(struct_name, enum_type, enum_data, key_enum_name)) };


            let (outside_const, inside_const) = common::in_or_out_scope(&struct_attr.visibility, quote! {
                #key_enum_quote

                #enum_struct_quote
            });

            let inside_const = quote! {
                #inside_const

                #impl_struct_map_functions_quote

                #impl_index

                #impl_map_value

                #impl_serialize

                #impl_deserialize
            };

            Ok((outside_const, Some(inside_const)))
        }
        _ => Err(()),
    }
}

/// Adds a trait as bound for each type in the where clause and a bound on the enum type
///
/// # Example
///
/// `where T: SomeTrait` => `where T: SomeTrait + NewBound, Enum: NewBound`
/// `None` => `where Enum: NewBound`
pub(crate) fn where_clause_add_enum_bound(where_clause: Option<&WhereClause>, enum_name: &Ident, type_generics: &TypeGenerics, bound: TokenStream) -> TokenStream {
    let new_enum_bound = quote! { #enum_name #type_generics: #bound };
    let where_clause = where_clause.map(|where_clause| {
        let where_clause = common::where_clause_add_trait(where_clause, bound);
        quote! {
            #where_clause, #new_enum_bound
        }
    }).unwrap_or_else(|| {
        quote! { where #new_enum_bound }
    });

    where_clause
}

/// Generates an implementation of [Serialize][serde::Serialize] for the target enum
///
/// Adds a bound on generics for [Serialize][serde::Serialize] on the impl block
///
/// Implemented only if the `StructMap` has the [feature][crate::attrs::StructMapFeaturesAttr] `serialize`
pub(crate) fn generate_impl_serialize(struct_name: &Ident, enum_type: &EnumType, enum_data: &DataEnum, key_enum_name: &Ident) -> TokenStream {
    let EnumType {
        enum_name,
        generics
    } = enum_type;

    let serialize_fields = common::enum_entries_map_to(enum_name, enum_data, key_enum_name, |_enum_name, _variant_name, _skip_fields, _key_enum_name, key_name| {
        quote! {
            if let Some(ref value) = self.#key_name { seq.serialize_element(value)? }
        }
    });
    let fields_len = enum_data.variants.len();

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    // Update where clause with Serialize trait
    let where_clause = where_clause_add_enum_bound(where_clause, enum_name, &type_generics, quote!(::serde::Serialize));

    quote! {
        use ::serde::ser::SerializeSeq;
        #[automatically_derived]
        impl #impl_generics ::serde::Serialize for #struct_name #type_generics #where_clause {
            fn serialize<__serde_S>(&self, serializer: __serde_S) -> Result<__serde_S::Ok, __serde_S::Error>
            where
                __serde_S: ::serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(Some(#fields_len))?;

                #serialize_fields

                seq.end()
            }
        }
    }
}

/// Generates an implementation of [Deserialize<'_serde_deserializer_lifetime_de>][serde::Deserialize] for the target enum
///
/// Adds a bound on generics for [Deserialize<'_serde_deserializer_lifetime_de>][serde::Deserialize] on the impl block
///
/// The lifetime used is always `'_serde_deserializer_lifetime_de` make sure there are no collisions (which is unlikely)
/// in the definition of the target enum
///
/// The visitor for the deserialization is called `__VariantStore__StructMap__{EnumName}__Visitor`, make sure there are collisions too
///
/// Implemented only if the `StructMap` has the [feature][crate::attrs::StructMapFeaturesAttr] `deserialize`
fn generate_impl_deserialize(struct_name: &Ident, enum_type: &EnumType, enum_data: &DataEnum, key_enum_name: &Ident) -> TokenStream {
    let EnumType {
        enum_name,
        generics
    } = enum_type;

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let visitor = format_ident!("__VariantStore__StructMap__{}__Visitor", key_enum_name);
    let phantom = if generics.params.is_empty() { None } else {
        Some(
            quote! {
                (PhantomData #type_generics)
            }
        )
    };

    let visitor_quote = quote! {
        use core::marker::PhantomData;
        struct #visitor #type_generics #phantom;
    };

    let deser_lifetime = quote!('_serde_deserializer_lifetime_de);
    let mut generics = (*generics).clone();
    generics.params.push(GenericParam::Lifetime(LifetimeParam::new(Lifetime::new("'_serde_deserializer_lifetime_de", impl_generics.span()))));

    // Update where clause with Deserialize trait
    let where_clause = where_clause_add_enum_bound(where_clause, enum_name, &type_generics, quote!(::serde::Deserialize<#deser_lifetime>));

    let (impl_generics, _, _) = generics.split_for_impl();

    let deser_match = common::enum_entries_map_to(enum_name, enum_data, key_enum_name, |enum_name, variant_name, skip_fields, _key_enum_name, key_name| {
        quote! { Some(#enum_name::#variant_name #skip_fields) => result.#key_name = elem, }
    });

    let expected_msg = format!("{visitor} expects a {struct_name} holding {enum_name} variants");

    let impl_visitor = quote! {
        impl #impl_generics ::serde::de::Visitor<#deser_lifetime> for #visitor #type_generics #where_clause {
            type Value = #struct_name #type_generics;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(#expected_msg)
            }

            fn visit_seq<__serde__A>(self, mut seq: __serde__A) -> core::result::Result<Self::Value, __serde__A::Error>
                where
                    __serde__A: ::serde::de::SeqAccess<#deser_lifetime>,
            {
                let mut result = #struct_name::default();

                while let Some(elem) = seq.next_element::<Option<#enum_name #type_generics>>()? {
                    match elem {
                        #deser_match
                        None => {}
                    }
                }

                Ok(result)
            }
        }
    };

    let visitor_init = if phantom.is_some() {
        quote!{ #visitor (PhantomData::default())}
    } else {
        quote!( #visitor {} )
    };

    let impl_deserialize_struct = quote! {
        impl #impl_generics Deserialize<#deser_lifetime> for #struct_name #type_generics #where_clause {
            fn deserialize<__serde_D>(deserializer: __serde_D) -> core::result::Result<Self, __serde_D::Error>
            where
                __serde_D: ::serde::de::Deserializer<#deser_lifetime>,
            {
                deserializer.deserialize_seq( #visitor_init )
            }
        }
    };

    quote! {
        #[automatically_derived]
        #visitor_quote

        #[automatically_derived]
        #impl_visitor

        #[automatically_derived]
        #impl_deserialize_struct
    }
}

/// Implements base methods on the `StructMap`
///
/// Defines `get` `get_mut` `insert` `remove`
pub(crate) fn generate_enum_struct_impl(
    enum_type: &EnumType,
    enum_data: &DataEnum,
    key_enum_name: &Ident,
    struct_name: &Ident,
) -> TokenStream {
    let EnumType { enum_name, .. } = enum_type;

    let fn_remove_match_body = common::enum_entries_map_to(
        enum_name,
        enum_data,
        key_enum_name,
        |_enum_name, _variant_name, _skip_fields, key_enum_name, key_name| {
            quote! {
                #key_enum_name::#key_name => std::mem::take(&mut self.#key_name),
            }
        },
    );

    let fn_insert_match_body = common::enum_entries_map_to(
        enum_name,
        enum_data,
        key_enum_name,
        |enum_name, variant_name, skip_fields, _key_enum_name, key_name| {
            quote! {
                #enum_name::#variant_name #skip_fields => std::mem::replace(&mut self.#key_name, Some(value)),
            }
        },
    );

    let fn_get_match_body = common::enum_entries_map_to(
        enum_name,
        enum_data,
        key_enum_name,
        |_enum_name, _variant_name, _skip_fields, key_enum_name, key_name| {
            quote! {
                #key_enum_name::#key_name => &self.#key_name,
            }
        },
    );

    let fn_get_mut_match_body = common::enum_entries_map_to(
        enum_name,
        enum_data,
        key_enum_name,
        |_enum_name, _variant_name, _skip_fields, key_enum_name, key_name| {
            quote! {
                #key_enum_name::#key_name => &mut self.#key_name,
            }
        },
    );

    let (impl_generics, type_generics, where_clause) = enum_type.generics.split_for_impl();
    let enum_name_w_generics = quote! {
        #enum_name #type_generics
    };

    quote! {
        #[automatically_derived]
        #[allow(dead_code)]
        impl #impl_generics #struct_name #type_generics #where_clause {
            fn remove(&mut self, key: &#key_enum_name) -> Option<#enum_name_w_generics> {
                match key {
                    #fn_remove_match_body
                }
            }

            fn insert(&mut self, value: #enum_name_w_generics) -> Option<#enum_name_w_generics> {
                match value {
                    #fn_insert_match_body
                }
            }

            fn get(&self, key: &#key_enum_name) -> &Option<#enum_name_w_generics> {
                match key {
                    #fn_get_match_body
                }
            }

            fn get_mut(&mut self, key: &#key_enum_name) -> &mut Option<#enum_name_w_generics> {
                match key {
                    #fn_get_mut_match_body
                }
            }
        }
    }
}

/// Generates the code for the `StructMap` definition and implements [Default][std::default::Default] on it
pub(crate) fn generate_enum_struct_code(
    map_attr: &StructAttr,
    enum_type: &EnumType,
    enum_data: &DataEnum,
    key_enum_name: &Ident,
    struct_name: &Ident,
) -> TokenStream {
    let EnumType { enum_name, .. } = enum_type;

    let (impl_generics, type_generics, where_clause) = enum_type.generics.split_for_impl();

    let fields = common::enum_entries_map_to(
        enum_name,
        enum_data,
        key_enum_name,
        |enum_name, _variant_name, _skip_fields, _key_enum_name, key_name| {
            quote! {
                #key_name: Option<#enum_name #type_generics>,
            }
        },
    );

    let fields_none = common::enum_entries_map_to(
        enum_name,
        enum_data,
        key_enum_name,
        |_enum_name, _variant_name, _skip_fields, _key_enum_name, key_name| {
            quote! {
                #key_name: None,
            }
        },
    );

    let vis = &map_attr.visibility;
    let derives = map_attr.derives();
    quote! {
        #[automatically_derived]
        #[derive(Debug, #derives)]
        #[allow(non_snake_case)]
        #vis struct #struct_name #type_generics #where_clause  {
            #fields
        }

        #[automatically_derived]
        impl #impl_generics Default for #struct_name #type_generics #where_clause {
            fn default() -> Self {
                #struct_name {
                    #fields_none
                }
            }
        }
    }
}

/// Implements [Index][std::ops::Index] and [IndexMut][std::ops::IndexMut] for the `StructMap`
///
/// Implemented only if the `StructMap` has the [feature][crate::attrs::StructMapFeaturesAttr] `index`
pub(crate) fn generate_impl_index(
    enum_type: &EnumType,
    enum_data: &DataEnum,
    key_enum_name: &Ident,
    struct_name: &Ident,
) -> TokenStream {
    let EnumType { enum_name, .. } = enum_type;

    let (impl_generics, type_generics, where_clause) = enum_type.generics.split_for_impl();

    let impl_index = {
        let match_body = common::enum_entries_map_to(
            enum_name,
            enum_data,
            key_enum_name,
            |_enum_name, _variant_name, _skip_fields, key_enum_name, key_name| {
                quote! {
                    #key_enum_name::#key_name => &self.#key_name,
                }
            },
        );

        quote! {
            impl #impl_generics Index<#key_enum_name> for #struct_name #type_generics #where_clause {
                type Output = Option<#enum_name #type_generics>;

                fn index(&self, index: #key_enum_name) -> &Self::Output {
                    match index {
                        #match_body
                    }
                }
            }
        }
    };

    let impl_index_mut = {
        let match_body = common::enum_entries_map_to(
            enum_name,
            enum_data,
            key_enum_name,
            |_enum_name, _variant_name, _skip_fields, key_enum_name, key_name| {
                quote! {
                    #key_enum_name::#key_name => &mut self.#key_name,
                }
            },
        );

        quote! {
            impl #impl_generics IndexMut<#key_enum_name> for #struct_name #type_generics #where_clause {
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

/// Implement the `MapValue` trait from `variant_map` for the Enum
///
/// This binds the Enum to its `StructMap` and its Key enum
pub(crate) fn generate_impl_map_value(
    struct_name: &Ident,
    enum_type: &EnumType,
    enum_data: &DataEnum,
    key_enum_name: &Ident,
) -> TokenStream {
    let EnumType {
        generics,
        enum_name,
    } = enum_type;

    let match_body = common::enum_entries_map_to(
        enum_name,
        enum_data,
        key_enum_name,
        |enum_name, variant_name, skip_fields, key_enum_name, key_name| {
            quote! {
                #enum_name::#variant_name #skip_fields => #key_enum_name::#key_name,
            }
        },
    );

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        #[automatically_derived]
        impl #impl_generics MapValue for #enum_name #ty_generics #where_clause {
            type Key = #key_enum_name;
            type Map = #struct_name #ty_generics;

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
