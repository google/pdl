use proc_macro2::Ident;
use quote::format_ident;

pub fn get_integer_type(width: usize) -> Ident {
    let best_width = [8, 16, 32, 64]
        .into_iter()
        .filter(|x| *x >= width)
        .min()
        .unwrap_or_else(|| panic!("width {width} is too large"));
    format_ident!("u{best_width}")
}
