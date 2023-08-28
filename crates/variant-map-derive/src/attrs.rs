use std::ops::Deref;
use darling::{FromDeriveInput, FromMeta, FromVariant};
use quote::{format_ident, quote, ToTokens};
use syn::{DeriveInput, Ident, Variant, Visibility};
use crate::common::EnumType;

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