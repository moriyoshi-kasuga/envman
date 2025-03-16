use crate::{check_duplicate, require_lit_str, EnvManStructArgs};

use convert_case::Case;
use syn::{punctuated::Punctuated, spanned::Spanned, Meta, Token};

/// Find the value of a #[envman(rename_all = "...")] attribute.
pub(crate) fn struct_attr(derive: &syn::DeriveInput) -> syn::Result<EnvManStructArgs> {
    let mut rename_all: Option<Case> = None;
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
                        from_str_to_case(&string)
                            .ok_or_else(|| syn::Error::new(meta.span(), "invalid rename_all"))?,
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
        rename_all: rename_all.unwrap_or(Case::UpperSnake),
        prefix,
        suffix,
    })
}

pub(crate) fn from_str_to_case(text: &str) -> Option<Case> {
    let case = match text {
        "lowercase" => Case::Lower,
        "UPPERCASE" => Case::Upper,
        "PascalCase" => Case::Pascal,
        "camelCase" => Case::Camel,
        "snake_case" => Case::Snake,
        "SCREAMING_SNAKE_CASE" => Case::UpperSnake,
        "kebab-case" => Case::Kebab,
        "SCREAMING-KEBAB-CASE" => Case::UpperKebab,
        _ => return None,
    };

    Some(case)
}
