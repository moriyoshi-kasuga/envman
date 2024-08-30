use proc_macro2::Ident;
use syn::{Field, LitStr, PathSegment, Result, Type};

/// Find the value of a #[envman(name = "...")] attribute.
pub fn attr(field: &Field) -> Result<(String, Option<String>, bool)> {
    let mut rename = None;
    let mut default = None;

    for attr in &field.attrs {
        if !attr.path().is_ident("envman") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let s: LitStr = meta.value()?.parse()?;
                if rename.is_some() {
                    return Err(meta.error(String::from("duplicate rename attribute")));
                }
                rename = Some(s.value());
                return Ok(());
            }
            if meta.path.is_ident("default") {
                let s: LitStr = meta.value()?.parse()?;
                if default.is_some() {
                    return Err(meta.error(String::from("duplicate default attribute")));
                }
                default = Some(s.value());
                return Ok(());
            }
            Err(meta.error("unsupported attribute"))
        })?;
    }
    Ok((
        rename.unwrap_or_else(|| unraw(field.ident.as_ref().unwrap())),
        default,
        is_option(&field.ty),
    ))
}

fn is_option(ty: &Type) -> bool {
    match get_last_path_segment(ty) {
        Some(seg) => seg.ident == "Option",
        _ => false,
    }
}

fn get_last_path_segment(ty: &Type) -> Option<&PathSegment> {
    match ty {
        Type::Path(path) => path.path.segments.last(),
        _ => None,
    }
}

fn unraw(ident: &Ident) -> String {
    ident
        .to_string()
        .trim_start_matches("r#")
        .to_owned()
        .to_uppercase()
}
