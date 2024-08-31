use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Fields, FieldsNamed};

mod attr;
mod derive;

pub struct EnvManArgs {
    name: String,
    default: Option<String>,
    test: Option<String>,
    is_option: bool,
}

pub fn derive_envman(input: DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => derive_envman_internal(&input, fields),
        _ => Err(Error::new(
            Span::call_site(),
            "currently only structs with named fields are supported",
        )),
    }
}

pub fn derive_envman_internal(
    input: &DeriveInput,
    fields: &FieldsNamed,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let field_name = fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();

    let body = fields
        .named
        .iter()
        .map(attr::attr)
        .collect::<syn::Result<Vec<_>>>()?
        .into_iter()
        .map(derive::derive)
        .collect::<Vec<_>>();

    let expr = quote! {
        impl #impl_generics envman::EnvMan for #ident #ty_generics #where_clause {
            fn load() -> Result<Self, envman::EnvManError> {
                Ok(Self { #( #field_name: #body, )* })
            }
        }
    };
    Ok(expr)
}
