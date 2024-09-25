mod attr;
mod derive;

struct EnvManArgs {
    name: String,
    default: Option<String>,
    test: Option<String>,
    is_option: bool,
}

pub fn derive_envman(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
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
) -> syn::Result<proc_macro2::TokenStream> {
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
