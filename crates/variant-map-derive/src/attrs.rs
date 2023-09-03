use std::ops::Deref;
use darling::{Error, FromDeriveInput, FromMeta, FromVariant};
use darling::util::PathList;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{DeriveInput, Expr, Ident, Lit, Variant, Visibility};
use crate::common::EnumType;

/// Attribute macro `key_name`
/// Applied on an enum variant to specify its key's name in the `code` and when de/serialized by [serde]
///
/// # Example
///
/// ```
/// use variant_map_derive::VariantStore;
///
/// #[derive(VariantStore)]
/// enum MyEnum {
///     A,
///     #[key_name(code = "Bamboo", serde = "bamboo")]
///     B
/// }
///
/// fn main() {
///     let key: MyEnumKey = MyEnumKey::Bamboo;
///     assert_eq!("bamboo", serde_json::to_string(&key).unwrap().as_str());
/// }
/// ```
#[derive(FromVariant, Default, Debug)]
#[darling(default, attributes(key_name))]
pub(crate) struct KeyNameAttr {
    /// Name of the enum variant in the code
    code: Option<String>,

    /// Name of the enum variant when de(serialized) by [serde]
    serde: Option<String>
}

impl KeyNameAttr {
    pub(crate) fn key_name(&self, variant: &Variant) -> Ident {
        self.code
            .as_ref()
            .map(|code| format_ident!("{}", code))
            .unwrap_or_else(|| variant.ident.clone())
    }

    pub(crate) fn serde_rename(&self) -> &Option<String> {
        &self.serde
    }
}

/// Parameters of the [crate::VariantStore] macro
///
/// # Arguments
///
/// `datastruct` : any of { `HashMap`, `BTreeMap`, `StructMap` }
///
/// default is `HashMap`
///
///
/// `keys` : specify the parameters for the generated enum of keys
///
/// see [crate::attrs::BaseKeysAttr]
///
///
/// `visibility` : specify the [Visibility][::syn::Visibility] of the generated enums / structs
/// associated to the target enum
///
/// default is private
///
/// # Example
///
/// ```
/// use variant_map_derive::VariantStore;
///
/// #[derive(VariantStore)]
/// #[VariantStore(keys(name = "MySuperKeys"), datastruct = "BTreeMap", visibility="pub(crate)")]
/// enum MyEnum {
///     A
/// }
///
/// fn main() {
///     let key: MySuperKeys = MySuperKeys::A;
///     // see macro expansion to check that the used inner map is a BTreeMap
///     // and that the keys have pub(crate) visibility
/// }
#[derive(Default, Debug, FromDeriveInput)]
#[darling(default, attributes(VariantStore))]
pub(crate) struct BaseAttr {
    /// [Type of the data structure][MapType] generated as a String
    pub(crate) datastruct: Option<String>,

    /// Name of the generate Key enum
    pub(crate) keys: Option<BaseKeysAttr>,

    /// Visibility of the generated Key enum and other structs
    #[darling(with="parse_visibility")]
    pub(crate) visibility: OptionalVisibility,
}

/// Either an [OptionalVisibility::OutOfScope] or a classic [Visibility]
///
/// see [BaseAttr] and [parse_visibility] for more details
#[derive(Debug)]
pub(crate) enum OptionalVisibility {
    /// User specified the `out-of-scope` Visibility
    OutOfScope,
    Specified(Visibility)
}

impl Default for OptionalVisibility {
    fn default() -> Self {
        Self::Specified(Visibility::Inherited)
    }
}

impl ToTokens for OptionalVisibility {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            OptionalVisibility::OutOfScope => { Visibility::Inherited.to_tokens(tokens )}
            OptionalVisibility::Specified(ref vis) => { vis.to_tokens(tokens) }
        }
    }
}

