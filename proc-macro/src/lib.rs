mod attr;

use itertools::Itertools;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, Data, DeriveInput, Error, Result, Visibility};

use crate::{attr::struct_level::{self, MetadataStruct}, struct_level::StructLevelAttributeArgPrefix};

/// ```
/// use structfield_metadata_proc_macro::Metadata;
///
/// #[derive(Metadata)]
/// #[metadata(pub(crate) struct a = Thing)]
/// struct YourStruct {
///     #[metadata(c = "d")]
///     field_a: bool,
///     field_b: usize,
/// }
/// ```
#[proc_macro_derive(Metadata, attributes(metadata))]
pub fn metadata_derive(item: TokenStream) -> TokenStream {
    proc(item).unwrap_or_else(Error::into_compile_error).into()
}

fn proc(item: TokenStream) -> Result<proc_macro2::TokenStream> {
    let item = syn::parse::<DeriveInput>(item)?;
    let attrs: Vec<_> = item.attrs.iter().filter(filter_attr).collect();
    let parsed_attrs: Vec<_> = struct_level::proc_all_attributes(&attrs)
        .into_iter()
        .try_collect()?;
    let output = wrap_in_macro(&item, &parsed_attrs);

    Ok(output)
}

fn filter_attr(attr: &&Attribute) -> bool {
    attr.path().is_ident("metadata")
}

/// Since the procedural macro is only responsible for implementing [core::Default] to the
/// generated metadata structs, this function wraps the main struct with [crate::metadata],
/// who handles the generation of metadata structs.
fn wrap_in_macro(main_struct: &DeriveInput, metadata_structs: &[MetadataStruct]) -> proc_macro2::TokenStream {
    let struct_decs: Vec<_> = metadata_structs.iter().map(|metadata_struct| {
        let prefix = if let StructLevelAttributeArgPrefix::NoPrefix = metadata_struct.prefix {
            "struct"
        } else {
            ""
        };
        let struct_dec = metadata_struct.to_token_stream();

        quote! { #prefix #struct_dec }
    }).collect();

    let vis = main_struct.vis;
    // main_struct.attrs
    let output = quote! {
        use crate::metadata;
        metadata!(
            {
                #main_struct
            },
            #(#struct_decs),*
        );
    };
    output
}
