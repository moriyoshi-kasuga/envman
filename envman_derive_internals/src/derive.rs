use crate::EnvManArgs;
use quote::quote;

pub fn derive(args: EnvManArgs) -> proc_macro2::TokenStream {
    let EnvManArgs {
        name,
        test,
        is_option,
        default,
    } = args;

    let opt = match default {
        Some(text) => quote! { Some(#text) },
        None => quote! { None::<String> },
    };

    let test = match test {
        Some(test) => quote! { Some(#test) },
        None => quote! { None::<String> },
    };

    let variable = quote! {
        match #test {
            Some(v) if cfg!(test) => Ok(v.to_string()),
            _ => std::env::var(#name),
        }
    };

    if is_option {
        quote! { match match #variable {
            Ok(v) => Some(v),
            Err(_) => #opt.map(String::from),
        } {
            Some(v) => Some(v.parse().map_err(|_| envman::EnvManError::Parse { key: #name.to_string() })?),
            None => None,
        } }
    } else {
        quote! {
            match #variable {
                Ok(v) => Ok(v),
                Err(e) => match #opt {
                    Some(v) => Ok(String::from(v)),
                    None => Err(envman::EnvManError::NotFound { key: #name.to_string() }),
                }
            }?.parse().map_err(|_| envman::EnvManError::Parse { key: #name.to_string() })?
        }
    }
}
