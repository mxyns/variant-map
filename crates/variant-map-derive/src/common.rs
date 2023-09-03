use crate::attrs::{KeyNameAttr, MapType, BaseAttr, OptionalVisibility};
use darling::FromVariant;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Generics, Ident, Variant, WhereClause, WherePredicate};
use syn::TypeParamBound::Verbatim;

/// All required information about the type of an enum
pub struct EnumType<'a> {
    pub(crate) generics: &'a Generics,
    pub(crate) enum_name: &'a Ident,
}

/// Generates an enum of Keys for the specified enum
pub(crate) fn generate_key_enum(
    map_type: &MapType,
    map_attr: &BaseAttr,
    enum_data: &DataEnum,
    key_enum_name: &Ident,
) -> TokenStream {
    let key_variants = enum_data.variants.iter().map(|variant| {
        let key_name_attr = KeyNameAttr::from_variant(variant).expect("Wrong key_name options");

        // Useful in case of variant identifier renaming
        let key_name = key_name_attr.key_name(variant);
        let serde_rename = key_name_attr.serde_rename().as_ref().map(|name| quote!{
            #[serde(rename=#name)]
        });

        quote! {
            #serde_rename


            #key_name
        }
    });

    let derives = map_attr.keys_derive();
    let derives_quote = match map_type {
        MapType::HashMap => {
            quote! { #[derive(Debug, PartialEq, Eq, Hash, #derives)] }
        }
        MapType::BTreeMap => {
            quote! { #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, #derives)] }
        }
        MapType::Struct => {
            quote! {#[derive(Debug, #derives)] }
        }
    };

    let vis = &map_attr.visibility;
    quote! {
        #[automatically_derived]
        #derives_quote
        #vis enum #key_enum_name {
            #(#key_variants),*
        }
    }
}

/// Decide whether the input should be in scope or not
///
/// # Arguments
///
/// `visibility`: used to determine if the provided input should be in or out of scope.
///
/// `possibly_outside`: input that was supposed to be in scope, but may be moved out of it.
///
/// # Returns
///
/// `(stay_in_scope, out_of_scope): (Option<T>, Option<T>)`
pub(crate) fn in_or_out_scope<T>(visibility: &OptionalVisibility, possibly_in_scope: T) -> (Option<T>, Option<T>) {

    if let &OptionalVisibility::OutOfScope = &visibility {
        (None, Some(possibly_in_scope))
    } else {
        (Some(possibly_in_scope), None)
    }
}

/// Iterate on the variants of a map and generate a [TokenStream]
/// The result contains the agglomeration of each [TokenStream] returned by `to: F` applied on each variant
pub(crate) fn enum_entries_map_to<F>(
    enum_name: &Ident,
    enum_data: &DataEnum,
    key_enum_name: &Ident,
    to: F,
) -> TokenStream
where
    F: Fn(&Ident, &Ident, Option<TokenStream>, &Ident, &Ident) -> TokenStream,
{
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
            Some(quote!((..)))
        } else {
            None
        };

        to(enum_name, ident, skip_fields, key_enum_name, key_name)
    });

    quote! {
        #(#match_cases)*
    }
}

/// Add a [Verbatim] bound to all bounded types of a where clause
pub(crate) fn where_clause_add_trait(where_clause: &WhereClause, the_trait: TokenStream) -> WhereClause {
    let mut cloned = where_clause.clone();
    for predicate in cloned.predicates.iter_mut() {
        if let WherePredicate::Type(value) = predicate {
            value.bounds.push(Verbatim(quote!(#the_trait)));
        }
    }

    cloned
}