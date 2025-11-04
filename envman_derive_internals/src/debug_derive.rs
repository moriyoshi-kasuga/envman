use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::attr;

pub(crate) fn derive_debug(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "EnvManDebug only supports structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "EnvManDebug only supports structs",
            ))
        }
    };

    let struct_arg = crate::struct_attr::struct_attr(input)?;

    let field_formatters: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_args = attr::attr(field, &struct_arg)?;
            let field_name = &field.ident;
            let field_name_str = field_name
                .as_ref()
                .map(|i| i.to_string())
                .unwrap_or_default();

            if field_args.secret {
                Ok(quote! {
                    .field(#field_name_str, &"***")
                })
            } else {
                Ok(quote! {
                    .field(#field_name_str, &self.#field_name)
                })
            }
        })
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! {
        impl #impl_generics std::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#name))
                    #(#field_formatters)*
                    .finish()
            }
        }
    })
}
