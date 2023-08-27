use darling::{FromDeriveInput, FromVariant};
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
    name: Option<String>,
    map: Option<String>,
}

#[derive(Default, Debug)]
pub(crate) enum MapType {
    #[default]
    HashMap,
    BTreeMap,
    StructMap
}

impl TryFrom<&String> for MapType {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "hashmap" => Ok(Self::HashMap),
            "btreemap" => Ok(Self::BTreeMap),
            "structmap" => Ok(Self::StructMap),
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
            MapType::StructMap => {
                quote!()
            }
        };

        token.to_tokens(tokens);
    }
}

impl MapAttr {
    pub(crate) fn enum_name(&self, enum_name: Ident) -> Ident {
        self.name
            .as_ref()
            .map(|name| format_ident!("{}", name))
            .unwrap_or_else(|| enum_name)
    }

    pub(crate) fn map_impl_mod(&self) -> MapType {
        if let Some(name) = &self.map {
            MapType::try_from(name).unwrap()
        } else {
            MapType::default()
        }
    }
}