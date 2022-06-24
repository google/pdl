use std::fs;
use std::process::Command;

// The integration test in this file is not part of the pdl crate, and
// so we cannot directly depend on anything from pdl. However, we can
// include the test_utils.rs file directly.

#[path = "../src/test_utils.rs"]
mod test_utils;
use test_utils::{assert_eq_with_diff, find_binary, rustfmt};

fn strip_blank_lines(text: &str) -> String {
    text.lines().filter(|line| !line.trim().is_empty()).collect::<Vec<_>>().join("\n")
}

/// Run `code` through `pdl`.
///
/// # Panics
///
/// Panics if `pdl` cannot be found or if it fails.
fn pdl(code: &str) -> String {
    let tempdir = tempfile::tempdir().unwrap();
    let input = tempdir.path().join("input.pdl");
    fs::write(&input, code.as_bytes()).unwrap();

    // Cargo will set `CARGO_BIN_EXE_pdl` when compiling the crate. If
    // we're not using Cargo, we search for `pdl` using find_binary.
    let pdl_path = match std::option_env!("CARGO_BIN_EXE_pdl") {
        Some(pdl_path) => std::path::PathBuf::from(pdl_path),
        None => find_binary("pdl").unwrap(),
    };
    let output = Command::new(&pdl_path)
        .arg("--output-format")
        .arg("rust")
        .arg(input)
        .output()
        .expect("pdl failed");
    assert!(output.status.success(), "pdl failure: {:?}, input:\n{}", output, code);
    String::from_utf8(output.stdout).unwrap()
}

/// Run `code` through `bluetooth_packetgen`.
///
/// # Panics
///
/// Panics if `bluetooth_packetgen` cannot be found on `$PATH` or if
/// it returns a non-zero exit code.
fn bluetooth_packetgen(code: &str) -> String {
    let tempdir = tempfile::tempdir().unwrap();
    let tempdir_path = tempdir.path().to_str().unwrap();
    let input_path = tempdir.path().join("input.pdl");
    let output_path = input_path.with_extension("rs");
    fs::write(&input_path, code.as_bytes()).unwrap();
    let bluetooth_packetgen_path = find_binary("bluetooth_packetgen").unwrap();
    let output = Command::new(&bluetooth_packetgen_path)
        .arg(&format!("--include={}", tempdir_path))
        .arg(&format!("--out={}", tempdir_path))
        .arg("--rust")
        .arg(input_path)
        .output()
        .expect("bluetooth_packetgen failed");
    assert!(output.status.success(), "bluetooth_packetgen failure: {:?}, input:\n{}", output, code);
    fs::read_to_string(output_path).unwrap()
}

#[track_caller]
fn assert_equal_compilation(pdl_code: &str) {
    let old_rust = rustfmt(&bluetooth_packetgen(pdl_code));
    let new_rust = rustfmt(&pdl(pdl_code));
    assert_eq_with_diff(&strip_blank_lines(&old_rust), &strip_blank_lines(&new_rust));
}

#[test]
fn test_prelude() {
    let pdl_code = r#"
        little_endian_packets
    "#;
    assert_equal_compilation(pdl_code);
}

#[test]
fn test_empty_packet() {
    let pdl_code = r#"
        little_endian_packets

        packet Foo {
        }
    "#;
    assert_equal_compilation(pdl_code);
}

#[test]
fn test_simple_le_packet() {
    let pdl_code = r#"
        little_endian_packets

        packet Foo {
          a: 8,
          b: 16,
        }
    "#;
    assert_equal_compilation(pdl_code);
}
