use std::str::FromStr;

use crate::{check_duplicate, require_lit_str, EnvManStructArgs};

use ident_case::RenameRule;
use syn::{punctuated::Punctuated, spanned::Spanned, Meta, Token};

/// Find the value of a #[envman(rename_all = "...")] attribute.
pub(crate) fn struct_attr(derive: &syn::DeriveInput) -> syn::Result<EnvManStructArgs> {
    let mut rename_all: Option<RenameRule> = None;
    let mut prefix: Option<String> = None;
    let mut suffix: Option<String> = None;

    for attr in &derive.attrs {
        if !attr.path().is_ident("envman") {
            continue;
        }

        let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

        for meta in nested {
            match meta {
                Meta::NameValue(meta) if meta.path.is_ident("rename_all") => {
                    check_duplicate!(meta.span(), rename_all);

                    let string = require_lit_str(&meta, &meta.value)?;

                    rename_all = Some(
                        RenameRule::from_str(&string)
                            .map_err(|_| syn::Error::new(meta.span(), "invalid RenameRule"))?,
                    );
                }
                Meta::NameValue(meta) if meta.path.is_ident("prefix") => {
                    check_duplicate!(meta.span(), prefix);

                    let string = require_lit_str(&meta, &meta.value)?;

                    prefix = Some(string);
                }
                Meta::NameValue(meta) if meta.path.is_ident("suffix") => {
                    check_duplicate!(meta.span(), suffix);

                    let string = require_lit_str(&meta, &meta.value)?;

                    suffix = Some(string);
                }
                _ => return Err(syn::Error::new_spanned(meta, "unexpected attribute")),
            }
        }
    }
    Ok(EnvManStructArgs {
        rename_all: rename_all.unwrap_or(RenameRule::ScreamingSnakeCase),
        prefix,
        suffix,
    })
}
