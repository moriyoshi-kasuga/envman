# Field Attributes FlowChart

Group items are always selected once  
All of the following are inside `#[envman()]`.

## List (priority for parse)

1. rename : `rename = "new name"`
2. parser: `parser = "constants::default_parser"`
3. group_test
   > if under test, use this value
   - `test`
   - `test = "default value under test"`
   - `default_f = "constants::get_test_value"`
4. group_default

   > if not found in env, use this value  
   > If a test exists and is under test, use the test

   - `default`
   - `default = "default value"`
   - `default_f = "constants::get_default_value"`

5. skip : `skip`

   > not get from env, use default value  
   > required group_default

6. alltime_parse : `alltime_parse`
   > The normal default (and test) return value is the field type  
   > If this is set, the return value is a string and the parser is used.
   > optional

## Process Flow

This is just an overview, there are many issues, but don't say nothing

It doesn't mention details.  
Assume groups, etc. have already been checked.

```rust
struct FieldAttrs {
    // already applied rename value
    pub name: String,
    pub parser: Option<Path>,

    // pub test: bool,
    // pub test_value: Option<Expr>,
    // pub test_f: Option<Path>,
    // pub default: bool,
    // pub default_value: Option<Expr>,
    // pub default_f: Option<Path>,
    // All of the above apply.
    pub default: Option<TokenStream>,
    pub skip: bool,
    pub alltime_parse: bool,
    /// todo
    // pub is_option: bool,
}

fn parse(input:syn::DeriveInput, attrs: FieldAttrs) -> syn::Result<proc_macro2::TokenStream> {
    let parser = match attrs.parser {
        Some(path) => path.to_token_stream(),
        None => "FromStr::from_str".to_token_stream(),
    };
    if attrs.skip {
        return attrs.default.map(|token| {
            if attrs.alltime_parse {
                quote! {
                    #parser(#token)
                }
            } else {
                token
            }
        }).ok_or_else(|| syn::Error::new_spanned(input, "not found default value"));
    };
    quote! {
        match std::env::var(attrs.name) {
            Ok(v) => #parser(v)?,
            Err(e) => match #attrs.default {
                Some(v) if #attrs.alltime_parse => #parser(v)?,
                Some(v) => Ok(v)
                None => Err(e)
            }
        }
    }

}
```
