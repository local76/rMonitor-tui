//! F1..F7 doc filename lookup.

pub fn doc_for_f_key(n: u8) -> Option<&'static str> {
    match n {
        1 => Some("README.md"),
        2 => Some("SUPPORT.md"),
        3 => Some("LICENSE.md"),
        4 => Some("COPYRIGHT.md"),
        5 => Some("PRIVACY.md"),
        6 => Some("SECURITY.md"),
        7 => Some("CONTRIBUTING.md"),
        _ => None,
    }
}