/// Parses the `visibility` ([OptionalVisibility]) attribute of [BaseAttr] from a String literal
///
/// Unspecified is [OptionalVisibility::default]
///
/// `out-of-scope` is [OptionalVisibility::OutOfScope] meaning the types won't be accessible
///
/// Others are classic visibility tokens
pub(crate) fn parse_visibility(meta: &syn::Meta) -> Result<OptionalVisibility, Error> {
    let value = &meta.require_name_value()?.value;

    let literal = match value {
        Expr::Lit(ref lit) => {
            lit
        }
        _ => return Err(Error::unexpected_expr_type(value))
    };

    let literal_str = match &literal.lit {
        Lit::Str(ref str) => {
            str.value()
        }
        _ => return Err(Error::unexpected_lit_type(&literal.lit))
    };

    let optional_visibility = match literal_str.as_str() {
        "out-of-scope" => OptionalVisibility::OutOfScope,
        _ => OptionalVisibility::Specified(Visibility::from_expr(&value)?)
    };

    Ok(optional_visibility)
}

/// Parameters of the Key enum given in [BaseAttr] (`VariantStore` parameter macro)
///
/// # Arguments
///
/// `name` : name of the generated Key Enum
///
/// `derive` : additional derives on the Key Enum
///
/// # Example
///
/// ```
/// use variant_map_derive::VariantStore;
///
/// #[derive(VariantStore)]
/// #[VariantStore(keys(name = "MySuperKeys", derive(::serde::Serialize)), datastruct = "BTreeMap", visibility="pub(crate)")]
/// enum MyEnum {
///     A
/// }
///
/// fn main() {
///     let key: MySuperKeys = MySuperKeys::A;
///
///     // Thanks to the "derive(::serde::Serialize)"
///     println!("{}", serde_json::to_string(&key));
///     // see macro expansion to check that the used inner map is a BTreeMap
///     // and that the keys have pub(crate) visibility
/// }
#[derive(Debug, Default, FromMeta)]
pub(crate) struct BaseKeysAttr {
    pub(crate) name: Option<String>,
    pub(crate) derive: Option<PathList>
}

impl BaseAttr {
    pub(crate) fn keys_name(&self, enum_name: Ident) -> Ident {
        self.keys
            .as_ref()
            .map(|attrs| attrs.name.as_ref())
            .flatten()
            .map(|name| format_ident!("{}", name))
            .unwrap_or(enum_name)
    }

    pub(crate) fn keys_derive(&self) -> Option<TokenStream> {
        let path_list = self.keys
            .as_ref()
            .map(|attrs| attrs.derive.as_ref())
            .flatten();

        get_derives(path_list)

    }

    pub(crate) fn map_type(&self) -> MapType {
        if let Some(name) = &self.datastruct {
            MapType::try_from(name).unwrap()
        } else {
            MapType::default()
        }
    }
}

