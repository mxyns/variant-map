use darling::{FromDeriveInput, FromMeta, FromVariant};
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, Variant};

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

#[derive(FromDeriveInput, Default, Debug)]
#[darling(default, attributes(EnumMap))]
pub(crate) struct MapAttr {
    keys: Option<String>,
    map: Option<String>,
    pub(crate) struct_features: StructMapFeaturesAttr,
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

impl MapAttr {
    pub(crate) fn keys_name(&self, enum_name: Ident) -> Ident {
        self.keys
            .as_ref()
            .map(|name| format_ident!("{}", name))
            .unwrap_or_else(|| enum_name)
    }

    pub(crate) fn map_type(&self) -> MapType {
        if let Some(name) = &self.map {
            MapType::try_from(name).unwrap()
        } else {
            MapType::default()
        }
    }
}
