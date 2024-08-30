use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Fields, FieldsNamed};

mod attr;

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

    let field_attrs = fields
        .named
        .iter()
        .map(attr::attr)
        .collect::<syn::Result<Vec<_>>>()?;

    let mut body = vec![];

    for (rename, opt, is_opt) in field_attrs {
        let opt = match opt {
            Some(text) => quote! { Some(#text) },
            None => quote! { None::<String> },
        };
        let q = if is_opt {
            quote! { match match std::env::var(#rename) {
                Ok(v) => Some(v),
                Err(_) => #opt.map(String::from),
            } {
                Some(v) => Some(v.parse().map_err(|_| envman::EnvManError::Parse { key: #rename.to_string() })?),
                None => None,
            } }
        } else {
            quote! {
                match std::env::var(#rename) {
                    Ok(v) => Ok(v),
                    Err(e) => match #opt {
                        Some(v) => Ok(String::from(v)),
                        None => Err(envman::EnvManError::NotFound(e)),
                    }
                }?.parse().map_err(|_| envman::EnvManError::Parse { key: #rename.to_string() })?
            }
        };
        body.push(q);
    }

    let expr = quote! {
        impl #impl_generics envman::EnvMan for #ident #ty_generics #where_clause {
            fn load() -> Result<Self, envman::EnvManError> {
                Ok(Self { #( #field_name: #body, )* })
            }
        }
    };
    Ok(expr)
}
