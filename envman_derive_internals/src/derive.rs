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
        separator,
        validate,
        secret: _,
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

    // Handle separator (for Vec/array types)
    let parse_with_separator = if let Some(sep) = separator {
        quote! {
            {
                let parts: Vec<&str> = val.split(#sep).collect();
                let mut results = Vec::new();
                for part in parts {
                    let parsed = #parser(part.trim()).map_err(|err| envman::EnvManError::Parse {
                        key: #name,
                        value: part.to_string(),
                        expected_type: std::any::type_name::<Self>(),
                        source: Box::new(err)
                    })?;
                    results.push(parsed);
                }
                results
            }
        }
    } else {
        quote! {
            #parser(&val).map_err(|err| envman::EnvManError::Parse {
                key: #name,
                value: val.clone(),
                expected_type: std::any::type_name::<Self>(),
                source: Box::new(err)
            })?
        }
    };

    // Handle validation
    let validation_code = if let Some(validator) = validate {
        quote! {
            match #validator(&parsed_value) {
                Ok(_) => {},
                Err(e) => {
                    return Err(envman::EnvManError::Validation {
                        key: #name,
                        value: val.clone(),
                        message: format!("{}", e),
                    });
                }
            }
        }
    } else {
        quote! {}
    };

    let ok = if is_option {
        quote! {
            Some({
                let parsed_value = #parse_with_separator;
                #validation_code
                parsed_value
            })
        }
    } else {
        quote! {
            {
                let parsed_value = #parse_with_separator;
                #validation_code
                parsed_value
            }
        }
    };

    let default_value = match default {
        Some(ref default_expr) => {
            let parsed_default = if alltime_parse {
                quote! {
                    {
                        let val = #default_expr.to_string();
                        let parsed_value = #parse_with_separator;
                        #validation_code
                        parsed_value
                    }
                }
            } else {
                quote! { #default_expr }
            };

            if is_option {
                quote! { Some(#parsed_default) }
            } else {
                parsed_default
            }
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
            Err(_) => #default_value,
        }
    };

    match test {
        Some(ref test_expr) => {
            let parsed_test = if alltime_parse {
                quote! {
                    {
                        let val = #test_expr.to_string();
                        let parsed_value = #parse_with_separator;
                        #validation_code
                        parsed_value
                    }
                }
            } else {
                quote! { #test_expr }
            };

            let test_value = if is_option {
                quote! { Some(#parsed_test) }
            } else {
                parsed_test
            };

            Ok(quote! {
                if cfg!(test) {
                    #test_value
                } else {
                    #token
                }
            })
        }
        None => Ok(token),
    }
}
