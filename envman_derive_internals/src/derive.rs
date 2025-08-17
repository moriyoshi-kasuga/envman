use quote::quote;

use crate::attr::EnvManFieldArgs;

pub(crate) fn derive(args: EnvManFieldArgs) -> syn::Result<proc_macro2::TokenStream> {
    let EnvManFieldArgs {
        name,
        parser,
        default,
        test,
        alltime_parse,
        is_option,
        nest,
    } = args;

    if nest {
        if parser.is_some() {
            return Err(syn::Error::new_spanned(
                parser,
                "`parser` is not allowed when `nest` is true",
            ));
        }

        let load_from_env = quote! {
            envman::EnvMan::load_from_env()
        };

        let token = if is_option {
            if default.is_some() {
                quote! {
                    #load_from_env.ok().or_else(|| Some(#default))
                }
            } else {
                quote! {
                    #load_from_env.ok()
                }
            }
        } else if default.is_some() {
            quote! {
                #load_from_env.unwrap_or_else(|_| #default)
            }
        } else {
            quote! {
                #load_from_env?
            }
        };

        return match test {
            Some(test) => Ok(quote! {
                if cfg!(test) {
                    #test
                } else {
                    #token
                }
            }),
            None => Ok(token),
        };
    }

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
            Ok(quote! {
                if cfg!(test) {
                    #test
                } else {
                    #token
                }
            })
        }
        None => Ok(token),
    }
}