pub(crate) fn get_derives(path_list: Option<&PathList>) -> Option<TokenStream> {
    path_list.map(|list| {
        let quotes = list.iter().map(|v| v.into_token_stream());
        quote!{ #(#quotes),* }
    })
}


/// Parameter macro `VariantMap` of the map types
/// Has no specific attributes for now
///
/// # Example
///
/// ```
/// use variant_map_derive::VariantStore;
///
/// #[derive(VariantStore)]
/// #[VariantMap(/* nothing to do here */)]
/// enum MyEnum {
///     A
/// }
///
/// fn main() {
///     let key: MyEnumKey = MyEnumKey::A;
/// }
///
#[derive(Default, Debug, FromDeriveInput)]
#[darling(default, attributes(VariantMap))]
pub(crate) struct MapAttr {
    #[darling(skip)]
    base: BaseAttr,
}

impl MapAttr {
    pub(crate) fn new(ast: &DeriveInput) -> Self {
        Self {
            base: BaseAttr::from_derive_input(&ast).expect("Wrong VariantStore parameters"),
            ..Self::from_derive_input(&ast).expect("Wrong VariantMap parameters")
        }
    }
}

impl From<BaseAttr> for MapAttr {
    fn from(base: BaseAttr) -> Self {
        Self {
            base
        }
    }
}

impl Deref for MapAttr {
    type Target = BaseAttr;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Parameter macro `VariantStruct` of the `StructMap` type
///
/// # Arguments
///
/// `name` : name of the generated struct
///
/// `derive`: additional derives on the `StructMap`
///
/// `features` : list of features (see [features][StructMapFeaturesAttr])
///
/// # Example
///
/// ```
/// use variant_map_derive::VariantStore;
///
/// #[derive(Clone, VariantStore)]
/// #[VariantStore(datastruct = "StructMap")]
/// #[VariantStruct(name = "MySuperStruct", features(serialize, index), derive(Clone))]
/// enum MyEnum {
///     A(i32)
/// }
///
/// fn main() {
///     let key: MyEnumKey = MyEnumKey::A;
///     let mut map: MySuperStruct = MySuperStruct::default();
///     map.A = Some(MyEnum::A(1));
///     map.insert(MyEnum::A(1));
/// }
///
#[derive(FromDeriveInput, Default, Debug)]
#[darling(default, attributes(VariantStruct))]
pub(crate) struct StructAttr {
    #[darling(skip)]
    base: BaseAttr,
    name: Option<String>,
    derive: Option<PathList>,
    pub(crate) features: StructMapFeaturesAttr,
}

impl StructAttr {
    pub(crate) fn new(ast: &DeriveInput) -> Self {
        Self {
            base: BaseAttr::from_derive_input(&ast).expect("Wrong VariantStore parameters"),
            ..StructAttr::from_derive_input(&ast).expect("Wrong VariantStruct parameters")
        }
    }

    pub(crate) fn struct_name(&self, enum_type: &EnumType) -> Ident {
        if let Some(ref name) = self.name {
            Ident::from_string(name.as_str()).unwrap()
        } else {
            format_ident!("{}StructMap", enum_type.enum_name)
        }
    }

    pub(crate) fn derives(&self) -> Option<TokenStream> {
        get_derives(self.derive.as_ref())
    }
}

impl Deref for StructAttr {
    type Target = BaseAttr;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Features for the `StructMap` derive
///
/// # Arguments
///
/// `serialize` if present, will derive [Serialize][serde::Serialize] for the struct
///
/// `deserialize` if present, will derive [Deserialize][serde::Deserialize] for the struct
///
/// `index` if present, will derive [Index][std::ops::Index] and [IndexMut][std::ops::IndexMut] for the struct
///
/// # Example
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use variant_map_derive::VariantStore;
///
/// #[derive(Debug, Serialize, Deserialize, VariantStore)]
/// #[VariantStore(keys(name = "TestKeys"), datastruct = "StructMap")]
/// #[VariantStruct(features(index, serialize, deserialize))]
/// enum TestEnum {
///     A,
///     B(i32),
/// }
/// ```
///
#[derive(Default, Debug, FromMeta)]
pub(crate) struct StructMapFeaturesAttr {
    serialize: Option<()>,
    deserialize: Option<()>,
    index: Option<()>,
}

impl StructMapFeaturesAttr {
    pub(crate) fn use_serialize(&self) -> bool { self.serialize.is_some() }
    pub(crate) fn use_deserialize(&self) -> bool { self.deserialize.is_some() }
    pub(crate) fn use_index(&self) -> bool { self.index.is_some() }
}

/// Type of map selected with the `datastruct` attribute of [VariantStore][BaseAttr]
///
/// [MapType::HashMap] is from value `HashMap`
/// [MapType::BTreeMap] is from value `BTreeMap`
/// [MapType::Struct] is from value `StructMap`
///
#[derive(Default, Debug)]
pub(crate) enum MapType {
    #[default]
    HashMap,
    BTreeMap,
    Struct,
}

impl TryFrom<&String> for MapType {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "hashmap" => Ok(Self::HashMap),
            "btreemap" => Ok(Self::BTreeMap),
            "structmap" => Ok(Self::Struct),
            _ => Err("Invalid 'map' argument, available {{ \"hashmap\", \"btreemap\" }}".into()),
        }
    }
}

impl ToTokens for MapType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let token = match self {
            MapType::HashMap => {
                quote!(hashmap)
            }
            MapType::BTreeMap => {
                quote!(btreemap)
            }
            MapType::Struct => {
                quote!()
            }
        };

        token.to_tokens(tokens);
    }
}