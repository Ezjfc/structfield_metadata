//! #[metadata] attribute for the struct level.
//! This attribute defines the names, fields type, aliases for metadata structs.

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::{Parse, ParseStream}, token::{Colon, Eq, Struct}, Attribute, Ident, Token, Type, Visibility};

use crate::attr::ArgsList;

pub fn proc_attribute(attr: &Attribute) -> Result<Vec<MetadataStruct>, syn::Error> {
    let parser = <ArgsList<MetadataStruct>>::parse_terminated;
    Ok(attr.parse_args_with(parser)?.into_iter().collect())
}

pub fn proc_all_attributes(attrs: &[&Attribute]) -> Vec<Result<MetadataStruct, syn::Error>> {
    let structs: Vec<_> = attrs.into_iter()
        .map(|attr| proc_attribute(*attr))
        .flatten_ok()
        .collect();
    structs
}

#[derive(Debug)]
pub struct MetadataStruct {
    pub prefix: StructLevelAttributeArgPrefix,
    pub name: Ident,
    pub metadata_type: Option<MetadataType>,
    pub alias: Option<MetadataAlias>,
}

#[derive(Debug)]
pub enum StructLevelAttributeArgPrefix {
    NoPrefix,
    StructKeyword(Struct),
    Visibility(Visibility, Struct),
}

#[derive(Debug)]
pub struct MetadataType {
    pub colon: Colon,
    pub metadata_type: Type,
}

#[derive(Debug)]
pub struct MetadataAlias {
    pub eq: Eq,
    pub alias: Ident,
}

impl Parse for MetadataStruct {
    fn parse(input: ParseStream<'_>) -> Result<Self, syn::Error> {
        Ok(Self {
            prefix: {
                use StructLevelAttributeArgPrefix as Prefix;
                if input.peek(Token![pub]) {
                    Prefix::Visibility(input.parse()?, input.parse()?)
                } else if input.peek(Token![struct]) {
                    Prefix::StructKeyword(input.parse()?)
                } else {
                    Prefix::NoPrefix
                }
            },
            name: input.parse()?,
            metadata_type: {
                if input.peek(Token![:]) {
                    Some(input.parse()?)
                } else {
                    None
                }
            },
            alias: {
                if input.peek(Token![=]) {
                    Some(input.parse()?)
                } else {
                    None
                }
            },
        })
    }
}

impl ToTokens for MetadataStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use StructLevelAttributeArgPrefix as Prefix;
        match &self.prefix {
            Prefix::NoPrefix => (),
            Prefix::StructKeyword(keyword) => keyword.to_tokens(tokens),
            Prefix::Visibility(visibility, keyword) => {
                visibility.to_tokens(tokens);
                keyword.to_tokens(tokens);
            },
        }

        self.name.to_tokens(tokens);
        self.metadata_type.to_tokens(tokens);
        self.alias.to_tokens(tokens);
    }
}

impl Parse for MetadataType {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            colon: input.parse()?,
            metadata_type: input.parse()?,
        })
    }
}

impl ToTokens for MetadataType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.colon.to_tokens(tokens);
        self.metadata_type.to_tokens(tokens);
    }
}

impl Parse for MetadataAlias {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            eq: input.parse()?,
            alias: input.parse()?,
        })
    }
}

impl ToTokens for MetadataAlias {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.eq.to_tokens(tokens);
        self.alias.to_tokens(tokens);
    }
}
