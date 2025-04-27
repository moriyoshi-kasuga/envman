use convert_case::Case;
use proc_macro2::TokenStream;

mod attr;
mod derive;
mod struct_attr;

struct EnvManStructArgs {
    pub rename_all: Case<'static>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

pub fn derive_envman(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => derive_envman_internal(&input, fields),
        _ => Err(syn::Error::new_spanned(
            input,
            "currently only structs with named fields are supported",
        )),
    }
}

fn derive_envman_internal(
    input: &syn::DeriveInput,
    fields: &syn::FieldsNamed,
) -> syn::Result<TokenStream> {
    let attr_arg = struct_attr::struct_attr(input)?;

    let field_name = fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();

    let body = fields
        .named
        .iter()
        .map(|v| attr::attr(v, &attr_arg))
        .collect::<syn::Result<Vec<_>>>()?
        .into_iter()
        .map(derive::derive)
        .collect::<syn::Result<Vec<_>>>()?;

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expr = quote::quote! {
        impl #impl_generics envman::EnvMan for #ident #ty_generics #where_clause {
            fn load() -> Result<Self, envman::EnvManError> {
                Ok(Self { #( #field_name: #body, )* })
            }
        }
    };
    Ok(expr)
}

macro_rules! check_duplicate {
    ($span:expr, $variant:ident) => {
        check_duplicate!(@__message $span, $variant, $variant.is_some(),);
    };
    ($span:expr, $variant:ident, $additional:literal) => {
        check_duplicate!(@__message $span, $variant, $variant.is_some(), $additional);
    };
    ($span:expr, $variant:ident, $expr:expr) => {
        check_duplicate!(@__message $span, $variant, $expr,);
    };
    (@__message $span:expr, $variant:ident, $expr:expr, $($additional:expr)?) => {
        check_duplicate!(@__final $span, $expr, concat!("duplicate `", stringify!($variant), "` attribute.", $(" ", $additional)?));
    };
    (@__final $span:expr, $expr:expr, $message:expr) => {
        if $expr {
            return Err(syn::Error::new($span, $message));
        }
    };
}

pub(crate) use check_duplicate;
use syn::spanned::Spanned;

fn require_lit_str<S: Spanned>(span: &S, expr: &syn::Expr) -> syn::Result<String> {
    if let syn::Expr::Lit(expr_lit) = &expr {
        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
            return Ok(lit_str.value());
        }
    }

    Err(syn::Error::new(span.span(), "expected string literal"))
}
