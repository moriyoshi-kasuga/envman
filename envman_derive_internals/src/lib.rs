use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Fields, FieldsNamed, Result};

mod attr;

pub fn derive_envman(input: DeriveInput) -> Result<TokenStream> {
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

pub fn derive_envman_internal(input: &DeriveInput, fields: &FieldsNamed) -> Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fieldname = fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let fieldstr = fields
        .named
        .iter()
        .map(attr::name_of_field)
        .collect::<Result<Vec<_>>>()?;

    let expr = quote! {
        impl #impl_generics ::envman::EnvMan for #ident #ty_generics #where_clause {
            fn load() -> Result<Self, ::envman::EnvManError> {
                Ok(Self {
                    #(
                        #fieldname : std::env::var(#fieldstr).map_err(|e| ::envman::EnvManError::NotFound(e)).unwrap().parse()
                            .map_err(|e| ::envman::EnvManError::Parse { key: #fieldstr.to_string() }).unwrap(),
                    )*
                })
            }
        }
    };
    Ok(expr)
}
