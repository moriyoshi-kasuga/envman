use quote::quote;

use crate::attr::EnvManFieldArgs;

pub(crate) fn derive(args: EnvManFieldArgs) -> proc_macro2::TokenStream {
    let EnvManFieldArgs {
        name,
        parser,
        default,
        test,
        alltime_parse,
        is_option,
    } = args;

    let parser = match parser {
        Some(parser) => quote! { #parser },
        None => quote! { std::str::FromStr::from_str },
    };

    macro_rules! parse {
        ($token:ident) => {
            quote! { #parser(#$token).map_err(|err| envman::EnvManError::Parse { key: #name, source: Box::new(err) })? }
        };
    }

    macro_rules! alltime_parse {
        ($token:ident) => {
            if alltime_parse {
                parse!($token)
            } else {
                quote! { #$token }
            }
        };
    }

    macro_rules! wrap_opt {
        ($token:ident) => {
            if is_option {
                quote! { Some(#$token) }
            } else {
                quote! { #$token }
            }
        };
    }

    let val = quote! { &val };

    let ok = {
        let temp = parse!(val);
        let temp = wrap_opt!(temp);
        quote! { #temp }
    };

    let default = match default {
        Some(default) => {
            let token_parse = alltime_parse!(default);
            wrap_opt!(token_parse)
        }
        None => {
            if is_option {
                quote! { None }
            } else {
                quote! { return Err(envman::EnvManError::NotFound { key: #name }) }
            }
        }
    };

    let token = quote! {
        match std::env::var(#name) {
            Ok(val) => #ok,
            Err(_) => #default,
        }
    };

    match test {
        Some(test) => {
            let token_parse = alltime_parse!(test);
            let test = wrap_opt!(token_parse);
            quote! {
                if cfg!(test) {
                    #test
                } else {
                    #token
                }
            }
        }
        None => token,
    }
}
