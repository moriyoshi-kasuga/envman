use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{punctuated::Punctuated, Expr, Lit, Meta, Token, Type};

use crate::EnvManArgs;

/// Find the value of a #[envman(name = "...")] attribute.
pub fn attr(field: &syn::Field) -> syn::Result<EnvManArgs> {
    let mut rename: Option<String> = None;
    let mut parser: Option<TokenStream> = None;
    let mut default: Option<TokenStream> = None;
    let mut test: Option<TokenStream> = None;
    let mut skip = false;
    let mut alltime_parse = false;

    for attr in &field.attrs {
        if !attr.path().is_ident("envman") {
            continue;
        }

        let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

        for meta in nested {
            match meta {
                Meta::Path(ref path) if path.is_ident("skip") => {
                    if skip {
                        return Err(syn::Error::new_spanned(meta, "duplicate skip attribute"));
                    }
                    skip = true;
                }
                Meta::Path(ref path) if path.is_ident("alltime_parse") => {
                    if alltime_parse {
                        return Err(syn::Error::new_spanned(
                            meta,
                            "duplicate alltime_parse attribute",
                        ));
                    }
                    alltime_parse = true;
                }
                Meta::NameValue(meta) if meta.path.is_ident("rename") => {
                    if rename.is_some() {
                        return Err(syn::Error::new_spanned(meta, "duplicate rename attribute"));
                    }
                    if let Expr::Lit(expr) = &meta.value {
                        if let Lit::Str(lit_str) = &expr.lit {
                            rename = Some(lit_str.value());
                            continue;
                        }
                    }
                    return Err(syn::Error::new_spanned(meta, "expected string literal"));
                }
                Meta::Path(ref path) if path.is_ident("default") => {
                    if default.is_some() {
                        return Err(syn::Error::new_spanned(meta, "duplicate default attribute"));
                    }
                    default = Some(quote::quote!("Default::default()"))
                }
                Meta::NameValue(meta) if meta.path.is_ident("default") => {
                    if default.is_some() {
                        return Err(syn::Error::new_spanned(meta, "duplicate default attribute"));
                    }
                    default = Some(meta.value.into_token_stream())
                }
                Meta::NameValue(meta) if meta.path.is_ident("default_f") => {
                    if default.is_some() {
                        return Err(syn::Error::new_spanned(meta, "duplicate default attribute"));
                    }
                    if let Expr::Path(path) = &meta.value {
                        default = Some(quote::quote!(#path.path()))
                    }
                }
                Meta::Path(ref path) if path.is_ident("test") => {
                    if test.is_some() {
                        return Err(syn::Error::new_spanned(meta, "duplicate test attribute"));
                    }
                    test = Some(quote::quote!("Default::default()"))
                }
                Meta::NameValue(meta) if meta.path.is_ident("test") => {
                    if test.is_some() {
                        return Err(syn::Error::new_spanned(meta, "duplicate test attribute"));
                    }
                    test = Some(meta.value.into_token_stream())
                }
                Meta::NameValue(meta) if meta.path.is_ident("test_f") => {
                    if test.is_some() {
                        return Err(syn::Error::new_spanned(meta, "duplicate test attribute"));
                    }
                    if let Expr::Path(path) = &meta.value {
                        test = Some(quote::quote!(#path.path()))
                    }
                }
                Meta::NameValue(meta) if meta.path.is_ident("parser") => {
                    if parser.is_some() {
                        return Err(syn::Error::new_spanned(meta, "duplicate parser attribute"));
                    }
                    if let Expr::Path(path) = &meta.value {
                        parser = Some(path.to_token_stream())
                    }
                }
                _ => return Err(syn::Error::new_spanned(meta, "unexpected attribute")),
            }
        }
    }
    Ok(EnvManArgs {
        name: rename.unwrap_or_else(|| unraw(field.ident.as_ref().unwrap())),
        default,
        test,
        is_option: is_option(&field.ty),
        parser,
        skip,
        alltime_parse,
        ty: field.ty.to_token_stream(),
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
    ident
        .to_string()
        .trim_start_matches("r#")
        .to_owned()
        .to_uppercase()
}
