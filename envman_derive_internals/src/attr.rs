use proc_macro2::Ident;
use syn::{Attribute, Field, LitStr, Result};

/// Find the value of a #[envman(rename = "...")] attribute.
fn attr_rename(attrs: &[Attribute]) -> Result<Option<String>> {
    let mut rename = None;

    for attr in attrs {
        if !attr.path().is_ident("envman") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let s: LitStr = meta.value()?.parse()?;
                if rename.is_some() {
                    return Err(meta.error("duplicate rename attribute"));
                }
                rename = Some(s.value());
                Ok(())
            } else {
                Err(meta.error("unsupported attribute"))
            }
        })?;
    }

    Ok(rename)
}

/// Determine the name of a field, respecting a rename attribute.
pub fn name_of_field(field: &Field) -> Result<String> {
    let rename = attr_rename(&field.attrs)?;
    Ok(rename.unwrap_or_else(|| unraw(field.ident.as_ref().unwrap())))
}

fn unraw(ident: &Ident) -> String {
    ident
        .to_string()
        .trim_start_matches("r#")
        .to_owned()
        .to_uppercase()
}