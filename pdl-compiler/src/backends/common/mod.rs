pub mod alignment;
pub mod test;

use crate::ast;

// Determine if the enum is complete, i.e. all values in the backing
// integer range have a matching tag in the original declaration.
// The default tag, if present, does not count towards completion.
pub fn is_complete_enum(tags: &[ast::Tag], max: usize) -> bool {
    let mut ranges = tags
        .iter()
        .filter_map(|tag| match tag {
            ast::Tag::Value(tag) => Some((tag.value, tag.value)),
            ast::Tag::Range(tag) => Some(tag.range.clone().into_inner()),
            _ => None,
        })
        .collect::<Vec<_>>();
    ranges.sort_unstable();
    // An enum that declares no value or range tag cannot cover the backing
    // integer range, and `first`/`last` would be `None` below.
    !ranges.is_empty()
        && ranges.first().unwrap().0 == 0
        && ranges.last().unwrap().1 == max
        && ranges
            .windows(2)
            .all(|window| if let [left, right] = window { left.1 == right.0 - 1 } else { false })
}

// Determine if the enum is open, i.e. a default tag is defined.
pub fn is_open_enum(tags: &[ast::Tag]) -> bool {
    tags.iter().any(|tag| matches!(tag, ast::Tag::Other(_)))
}

// Determine if the enum is primitive, i.e. does not contain any tag range.
pub fn is_primitive_enum(tags: &[ast::Tag]) -> bool {
    tags.iter().all(|tag| matches!(tag, ast::Tag::Value(_)))
}
