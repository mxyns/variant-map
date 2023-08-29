use std::ops::Deref;
use darling::{FromDeriveInput, FromMeta, FromVariant};
use quote::{format_ident, quote, ToTokens};
use syn::{DeriveInput, Ident, Variant, Visibility};
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
    code: Option<String>,
    serde: Option<String>,
}

impl KeyNameAttr {
    pub(crate) fn key_name(&self, variant: &Variant) -> Ident {
        self.code
            .as_ref()
            .map(|code| format_ident!("{}", code))
            .unwrap_or_else(|| variant.ident.clone())
    }

    pub(crate) fn serde_rename(&self, variant: &Variant) -> String {
        self.serde
            .clone()
            .unwrap_or_else(|| variant.ident.to_string())
    }
}

/// Parameters of the [crate::VariantStore] macro
///
/// # Example
///
/// ```
/// use variant_map_derive::VariantStore;
///
/// #[derive(VariantStore)]
/// #[VariantStore(keys = "MySuperKeys", datastruct = "BTreeMap", visibility="pub(crate)")]
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
    pub(crate) datastruct: Option<String>,
    pub(crate) keys: Option<String>,
    pub(crate) visibility: Option<Visibility>,
}

impl BaseAttr {
    pub(crate) fn keys_name(&self, enum_name: Ident) -> Ident {
        self.keys
            .as_ref()
            .map(|name| format_ident!("{}", name))
            .unwrap_or_else(|| enum_name)
    }

    pub(crate) fn map_type(&self) -> MapType {
        if let Some(name) = &self.datastruct {
            MapType::try_from(name).unwrap()
        } else {
            MapType::default()
        }
    }
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
/// `features` : list of features (see [features][StructMapFeaturesAttr])
///
/// # Example
///
/// ```
/// use variant_map_derive::VariantStore;
///
/// #[derive(VariantStore)]
/// #[VariantStore(datastruct = "StructMap")]
/// #[VariantStruct(name = "MySuperStruct", features(serialize, index))]
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
/// #[VariantStore(keys = "TestKeys", datastruct = "StructMap")]
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