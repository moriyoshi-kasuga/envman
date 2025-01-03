use proc_macro2::TokenStream;

mod attr;
mod derive;

struct EnvManArgs {
    pub name: String,
    pub parser: Option<TokenStream>,
    pub default: Option<TokenStream>,
    pub test: Option<TokenStream>,
    pub alltime_parse: bool,
    pub is_option: bool,
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

    let expr = quote::quote! {
        impl #impl_generics envman::EnvMan for #ident #ty_generics #where_clause {
            fn load() -> Result<Self, envman::EnvManError> {
                Ok(Self { #( #field_name: #body, )* })
            }
        }
    };
    Ok(expr)
}
