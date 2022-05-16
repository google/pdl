//! Various utility functions used in tests.

// This file is included directly into integration tests in the
// `test/` directory. These tests are compiled without access to the
// rest of the `pdl` crate. To make this work, avoid `use crate::`
// statements below.

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

/// Search for a binary in `$PATH` or as a sibling to the current
/// executable (typically the test binary).
pub fn find_binary(name: &str) -> Result<std::path::PathBuf, String> {
    let mut current_exe = std::env::current_exe().unwrap();
    current_exe.pop();
    let paths = std::env::var_os("PATH").unwrap();
    for mut path in std::iter::once(current_exe.clone()).chain(std::env::split_paths(&paths)) {
        path.push(name);
        if path.exists() {
            return Ok(path);
        }
    }

    Err(format!(
        "could not find '{}' in the directory of the binary ({}) or in $PATH ({})",
        name,
        current_exe.to_string_lossy(),
        paths.to_string_lossy(),
    ))
}

/// Run `input` through `rustfmt`.
///
/// # Panics
///
/// Panics if `rustfmt` cannot be found in the same directory as the
/// test executable or if it returns a non-zero exit code.
pub fn rustfmt(input: &str) -> String {
    let rustfmt_path = find_binary("rustfmt").expect("cannot find rustfmt");
    let mut rustfmt = Command::new(&rustfmt_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("failed to start {:?}", &rustfmt_path));

    let mut stdin = rustfmt.stdin.take().unwrap();
    // Owned copy which we can move into the writing thread.
    let input = String::from(input);
    std::thread::spawn(move || {
        stdin.write_all(input.as_bytes()).expect("could not write to stdin");
    });

    let output = rustfmt.wait_with_output().expect("error executing rustfmt");
    assert!(output.status.success(), "rustfmt failed: {}", output.status);
    String::from_utf8(output.stdout).expect("rustfmt output was not UTF-8")
}

/// Find the unified diff between two strings using `diff`.
///
/// # Panics
///
/// Panics if `diff` cannot be found on `$PATH` or if it returns an
/// error.
pub fn diff(left: &str, right: &str) -> String {
    let mut temp_left = NamedTempFile::new().unwrap();
    temp_left.write_all(left.as_bytes()).unwrap();
    let mut temp_right = NamedTempFile::new().unwrap();
    temp_right.write_all(right.as_bytes()).unwrap();

    // We expect `diff` to be available on PATH.
    let output = Command::new("diff")
        .arg("--unified")
        .arg("--color=always")
        .arg("--label")
        .arg("left")
        .arg("--label")
        .arg("right")
        .arg(temp_left.path())
        .arg(temp_right.path())
        .output()
        .expect("failed to run diff");
    let diff_trouble_exit_code = 2; // from diff(1)
    assert_ne!(
        output.status.code().unwrap(),
        diff_trouble_exit_code,
        "diff failed: {}",
        output.status
    );
    String::from_utf8(output.stdout).expect("diff output was not UTF-8")
}

/// Compare two strings and output a diff if they are not equal.
#[track_caller]
pub fn assert_eq_with_diff(left: &str, right: &str) {
    assert!(left == right, "texts did not match, diff:\n{}\n", diff(left, right));
}

/// Compare a string with a snapshot file.
///
/// The `snapshot_path` is relative to the current working directory
/// of the test binary. This depends on how you execute the tests:
///
/// * When using `atest`: The current working directory is a random
///   temporary directory. You need to ensure that the snapshot file
///   is installed into this directory. You do this by adding the
///   snapshot to the `data` attribute of your test rule
///
/// * When using Cargo: The current working directory is set to
///   `CARGO_MANIFEST_DIR`, which is where the `Cargo.toml` file is
///   found.
///
/// If you run the test with Cargo and the `UPDATE_SNAPSHOTS`
/// environment variable is set, then the `actual_content` will be
/// written to `snapshot_path`. Otherwise the content is compared and
/// a panic is triggered if they differ.
#[track_caller]
pub fn assert_snapshot_eq<P: AsRef<Path>>(snapshot_path: P, actual_content: &str) {
    let snapshot = snapshot_path.as_ref();
    let snapshot_content = fs::read(&snapshot).unwrap_or_else(|err| {
        panic!("Could not read snapshot from {}: {}", snapshot.display(), err)
    });
    let snapshot_content = String::from_utf8(snapshot_content).expect("Snapshot was not UTF-8");

    // Normal comparison if UPDATE_SNAPSHOTS is unset.
    if std::env::var("UPDATE_SNAPSHOTS").is_err() {
        return assert_eq_with_diff(&snapshot_content, actual_content);
    }

    // Bail out if we are not using Cargo.
    if std::env::var("CARGO_MANIFEST_DIR").is_err() {
        panic!("Please unset UPDATE_SNAPSHOTS if you are not using Cargo");
    }

    if actual_content != snapshot_content {
        eprintln!(
            "Updating snapshot {}: {} -> {} bytes",
            snapshot.display(),
            snapshot_content.len(),
            actual_content.len()
        );
        fs::write(&snapshot_path, actual_content).unwrap_or_else(|err| {
            panic!("Could not write snapshot to {}: {}", snapshot.display(), err)
        });
    }
}
