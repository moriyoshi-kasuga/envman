use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EnvMan, attributes(envman))]
pub fn derive_envman(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    envman_derive_internals::derive_envman(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
