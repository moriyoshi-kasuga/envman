use crate::{check_duplicate, require_lit_str, EnvManStructArgs};
use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{punctuated::Punctuated, spanned::Spanned, Expr, Meta, Token, Type};

pub(crate) struct EnvManFieldArgs {
    pub name: String,
    pub parser: Option<TokenStream>,
    pub default: Option<TokenStream>,
    pub test: Option<TokenStream>,
    pub alltime_parse: bool,
    pub is_option: bool,
    pub nest: bool,
}

/// Find the value of a #[envman(name = "...")] attribute.
pub(crate) fn attr(
    field: &syn::Field,
    struct_arg: &EnvManStructArgs,
) -> syn::Result<EnvManFieldArgs> {
    let mut rename: Option<String> = None;
    let mut parser: Option<TokenStream> = None;
    let mut default: Option<TokenStream> = None;
    let mut test: Option<TokenStream> = None;
    let mut alltime_parse = false;
    let mut nest = false;

    for attr in &field.attrs {
        if !attr.path().is_ident("envman") {
            continue;
        }

        let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

        for meta in nested {
            match meta {
                Meta::Path(ref path) if path.is_ident("alltime_parse") => {
                    check_duplicate!(path.span(), alltime_parse, alltime_parse);
                    alltime_parse = true;
                }
                Meta::NameValue(meta) if meta.path.is_ident("rename") => {
                    check_duplicate!(meta.span(), rename);

                    let string = require_lit_str(&meta, &meta.value)?;

                    rename = Some(string);
                }
                Meta::Path(ref path) if path.is_ident("default") => {
                    check_duplicate!(meta.span(), default);

                    default = Some(quote::quote!(Default::default()))
                }
                Meta::NameValue(meta) if meta.path.is_ident("default") => {
                    check_duplicate!(meta.span(), default);

                    default = Some(meta.value.into_token_stream())
                }
                Meta::Path(ref path) if path.is_ident("test") => {
                    check_duplicate!(meta.span(), test);

                    test = Some(quote::quote!(Default::default()))
                }
                Meta::NameValue(meta) if meta.path.is_ident("test") => {
                    check_duplicate!(meta.span(), test);

                    test = Some(meta.value.into_token_stream())
                }
                Meta::NameValue(meta) if meta.path.is_ident("parser") => {
                    check_duplicate!(meta.span(), parser);

                    if let Expr::Path(path) = &meta.value {
                        parser = Some(path.to_token_stream());
                        continue;
                    }

                    return Err(syn::Error::new_spanned(meta, "expected path"));
                }
                Meta::Path(ref path) if path.is_ident("nest") => {
                    check_duplicate!(path.span(), nest, nest);
                    nest = true;
                }
                _ => return Err(syn::Error::new_spanned(meta, "unexpected attribute")),
            }
        }
    }
    let name = match rename {
        Some(x) => x,
        None => {
            let mut name = unraw(
                field
                    .ident
                    .as_ref()
                    .ok_or_else(|| syn::Error::new_spanned(field, "field must have a name"))?,
            );
            name = name.to_case(struct_arg.rename_all);
            if let Some(prefix) = &struct_arg.prefix {
                name = format!("{prefix}{name}");
            };
            if let Some(suffix) = &struct_arg.suffix {
                name = format!("{name}{suffix}");
            };
            name
        }
    };
    Ok(EnvManFieldArgs {
        name,
        default,
        test,
        is_option: is_option(&field.ty),
        parser,
        alltime_parse,
        nest,
    })
}

fn is_option(ty: &Type) -> bool {
    match get_last_path_segment(ty) {
        Some(seg) => seg.ident == "Option",
        _ => false,
    }
}

fn get_last_path_segment(ty: &Type) -> Option<&syn::PathSegment> {
    match ty {
        Type::Path(path) => path.path.segments.last(),
        _ => None,
    }
}

fn unraw(ident: &proc_macro2::Ident) -> String {
    ident.to_string().trim_start_matches("r#").to_owned()
}
