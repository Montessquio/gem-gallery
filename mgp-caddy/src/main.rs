//! A media caddy module that serves static files from a filesystem over HTTP.

#![deny(
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements ,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    unused_variables,
)]
#![allow(dead_code)]

pub mod fs;
pub mod convert;

static SAMPLE_FILE : &[u8] = include_bytes!(r#"sample.mp4"#);

fn main() -> eyre::Result<()> {
    let input = std::io::BufReader::new(SAMPLE_FILE);
    println!("{:?}", convert::convert_to_webm(input));

    Ok(())
}